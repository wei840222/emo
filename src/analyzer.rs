use charabia::Segment;

use crate::dataset::EmojiDataset;
use serde::Serialize;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitMode {
    Timeline,
    Paragraph,
    Line,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmojiStat {
    pub emoji: String,
    pub name: String,
    pub count: usize,
    pub score: f64,
    pub intensity: f64,
    pub in_dataset: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmojiBurst {
    pub emoji: String,
    pub name: String,
    pub max_streak: usize,
    pub total_bursts: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmojiCombo {
    pub combo: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnicodeBlockStat {
    pub block_name: String,
    pub count: usize,
    pub percentage: f64,
    pub avg_score: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct SentimentSegment {
    pub label: String,
    pub score: f64,
    pub intensity: f64,
    pub emoji_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct SentimentProgression {
    pub segments: Vec<SentimentSegment>,
    pub trend_status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PositionBias {
    pub avg_position: f64,
    pub front_pct: f64,
    pub mid_pct: f64,
    pub end_pct: f64,
    pub bias_status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileReport {
    pub file_name: String,
    pub total_chars: usize,
    pub total_words: usize,
    pub total_emojis: usize,
    pub overall_score: f64,
    pub overall_intensity: f64,
    pub top_emoji: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmotionStat {
    pub emotion: String,
    pub count: usize,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmotionProfile {
    pub primary_emotion: String,
    pub top_emotions: Vec<EmotionStat>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SlangStat {
    pub term: String,
    pub count: usize,
    pub sentiment_score: f64,
    pub sarcasm_weight: f64,
    pub meaning: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SlangAnalysis {
    pub total_slang_count: usize,
    pub slang_density_per_100_words: f64,
    pub sarcasm_index: f64,
    pub sarcasm_status: String,
    pub elongation_count: usize,
    pub hybrid_score: f64,
    pub top_slang: Vec<SlangStat>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AnalysisResult {
    pub total_chars: usize,
    pub total_words: usize,
    pub total_emojis: usize,
    pub unique_emojis: usize,
    pub matched_emojis_count: usize,
    pub unmatched_emojis_count: usize,
    pub overall_score: f64,
    pub overall_intensity: f64,
    pub positive_count: usize,
    pub neutral_count: usize,
    pub negative_count: usize,
    pub density_per_1000_chars: f64,
    pub density_per_100_words: f64,
    pub style_level: String,
    pub entropy: f64,
    pub diversity_ratio: f64,
    pub polarization_index: f64,
    pub polarization_status: String,
    pub volatility_std_dev: f64,
    pub volatility_status: String,
    pub ambiguity_index: f64,
    pub ambiguity_status: String,
    pub position_bias: PositionBias,
    pub emotion_profile: Option<EmotionProfile>,
    pub slang_analysis: Option<SlangAnalysis>,
    pub block_stats: Vec<UnicodeBlockStat>,
    pub progression: SentimentProgression,
    pub bursts: Vec<EmojiBurst>,
    pub combos: Vec<EmojiCombo>,
    pub top_used: Vec<EmojiStat>,
    pub top_positive: Vec<EmojiStat>,
    pub top_negative: Vec<EmojiStat>,
    pub all_stats: Vec<EmojiStat>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MultiFileAnalysisResult {
    pub file_reports: Vec<FileReport>,
    pub aggregate: AnalysisResult,
}

pub struct Analyzer<'a> {
    dataset: &'a EmojiDataset,
}

impl<'a> Analyzer<'a> {
    pub fn new(dataset: &'a EmojiDataset) -> Self {
        Self { dataset }
    }

    pub fn analyze(&self, text: &str, top_n: usize) -> AnalysisResult {
        self.analyze_with_mode(text, top_n, SplitMode::Timeline)
    }

    pub fn analyze_with_mode(&self, text: &str, top_n: usize, split_mode: SplitMode) -> AnalysisResult {
        let total_chars = text.chars().count();
        let total_words = text
            .segment_str()
            .filter(|t| !t.trim().is_empty())
            .count();

        let mut counts: HashMap<String, usize> = HashMap::new();
        let mut emoji_sequence: Vec<String> = Vec::new();
        let mut emoji_positions: Vec<f64> = Vec::new();

        let mut current_char_idx = 0usize;

        for grapheme in text.graphemes(true) {
            let is_em = self.dataset.get_by_char(grapheme).is_some() || is_likely_emoji(grapheme);
            if is_em {
                *counts.entry(grapheme.to_string()).or_insert(0) += 1;
                emoji_sequence.push(grapheme.to_string());
                let rel_pos = if total_chars > 0 {
                    current_char_idx as f64 / total_chars as f64
                } else {
                    0.5
                };
                emoji_positions.push(rel_pos);
            }
            current_char_idx += grapheme.chars().count();
        }

        let mut all_stats = Vec::new();
        let mut total_emojis = 0usize;
        let mut matched_emojis_count = 0usize;
        let mut unmatched_emojis_count = 0usize;
        let mut total_weighted_score = 0.0f64;
        let mut total_weighted_intensity = 0.0f64;
        let mut positive_count = 0usize;
        let mut neutral_count = 0usize;
        let mut negative_count = 0usize;

        let mut block_counts: HashMap<String, (usize, f64)> = HashMap::new();
        let mut emotion_counts: HashMap<String, usize> = HashMap::new();

        for (emoji_str, count) in &counts {
            total_emojis += count;
            if let Some(info) = self.dataset.get_by_char(emoji_str) {
                matched_emojis_count += count;
                let score = info.score();
                let intensity = info.intensity();

                total_weighted_score += score * (*count as f64);
                total_weighted_intensity += intensity * (*count as f64);

                if score > 0.05 {
                    positive_count += count;
                } else if score < -0.05 {
                    negative_count += count;
                } else {
                    neutral_count += count;
                }

                let entry = block_counts.entry(info.block.clone()).or_insert((0, 0.0));
                entry.0 += count;
                entry.1 += score * (*count as f64);

                *emotion_counts.entry(info.primary_emotion.clone()).or_insert(0) += count;

                all_stats.push(EmojiStat {
                    emoji: emoji_str.clone(),
                    name: info.name.clone(),
                    count: *count,
                    score,
                    intensity,
                    in_dataset: true,
                });
            } else {
                unmatched_emojis_count += count;
                let entry = block_counts.entry("Unknown Category".to_string()).or_insert((0, 0.0));
                entry.0 += count;

                *emotion_counts.entry("Unknown".to_string()).or_insert(0) += count;

                all_stats.push(EmojiStat {
                    emoji: emoji_str.clone(),
                    name: "UNKNOWN EMOJI".to_string(),
                    count: *count,
                    score: 0.0,
                    intensity: 0.0,
                    in_dataset: false,
                });
            }
        }

        let overall_score = if matched_emojis_count > 0 {
            total_weighted_score / (matched_emojis_count as f64)
        } else {
            0.0
        };

        let overall_intensity = if matched_emojis_count > 0 {
            total_weighted_intensity / (matched_emojis_count as f64)
        } else {
            0.0
        };

        // Sentiment Volatility (Std Dev \sigma)
        let mut variance_sum = 0.0f64;
        if matched_emojis_count > 0 {
            for stat in &all_stats {
                if stat.in_dataset {
                    let diff = stat.score - overall_score;
                    variance_sum += (diff * diff) * (stat.count as f64);
                }
            }
        }
        let volatility_std_dev = if matched_emojis_count > 0 {
            (variance_sum / matched_emojis_count as f64).sqrt()
        } else {
            0.0
        };

        let volatility_status = if volatility_std_dev < 0.2 {
            "Monotone / Highly Consistent".to_string()
        } else if volatility_std_dev < 0.4 {
            "Balanced Volatility".to_string()
        } else {
            "High Volatility / Emotional Swing 🌊".to_string()
        };

        // Ambiguity & Neutrality Index
        let ambiguity_index = if total_emojis > 0 {
            (neutral_count as f64 / total_emojis as f64) * 100.0
        } else {
            0.0
        };

        let ambiguity_status = if ambiguity_index > 50.0 {
            "Subtle / Ambiguous 💭".to_string()
        } else if ambiguity_index >= 20.0 {
            "Balanced Expression".to_string()
        } else {
            "Direct & Explicit 🎯".to_string()
        };

        // Position Bias
        let mut sum_pos = 0.0f64;
        let mut front_cnt = 0usize;
        let mut mid_cnt = 0usize;
        let mut end_cnt = 0usize;

        for pos in &emoji_positions {
            sum_pos += *pos;
            if *pos < 0.33 {
                front_cnt += 1;
            } else if *pos < 0.66 {
                mid_cnt += 1;
            } else {
                end_cnt += 1;
            }
        }

        let avg_position = if !emoji_positions.is_empty() {
            sum_pos / emoji_positions.len() as f64
        } else {
            0.5
        };

        let front_pct = if !emoji_positions.is_empty() {
            (front_cnt as f64 / emoji_positions.len() as f64) * 100.0
        } else {
            0.0
        };
        let mid_pct = if !emoji_positions.is_empty() {
            (mid_cnt as f64 / emoji_positions.len() as f64) * 100.0
        } else {
            0.0
        };
        let end_pct = if !emoji_positions.is_empty() {
            (end_cnt as f64 / emoji_positions.len() as f64) * 100.0
        } else {
            0.0
        };

        let bias_status = if front_pct >= 50.0 {
            "Front-loaded Preferred".to_string()
        } else if end_pct >= 50.0 {
            "Trailing / End-loaded Preferred".to_string()
        } else {
            "Balanced Placement".to_string()
        };

        let position_bias = PositionBias {
            avg_position,
            front_pct,
            mid_pct,
            end_pct,
            bias_status,
        };

        // Entropy & Diversity
        let unique_emojis = counts.len();
        let mut entropy = 0.0f64;
        if total_emojis > 0 {
            for count in counts.values() {
                let p = (*count as f64) / (total_emojis as f64);
                if p > 0.0 {
                    entropy -= p * p.log2();
                }
            }
        }

        let diversity_ratio = if total_emojis > 0 {
            (unique_emojis as f64) / (total_emojis as f64)
        } else {
            0.0
        };

        let density_per_1000_chars = if total_chars > 0 {
            (total_emojis as f64 / total_chars as f64) * 1000.0
        } else {
            0.0
        };

        let density_per_100_words = if total_words > 0 {
            (total_emojis as f64 / total_words as f64) * 100.0
        } else {
            0.0
        };

        let style_level = if total_emojis == 0 {
            "Text Only".to_string()
        } else if density_per_100_words < 1.0 {
            "Formal / Minimal".to_string()
        } else if density_per_100_words < 5.0 {
            "Balanced / Casual".to_string()
        } else if density_per_100_words < 15.0 {
            "Expressive / Interactive".to_string()
        } else {
            "Heavy Emoji / Social".to_string()
        };

        // Polarization Index: 4 * pos_ratio * neg_ratio
        let pos_ratio = if total_emojis > 0 {
            positive_count as f64 / total_emojis as f64
        } else {
            0.0
        };
        let neg_ratio = if total_emojis > 0 {
            negative_count as f64 / total_emojis as f64
        } else {
            0.0
        };
        let polarization_index = (4.0 * pos_ratio * neg_ratio).min(1.0);

        let polarization_status = if polarization_index < 0.1 {
            "Harmonious / Unified".to_string()
        } else if polarization_index < 0.4 {
            "Slight Contrast".to_string()
        } else if polarization_index < 0.7 {
            "Mixed / Controversial".to_string()
        } else {
            "Highly Polarized 🔥❄️".to_string()
        };

        // Unicode Block Distribution
        let mut block_stats = Vec::new();
        for (block_name, (cnt, score_sum)) in block_counts {
            let percentage = if total_emojis > 0 {
                (cnt as f64 / total_emojis as f64) * 100.0
            } else {
                0.0
            };
            let avg_score = if cnt > 0 { score_sum / cnt as f64 } else { 0.0 };
            block_stats.push(UnicodeBlockStat {
                block_name,
                count: cnt,
                percentage,
                avg_score,
            });
        }
        block_stats.sort_by_key(|b| std::cmp::Reverse(b.count));
        block_stats.truncate(top_n);

        // Sentiment Progression
        let progression = self.calculate_progression(text, &emoji_sequence, split_mode);

        // Bursts & Streaks
        let mut bursts_map: HashMap<String, (usize, usize)> = HashMap::new();
        if !emoji_sequence.is_empty() {
            let mut current_emoji = &emoji_sequence[0];
            let mut current_streak = 1usize;

            for em in emoji_sequence.iter().skip(1) {
                if em == current_emoji {
                    current_streak += 1;
                } else {
                    if current_streak >= 2 {
                        let entry = bursts_map.entry(current_emoji.clone()).or_insert((0, 0));
                        entry.0 = entry.0.max(current_streak);
                        entry.1 += 1;
                    }
                    current_emoji = em;
                    current_streak = 1;
                }
            }
            if current_streak >= 2 {
                let entry = bursts_map.entry(current_emoji.clone()).or_insert((0, 0));
                entry.0 = entry.0.max(current_streak);
                entry.1 += 1;
            }
        }

        let mut bursts = Vec::new();
        for (emoji_str, (max_streak, total_bursts)) in bursts_map {
            let name = self
                .dataset
                .get_by_char(&emoji_str)
                .map(|d| d.name.clone())
                .unwrap_or_else(|| "UNKNOWN EMOJI".to_string());
            bursts.push(EmojiBurst {
                emoji: emoji_str,
                name,
                max_streak,
                total_bursts,
            });
        }
        bursts.sort_by(|a, b| b.max_streak.cmp(&a.max_streak).then_with(|| b.total_bursts.cmp(&a.total_bursts)));

        // Combos (Bigrams)
        let mut combo_counts: HashMap<String, usize> = HashMap::new();
        for window in emoji_sequence.windows(2) {
            let combo_key = format!("{}{}", window[0], window[1]);
            *combo_counts.entry(combo_key).or_insert(0) += 1;
        }

        let mut combos = Vec::new();
        for (combo, count) in combo_counts {
            combos.push(EmojiCombo { combo, count });
        }
        combos.sort_by_key(|b| std::cmp::Reverse(b.count));
        combos.truncate(top_n);

        // Top Used, Positive, Negative
        let mut top_used = all_stats.clone();
        top_used.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.emoji.cmp(&b.emoji)));
        top_used.truncate(top_n);

        let mut top_positive = all_stats
            .iter()
            .filter(|s| s.in_dataset && s.score > 0.0)
            .cloned()
            .collect::<Vec<_>>();
        top_positive.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| b.count.cmp(&a.count))
        });
        top_positive.truncate(top_n);

        let mut top_negative = all_stats
            .iter()
            .filter(|s| s.in_dataset && s.score < 0.0)
            .cloned()
            .collect::<Vec<_>>();
        top_negative.sort_by(|a, b| {
            a.score
                .partial_cmp(&b.score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| b.count.cmp(&a.count))
        });
        // GoEmotions Profile
        let mut emotion_stats = Vec::new();
        for (emotion, cnt) in emotion_counts {
            let percentage = if total_emojis > 0 {
                (cnt as f64 / total_emojis as f64) * 100.0
            } else {
                0.0
            };
            emotion_stats.push(EmotionStat {
                emotion,
                count: cnt,
                percentage,
            });
        }
        emotion_stats.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.emotion.cmp(&b.emotion)));

        let emotion_profile = if !emotion_stats.is_empty() {
            Some(EmotionProfile {
                primary_emotion: emotion_stats[0].emotion.clone(),
                top_emotions: emotion_stats,
            })
        } else {
            None
        };

        // Multilingual Slang & Sarcasm Analysis
        let mut slang_counts: HashMap<String, usize> = HashMap::new();
        let mut elongation_count = 0usize;

        for token in text.segment_str() {
            let trimmed = token.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Some(slang) = self.dataset.get_slang(trimmed) {
                *slang_counts.entry(slang.term.clone()).or_insert(0) += 1;
            }
            if is_elongated(trimmed) {
                elongation_count += 1;
            }
        }

        let total_slang_count: usize = slang_counts.values().sum();
        let mut slang_sarcasm_sum = 0.0f64;
        let mut slang_score_sum = 0.0f64;
        let mut top_slang = Vec::new();

        for (term, cnt) in &slang_counts {
            if let Some(slang) = self.dataset.get_slang(term) {
                slang_sarcasm_sum += slang.sarcasm_weight * (*cnt as f64);
                slang_score_sum += slang.sentiment_score * (*cnt as f64);
                top_slang.push(SlangStat {
                    term: slang.term.clone(),
                    count: *cnt,
                    sentiment_score: slang.sentiment_score,
                    sarcasm_weight: slang.sarcasm_weight,
                    meaning: slang.meaning.clone(),
                });
            }
        }
        top_slang.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.term.cmp(&b.term)));

        let sarcastic_emoji_count = counts
            .iter()
            .filter(|(em, _)| matches!(em.as_str(), "🤡" | "🫠" | "🙃" | "💅" | "🙄"))
            .map(|(_, c)| c)
            .sum::<usize>();

        let sarcasm_score_raw = if total_slang_count > 0 || sarcastic_emoji_count > 0 {
            (slang_sarcasm_sum + (sarcastic_emoji_count as f64 * 0.8))
                / ((total_slang_count + sarcastic_emoji_count) as f64)
        } else {
            0.0
        };
        let sarcasm_index = (sarcasm_score_raw * 100.0).min(100.0);

        let sarcasm_status = if sarcasm_index >= 70.0 {
            "High Sarcasm / Irony Alert 🎭".to_string()
        } else if sarcasm_index >= 30.0 {
            "Moderate Sarcasm / Playful Irony 😏".to_string()
        } else {
            "Direct & Literal Expression 🎯".to_string()
        };

        let slang_density_per_100_words = if total_words > 0 {
            (total_slang_count as f64 / total_words as f64) * 100.0
        } else {
            0.0
        };

        let hybrid_score = if matched_emojis_count + total_slang_count > 0 {
            (total_weighted_score + slang_score_sum)
                / ((matched_emojis_count + total_slang_count) as f64)
        } else {
            overall_score
        };

        let slang_analysis = if total_slang_count > 0 || elongation_count > 0 || sarcasm_index > 10.0 {
            Some(SlangAnalysis {
                total_slang_count,
                slang_density_per_100_words,
                sarcasm_index,
                sarcasm_status,
                elongation_count,
                hybrid_score,
                top_slang,
            })
        } else {
            None
        };

        AnalysisResult {
            total_chars,
            total_words,
            total_emojis,
            unique_emojis,
            matched_emojis_count,
            unmatched_emojis_count,
            overall_score,
            overall_intensity,
            positive_count,
            neutral_count,
            negative_count,
            density_per_1000_chars,
            density_per_100_words,
            style_level,
            entropy,
            diversity_ratio,
            polarization_index,
            polarization_status,
            volatility_std_dev,
            volatility_status,
            ambiguity_index,
            ambiguity_status,
            position_bias,
            emotion_profile,
            slang_analysis,
            block_stats,
            progression,
            bursts,
            combos,
            top_used,
            top_positive,
            top_negative,
            all_stats,
        }
    }

    pub fn analyze_multiple(&self, files: &[(&str, &str)], top_n: usize, split_mode: SplitMode) -> MultiFileAnalysisResult {
        let mut file_reports = Vec::new();
        let mut combined_text = String::new();

        for (file_name, content) in files {
            let res = self.analyze_with_mode(content, top_n, split_mode);
            let top_emoji = res.top_used.first().map(|e| e.emoji.clone()).unwrap_or_else(|| "None".to_string());
            file_reports.push(FileReport {
                file_name: file_name.to_string(),
                total_chars: res.total_chars,
                total_words: res.total_words,
                total_emojis: res.total_emojis,
                overall_score: res.overall_score,
                overall_intensity: res.overall_intensity,
                top_emoji,
            });
            combined_text.push_str(content);
            combined_text.push('\n');
        }

        let aggregate = self.analyze_with_mode(&combined_text, top_n, split_mode);

        MultiFileAnalysisResult {
            file_reports,
            aggregate,
        }
    }

    fn calculate_progression(&self, text: &str, emoji_sequence: &[String], mode: SplitMode) -> SentimentProgression {
        let mut segments = Vec::new();

        match mode {
            SplitMode::Paragraph => {
                let paras: Vec<&str> = text.split("\n\n").filter(|p| !p.trim().is_empty()).collect();
                for (idx, para) in paras.iter().enumerate() {
                    let seg_result = self.analyze_text_chunk(para);
                    segments.push(SentimentSegment {
                        label: format!("Paragraph {}", idx + 1),
                        score: seg_result.0,
                        intensity: seg_result.1,
                        emoji_count: seg_result.2,
                    });
                }
            }
            SplitMode::Line => {
                let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
                for (idx, line) in lines.iter().enumerate() {
                    let seg_result = self.analyze_text_chunk(line);
                    segments.push(SentimentSegment {
                        label: format!("Line {}", idx + 1),
                        score: seg_result.0,
                        intensity: seg_result.1,
                        emoji_count: seg_result.2,
                    });
                }
            }
            SplitMode::Timeline => {
                let total = emoji_sequence.len();
                if total > 0 {
                    let chunk_size = (total as f64 / 4.0).ceil() as usize;
                    let labels = ["Q1 (Beginning)", "Q2 (Early Mid)", "Q3 (Late Mid)", "Q4 (Ending)"];
                    for (i, label) in labels.iter().enumerate() {
                        let start = (i * chunk_size).min(total);
                        let end = ((i + 1) * chunk_size).min(total);
                        if start < end {
                            let slice = &emoji_sequence[start..end];
                            let seg_result = self.analyze_emoji_slice(slice);
                            segments.push(SentimentSegment {
                                label: label.to_string(),
                                score: seg_result.0,
                                intensity: seg_result.1,
                                emoji_count: seg_result.2,
                            });
                        }
                    }
                }
            }
        }

        let trend_status = compute_trend_status(&segments);

        SentimentProgression {
            segments,
            trend_status,
        }
    }

    fn analyze_text_chunk(&self, chunk: &str) -> (f64, f64, usize) {
        let mut count = 0usize;
        let mut score_sum = 0.0f64;
        let mut intensity_sum = 0.0f64;

        for grapheme in chunk.graphemes(true) {
            if let Some(info) = self.dataset.get_by_char(grapheme) {
                count += 1;
                score_sum += info.score();
                intensity_sum += info.intensity();
            }
        }

        let score = if count > 0 { score_sum / count as f64 } else { 0.0 };
        let intensity = if count > 0 { intensity_sum / count as f64 } else { 0.0 };

        (score, intensity, count)
    }

    fn analyze_emoji_slice(&self, slice: &[String]) -> (f64, f64, usize) {
        let mut count = 0usize;
        let mut score_sum = 0.0f64;
        let mut intensity_sum = 0.0f64;

        for grapheme in slice {
            if let Some(info) = self.dataset.get_by_char(grapheme) {
                count += 1;
                score_sum += info.score();
                intensity_sum += info.intensity();
            }
        }

        let score = if count > 0 { score_sum / count as f64 } else { 0.0 };
        let intensity = if count > 0 { intensity_sum / count as f64 } else { 0.0 };

        (score, intensity, count)
    }
}

fn compute_trend_status(segments: &[SentimentSegment]) -> String {
    if segments.is_empty() {
        return "N/A".to_string();
    }
    let valid_segs: Vec<&SentimentSegment> = segments.iter().filter(|s| s.emoji_count > 0).collect();
    if valid_segs.len() < 2 {
        return "Stable / Single Phase".to_string();
    }

    let first = valid_segs.first().unwrap().score;
    let last = valid_segs.last().unwrap().score;
    let diff = last - first;

    let all_positive = valid_segs.iter().all(|s| s.score > 0.05);
    let all_negative = valid_segs.iter().all(|s| s.score < -0.05);

    if all_positive {
        "Consistently Positive 😊".to_string()
    } else if all_negative {
        "Consistently Negative 🙁".to_string()
    } else if diff > 0.3 {
        "Warming Up 📈 (Negative → Positive)".to_string()
    } else if diff < -0.3 {
        "Cooling Down 📉 (Positive → Negative)".to_string()
    } else {
        "Fluctuating 🌊".to_string()
    }
}

fn is_likely_emoji(grapheme: &str) -> bool {
    let mut chars = grapheme.chars();
    if let Some(first) = chars.next() {
        let cp = first as u32;
        matches!(
            cp,
            0x1F600..=0x1F64F   // Emoticons
            | 0x1F300..=0x1F5FF // Misc Symbols and Pictographs
            | 0x1F680..=0x1F6FF // Transport and Map
            | 0x1F1E0..=0x1F1FF // Flags
            | 0x2600..=0x26FF   // Misc Symbols
            | 0x2700..=0x27BF   // Dingbats
            | 0x1F900..=0x1F9FF // Supplemental Symbols and Pictographs
            | 0x1FA70..=0x1FAFF // Symbols and Pictographs Extended-A
        )
    } else {
        false
    }
}

fn is_elongated(token: &str) -> bool {
    let chars: Vec<char> = token.chars().collect();
    if chars.len() < 3 {
        return false;
    }
    let mut repeat_count = 1usize;
    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            repeat_count += 1;
            if repeat_count >= 3 {
                return true;
            }
        } else {
            repeat_count = 1;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_with_text() {
        let dataset = EmojiDataset::load_embedded().unwrap();
        let analyzer = Analyzer::new(&dataset);

        let text = "I love Rust! 😂😍🎉 But bugs make me sad 😭";
        let result = analyzer.analyze(text, 5);

        assert_eq!(result.total_emojis, 4);
        assert_eq!(result.unique_emojis, 4);
        assert_eq!(result.matched_emojis_count, 4);

        assert_eq!(result.positive_count, 3);
        assert_eq!(result.negative_count, 1);
        assert!(result.overall_score > 0.0);
    }

    #[test]
    fn test_polarization_and_progression() {
        let dataset = EmojiDataset::load_embedded().unwrap();
        let analyzer = Analyzer::new(&dataset);

        let text = "Great news! 🎉😍 Bad news! 😭💔";
        let result = analyzer.analyze(text, 5);

        assert!(result.polarization_index > 0.5);
        assert!(!result.block_stats.is_empty());
        assert!(!result.progression.segments.is_empty());
    }

    #[test]
    fn test_multi_file_analysis() {
        let dataset = EmojiDataset::load_embedded().unwrap();
        let analyzer = Analyzer::new(&dataset);

        let files = [
            ("file1.txt", "Rust is awesome 🎉🚀"),
            ("file2.txt", "Bugs are annoying 😭💔"),
        ];

        let res = analyzer.analyze_multiple(&files, 5, SplitMode::Timeline);
        assert_eq!(res.file_reports.len(), 2);
        assert_eq!(res.aggregate.total_emojis, 4);
    }

    #[test]
    fn test_cjk_word_segmentation() {
        let dataset = EmojiDataset::load_embedded().unwrap();
        let analyzer = Analyzer::new(&dataset);

        let text = "我喜歡用 Rust 寫程式 🎉🚀";
        let result = analyzer.analyze(text, 5);

        assert!(result.total_words > 1);
        assert_eq!(result.total_emojis, 2);
    }

    #[test]
    fn test_emojinet_and_goemotions() {
        let dataset = EmojiDataset::load_embedded().unwrap();
        let analyzer = Analyzer::new(&dataset);

        let text = "Party time! 🥳 Modern emoji 🥺🤯🫠🥰!";
        let result = analyzer.analyze(text, 10);

        assert_eq!(result.total_emojis, 5);
        assert_eq!(result.matched_emojis_count, 5);
        assert_eq!(result.unmatched_emojis_count, 0);

        assert!(result.emotion_profile.is_some());
        let profile = result.emotion_profile.unwrap();
        assert!(!profile.top_emotions.is_empty());
    }

    #[test]
    fn test_multilingual_slang_and_sarcasm() {
        let dataset = EmojiDataset::load_embedded().unwrap();
        let analyzer = Analyzer::new(&dataset);

        let text = "tbh this is sooooo fire 🔥 笑死 破防 🤡 666 fr fr";
        let result = analyzer.analyze(text, 10);

        assert!(result.slang_analysis.is_some());
        let slang = result.slang_analysis.unwrap();
        assert!(slang.total_slang_count >= 3);
        assert!(slang.elongation_count >= 1);
        assert!(slang.sarcasm_index > 0.0);
    }
}
