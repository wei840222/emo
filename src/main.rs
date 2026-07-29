use anyhow::{Context, Result};
use clap::Parser;
use emo::{render_output, Analyzer, EmojiDataset, OutputFormat};
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "emo",
    author = "Wan, Jiun-Wei <wei840222@gmail.com>",
    version = "0.1.0",
    about = "Emoji usage & sentiment analysis CLI tool powered by Emoji Sentiment Ranking dataset"
)]
struct Cli {
    /// File(s) to analyze. If empty or '-', reads from stdin.
    #[arg(value_name = "FILE")]
    files: Vec<PathBuf>,

    /// Output results as formatted JSON
    #[arg(short, long)]
    json: bool,

    /// Output short one-line summary
    #[arg(short, long)]
    summary: bool,

    /// Number of top emojis to display in report
    #[arg(short = 't', long, default_value_t = 10)]
    top: usize,

    /// Disable terminal color output
    #[arg(long)]
    no_color: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let dataset = EmojiDataset::load_embedded().context("Failed to load embedded emoji dataset")?;
    let analyzer = Analyzer::new(&dataset);

    let mut text_buffer = String::new();

    if cli.files.is_empty() || (cli.files.len() == 1 && cli.files[0].to_str() == Some("-")) {
        io::stdin()
            .read_to_string(&mut text_buffer)
            .context("Failed to read from stdin")?;
    } else {
        for path in &cli.files {
            let content = fs::read_to_string(path)
                .with_context(|| format!("Failed to read file: {}", path.display()))?;
            text_buffer.push_str(&content);
            text_buffer.push('\n');
        }
    }

    let result = analyzer.analyze(&text_buffer, cli.top);

    let format = if cli.json {
        OutputFormat::Json
    } else if cli.summary {
        OutputFormat::Summary
    } else {
        OutputFormat::Table
    };

    render_output(&result, format, cli.no_color);

    Ok(())
}
