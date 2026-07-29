use crate::analyzer::{AnalysisResult, MultiFileAnalysisResult};
use colored::*;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Color, ContentArrangement, Table};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub enum OutputFormat {
    Table,
    Json,
    Summary,
}

pub fn render_output(result: &AnalysisResult, format: OutputFormat, no_color: bool) {
    if no_color {
        colored::control::set_override(false);
    }

    match format {
        OutputFormat::Json => {
            if let Ok(json) = serde_json::to_string_pretty(result) {
                println!("{}", json);
            }
        }
        OutputFormat::Summary => {
            render_summary(result);
        }
        OutputFormat::Table => {
            render_table(result);
        }
    }
}

pub fn render_multi_output(result: &MultiFileAnalysisResult, format: OutputFormat, no_color: bool) {
    if no_color {
        colored::control::set_override(false);
    }

    match format {
        OutputFormat::Json => {
            if let Ok(json) = serde_json::to_string_pretty(result) {
                println!("{}", json);
            }
        }
        OutputFormat::Summary => {
            println!(
                "Analyzed {} files | Total Emojis: {} | Aggregate Score: {:.3}",
                result.file_reports.len(),
                result.aggregate.total_emojis,
                result.aggregate.overall_score
            );
        }
        OutputFormat::Table => {
            println!("{}", "================================================".dimmed());
            println!(
                "  {} {}",
                "MULTI-FILE EMOJI BENCHMARK REPORT".bold().cyan(),
                "📂".to_string()
            );
            println!("{}", "================================================".dimmed());

            let mut comparison_table = Table::new();
            comparison_table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec![
                    "File Name",
                    "Total Emojis",
                    "Overall Score\n[-1.0 ~ +1.0]",
                    "Intensity\n[0.0 ~ 1.0]",
                    "Top Emoji",
                ]);

            for r in &result.file_reports {
                let score_str = format!("{:.3}", r.overall_score);
                let score_cell = if r.overall_score > 0.05 {
                    Cell::new(score_str).fg(Color::Green)
                } else if r.overall_score < -0.05 {
                    Cell::new(score_str).fg(Color::Red)
                } else {
                    Cell::new(score_str).fg(Color::Blue)
                };

                comparison_table.add_row(vec![
                    Cell::new(&r.file_name),
                    Cell::new(r.total_emojis),
                    score_cell,
                    Cell::new(format!("{:.3}", r.overall_intensity)),
                    Cell::new(sanitize_emoji_for_cell(&r.top_emoji)),
                ]);
            }
            println!("{}", comparison_table);

            println!("\n{}", "--- Aggregate Summary Report ---".dimmed().bold());
            render_table(&result.aggregate);
        }
    }
}

fn render_summary(result: &AnalysisResult) {
    let sentiment_label = get_sentiment_label(result.overall_score);
    println!(
        "Emojis: {} ({} unique) | Score: {:.3} ({}) | Intensity: {:.3} | Volatility σ: {:.3} | Style: {}",
        result.total_emojis,
        result.unique_emojis,
        result.overall_score,
        sentiment_label,
        result.overall_intensity,
        result.volatility_std_dev,
        result.style_level
    );
}

fn render_table(result: &AnalysisResult) {
    println!("{}", "================================================".dimmed());
    println!(
        "  {} {}",
        "EMOJI SENTIMENT ANALYSIS REPORT".bold().cyan(),
        "📊".to_string()
    );
    println!("{}", "================================================".dimmed());

    // Overview Stats
    println!(
        " 📄 Total Text Characters : {}",
        result.total_chars.to_string().yellow()
    );
    println!(
        " 🔤 Total Words Scanned   : {}",
        result.total_words.to_string().yellow()
    );
    println!(
        " 😊 Emojis Found         : {} ({} Unique)",
        result.total_emojis.to_string().bold().green(),
        result.unique_emojis.to_string().bold()
    );
    println!(
        " 🎯 Matched in Dataset    : {} (Unmatched: {})",
        result.matched_emojis_count.to_string().green(),
        result.unmatched_emojis_count.to_string().dimmed()
    );

    let score_str = format!("{:.3}", result.overall_score);
    let score_colored = if result.overall_score > 0.05 {
        score_str.bold().green()
    } else if result.overall_score < -0.05 {
        score_str.bold().red()
    } else {
        score_str.bold().blue()
    };

    println!(
        " 📈 Overall Score        : {} ({}) {}",
        score_colored,
        get_sentiment_label(result.overall_score),
        "[-1.0 ~ +1.0]".dimmed()
    );
    println!(
        " ⚡ Sentiment Intensity  : {:.3} {}",
        result.overall_intensity,
        "[0.0 ~ 1.0]".dimmed()
    );
    println!(
        " 🌊 Sentiment Volatility  : σ = {:.3} ({}) {}",
        result.volatility_std_dev,
        result.volatility_status.yellow(),
        "[≥ 0.0]".dimmed()
    );
    println!(
        " ⚖️  Polarization Index   : {:.3} ({}) {}",
        result.polarization_index,
        result.polarization_status.magenta().bold(),
        "[0.0 ~ 1.0]".dimmed()
    );
    println!(
        " 💭 Ambiguity & Neutral   : {:.1}% Neutral ({}) {}",
        result.ambiguity_index,
        result.ambiguity_status.blue(),
        "[0.0% ~ 100.0%]".dimmed()
    );

    // Advanced Metrics: Density & Diversity & Style & Position
    println!("\n{}", "📐 Expression Metrics & Placement".bold().underline());
    println!(
        " 🏷️  Text Style Level     : {}",
        result.style_level.bold().cyan()
    );
    println!(
        " 📏 Emoji Density        : {:.2} per 1,000 chars / {:.2} per 100 words",
        result.density_per_1000_chars, result.density_per_100_words
    );
    println!(
        " 🌀 Diversity & Entropy   : {:.3} bits (Unique ratio: {:.1}%)",
        result.entropy,
        result.diversity_ratio * 100.0
    );
    println!(
        " 📍 Placement Bias        : Avg Pos: {:.2} ({})",
        result.position_bias.avg_position,
        result.position_bias.bias_status.bold().magenta()
    );
    println!(
        "    └─ Distribution       : Front: {:.1}% | Mid: {:.1}% | End: {:.1}%",
        result.position_bias.front_pct,
        result.position_bias.mid_pct,
        result.position_bias.end_pct
    );

    // Distribution Bar
    render_distribution_bar(result);

    // GoEmotions Fine-Grained Emotion Spectrum
    if let Some(profile) = &result.emotion_profile {
        if !profile.top_emotions.is_empty() {
            println!("\n{}", "🎭 Fine-Grained Emotion Spectrum (GoEmotions)".bold().underline());
            println!(" Primary Emotion: {}", profile.primary_emotion.bold().magenta());
            let mut emo_table = Table::new();
            emo_table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec!["Emotion Category", "Emoji Count", "Percentage"]);

            for em in profile.top_emotions.iter().take(8) {
                emo_table.add_row(vec![
                    Cell::new(&em.emotion),
                    Cell::new(em.count),
                    Cell::new(format!("{:.1}%", em.percentage)),
                ]);
            }
            println!("{}", emo_table);
        }
    }

    // Unicode Block Distribution
    if !result.block_stats.is_empty() {
        println!("\n{}", "📂 Top Unicode Emoji Categories".bold().underline());
        let mut block_table = Table::new();
        block_table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Category / Block", "Count", "Percentage", "Avg Score\n[-1.0 ~ +1.0]"]);

        for block in &result.block_stats {
            block_table.add_row(vec![
                Cell::new(&block.block_name),
                Cell::new(block.count),
                Cell::new(format!("{:.1}%", block.percentage)),
                Cell::new(format!("{:.3}", block.avg_score)),
            ]);
        }
        println!("{}", block_table);
    }

    // Sentiment Progression / Arc
    if !result.progression.segments.is_empty() {
        println!("\n{}", "📈 Sentiment Progression Arc".bold().underline());
        println!(" Trend Status: {}", result.progression.trend_status.bold().yellow());
        let mut arc_table = Table::new();
        arc_table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Segment", "Emoji Count", "Score\n[-1.0 ~ +1.0]", "Intensity\n[0.0 ~ 1.0]"]);

        for seg in &result.progression.segments {
            arc_table.add_row(vec![
                Cell::new(&seg.label),
                Cell::new(seg.emoji_count),
                Cell::new(format!("{:.3}", seg.score)),
                Cell::new(format!("{:.3}", seg.intensity)),
            ]);
        }
        println!("{}", arc_table);
    }

    // Top Emojis Table
    if !result.top_used.is_empty() {
        println!("\n{}", "📌 Most Used Emojis".bold().underline());
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Emoji", "Unicode Name", "Count", "Score\n[-1.0 ~ +1.0]", "Sentiment"]);

        for stat in &result.top_used {
            let score_cell = if stat.in_dataset {
                let s = format!("{:.3}", stat.score);
                if stat.score > 0.05 {
                    Cell::new(s).fg(Color::Green)
                } else if stat.score < -0.05 {
                    Cell::new(s).fg(Color::Red)
                } else {
                    Cell::new(s).fg(Color::Blue)
                }
            } else {
                Cell::new("N/A").fg(Color::DarkGrey)
            };

            let category = if !stat.in_dataset {
                "Unknown"
            } else if stat.score > 0.05 {
                "Positive"
            } else if stat.score < -0.05 {
                "Negative"
            } else {
                "Neutral"
            };

            table.add_row(vec![
                Cell::new(sanitize_emoji_for_cell(&stat.emoji)),
                Cell::new(&stat.name),
                Cell::new(stat.count),
                score_cell,
                Cell::new(category),
            ]);
        }
        println!("{}", table);
    }

    // Bursts & Streaks
    if !result.bursts.is_empty() {
        println!("\n{}", "🔥 Top Emoji Bursts & Streaks".bold().underline());
        let mut burst_table = Table::new();
        burst_table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Emoji", "Unicode Name", "Max Streak", "Total Bursts"]);

        for burst in result.bursts.iter().take(5) {
            burst_table.add_row(vec![
                Cell::new(sanitize_emoji_for_cell(&burst.emoji)),
                Cell::new(&burst.name),
                Cell::new(format!("{}x", burst.max_streak)),
                Cell::new(burst.total_bursts),
            ]);
        }
        println!("{}", burst_table);
    }

    // Combos / Bigrams
    if !result.combos.is_empty() {
        println!("\n{}", "🔗 Frequent Emoji Combos".bold().underline());
        let mut combo_table = Table::new();
        combo_table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Combo Pair", "Occurrences"]);

        for combo in result.combos.iter().take(5) {
            combo_table.add_row(vec![
                Cell::new(sanitize_emoji_for_cell(&combo.combo)),
                Cell::new(combo.count),
            ]);
        }
        println!("{}", combo_table);
    }
}

fn render_distribution_bar(result: &AnalysisResult) {
    let total = result.total_emojis as f64;
    if total == 0.0 {
        return;
    }

    let pos_pct = (result.positive_count as f64 / total) * 100.0;
    let neu_pct = (result.neutral_count as f64 / total) * 100.0;
    let neg_pct = (result.negative_count as f64 / total) * 100.0;

    let bar_width: usize = 30;
    let pos_blocks = ((pos_pct / 100.0) * bar_width as f64).round() as usize;
    let neu_blocks = ((neu_pct / 100.0) * bar_width as f64).round() as usize;
    let neg_blocks = bar_width.saturating_sub(pos_blocks + neu_blocks);

    let bar = format!(
        "{}{}{}",
        "█".repeat(pos_blocks).green(),
        "█".repeat(neu_blocks).blue(),
        "█".repeat(neg_blocks).red()
    );

    println!("\n {}", "Breakdown Distribution".bold());
    println!(" [{}]", bar);
    println!(
        " Positive: {} ({:.1}%) | Neutral: {} ({:.1}%) | Negative: {} ({:.1}%)",
        result.positive_count.to_string().green(),
        pos_pct,
        result.neutral_count.to_string().blue(),
        neu_pct,
        result.negative_count.to_string().red(),
        neg_pct
    );
}

pub fn sanitize_emoji_for_cell(s: &str) -> String {
    let mut result = String::new();
    for g in s.graphemes(true) {
        let w = g.width();
        let is_em = g.chars().any(|c| {
            let cp = c as u32;
            matches!(
                cp,
                0x2600..=0x27BF
                    | 0x1F300..=0x1F5FF
                    | 0x1F600..=0x1F64F
                    | 0x1F680..=0x1F6FF
                    | 0x1F900..=0x1F9FF
                    | 0x1FA70..=0x1FAFF
            )
        });
        result.push_str(g);
        if is_em && w == 1 {
            // Soft hyphen U+00AD adds +1 to unicode_width for comfy-table, but is invisible/0-width in terminal!
            result.push('\u{00AD}');
        }
    }
    result
}

fn get_sentiment_label(score: f64) -> ColoredString {
    if score >= 0.5 {
        "Very Positive 😃".bold().green()
    } else if score > 0.05 {
        "Positive 😊".green()
    } else if score >= -0.05 {
        "Neutral 😐".blue()
    } else if score > -0.5 {
        "Negative 🙁".red()
    } else {
        "Very Negative 😭".bold().red()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comfy_table_emoji_alignment() {
        let mut table = Table::new();
        table.set_header(vec!["File", "Top Emoji"]);
        table.add_row(vec!["file1.txt", &sanitize_emoji_for_cell("✨")]);
        table.add_row(vec!["file2.txt", &sanitize_emoji_for_cell("⚙️")]);

        let rendered = table.to_string();
        assert!(rendered.contains("✨"));
        assert!(rendered.contains("⚙️"));
    }
}
