use crate::analyzer::AnalysisResult;
use colored::*;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Color, ContentArrangement, Table};

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

fn render_summary(result: &AnalysisResult) {
    let sentiment_label = get_sentiment_label(result.overall_score);
    println!(
        "Emojis: {} ({} unique) | Score: {:.3} ({}) | Intensity: {:.3} | Style: {}",
        result.total_emojis,
        result.unique_emojis,
        result.overall_score,
        sentiment_label,
        result.overall_intensity,
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
        " 📈 Overall Score        : {} ({})",
        score_colored,
        get_sentiment_label(result.overall_score)
    );
    println!(
        " ⚡ Sentiment Intensity  : {:.3}",
        result.overall_intensity
    );

    // Advanced Metrics: Density & Diversity & Style
    println!("\n{}", "📐 Expression Metrics & Style".bold().underline());
    println!(
        " 🏷️  Text Style Level     : {}",
        result.style_level.bold().magenta()
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

    // Distribution Bar
    render_distribution_bar(result);

    // Top Emojis Table
    if !result.top_used.is_empty() {
        println!("\n{}", "📌 Most Used Emojis".bold().underline());
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Emoji", "Unicode Name", "Count", "Score", "Sentiment"]);

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
                Cell::new(&stat.emoji),
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
                Cell::new(&burst.emoji),
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
                Cell::new(&combo.combo),
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
