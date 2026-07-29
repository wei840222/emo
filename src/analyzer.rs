use crate::dataset::EmojiDataset;
use serde::Serialize;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

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
    pub bursts: Vec<EmojiBurst>,
    pub combos: Vec<EmojiCombo>,
    pub top_used: Vec<EmojiStat>,
    pub top_positive: Vec<EmojiStat>,
    pub top_negative: Vec<EmojiStat>,
    pub all_stats: Vec<EmojiStat>,
}

pub struct Analyzer<'a> {
    dataset: &'a EmojiDataset,
}

impl<'a> Analyzer<'a> {
    pub fn new(dataset: &'a EmojiDataset) -> Self {
        Self { dataset }
    }

    pub fn analyze(&self, text: &str, top_n: usize) -> AnalysisResult {
        let total_chars = text.chars().count();
        let total_words = text.split_whitespace().count();

        let mut counts: HashMap<String, usize> = HashMap::new();
        let mut emoji_sequence: Vec<String> = Vec::new();

        for grapheme in text.graphemes(true) {
            if self.dataset.get_by_char(grapheme).is_some() || is_likely_emoji(grapheme) {
                *counts.entry(grapheme.to_string()).or_insert(0) += 1;
                emoji_sequence.push(grapheme.to_string());
            }
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

        // Calculate Bursts & Streaks
        let mut bursts_map: HashMap<String, (usize, usize)> = HashMap::new(); // emoji -> (max_streak, total_bursts)
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

        // Calculate Combos (Bigrams)
        let mut combo_counts: HashMap<String, usize> = HashMap::new();
        for window in emoji_sequence.windows(2) {
            let combo_key = format!("{}{}", window[0], window[1]);
            *combo_counts.entry(combo_key).or_insert(0) += 1;
        }

        let mut combos = Vec::new();
        for (combo, count) in combo_counts {
            combos.push(EmojiCombo { combo, count });
        }
        combos.sort_by(|a, b| b.count.cmp(&a.count));
        combos.truncate(top_n);

        // Sort for top lists
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
        top_negative.truncate(top_n);

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
            bursts,
            combos,
            top_used,
            top_positive,
            top_negative,
            all_stats,
        }
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
    fn test_bursts_and_combos() {
        let dataset = EmojiDataset::load_embedded().unwrap();
        let analyzer = Analyzer::new(&dataset);

        let text = "Super fast 🔥🔥🔥! Rocket launch 🚀🚀🔥";
        let result = analyzer.analyze(text, 5);

        assert!(!result.bursts.is_empty());
        let fire_burst = result.bursts.iter().find(|b| b.emoji == "🔥").unwrap();
        assert_eq!(fire_burst.max_streak, 3);

        assert!(!result.combos.is_empty());
        let fire_combo = result.combos.iter().find(|c| c.combo == "🔥🔥").unwrap();
        assert!(fire_combo.count >= 2);
    }
}
