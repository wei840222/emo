use anyhow::{Context, Result};
use clap::Parser;
use emo::{render_multi_output, render_output, Analyzer, EmojiDataset, OutputFormat, SplitMode};
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
    /// File(s) or directory(ies) to analyze. If empty or '-', reads from stdin.
    #[arg(value_name = "FILE")]
    files: Vec<PathBuf>,

    /// Output results as formatted JSON
    #[arg(short, long)]
    json: bool,

    /// Output short one-line summary
    #[arg(short, long)]
    summary: bool,

    /// Number of top emojis and categories to display
    #[arg(short = 't', long, default_value_t = 10)]
    top: usize,

    /// Calculate sentiment progression by paragraph (split by \\n\\n)
    #[arg(long, group = "split_group")]
    by_paragraph: bool,

    /// Calculate sentiment progression by line (split by \\n)
    #[arg(long, group = "split_group")]
    by_line: bool,

    /// Disable terminal color output
    #[arg(long)]
    no_color: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let dataset = EmojiDataset::load_embedded().context("Failed to load embedded emoji dataset")?;
    let analyzer = Analyzer::new(&dataset);

    let split_mode = if cli.by_paragraph {
        SplitMode::Paragraph
    } else if cli.by_line {
        SplitMode::Line
    } else {
        SplitMode::Timeline
    };

    let format = if cli.json {
        OutputFormat::Json
    } else if cli.summary {
        OutputFormat::Summary
    } else {
        OutputFormat::Table
    };

    if cli.files.is_empty() || (cli.files.len() == 1 && cli.files[0].to_str() == Some("-")) {
        let mut text_buffer = String::new();
        io::stdin()
            .read_to_string(&mut text_buffer)
            .context("Failed to read from stdin")?;
        let result = analyzer.analyze_with_mode(&text_buffer, cli.top, split_mode);
        render_output(&result, format, cli.no_color);
        return Ok(());
    }

    let expanded_files = collect_filepaths(&cli.files)?;

    if expanded_files.is_empty() {
        eprintln!("No readable files found.");
        return Ok(());
    }

    if expanded_files.len() == 1 {
        let path = &expanded_files[0];
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;
        let result = analyzer.analyze_with_mode(&content, cli.top, split_mode);
        render_output(&result, format, cli.no_color);
    } else {
        let mut file_data = Vec::new();
        for path in &expanded_files {
            if let Ok(content) = fs::read_to_string(path) {
                let name_str = path.to_string_lossy().to_string();
                file_data.push((name_str, content));
            }
        }

        let refs: Vec<(&str, &str)> = file_data.iter().map(|(n, c)| (n.as_str(), c.as_str())).collect();
        let multi_result = analyzer.analyze_multiple(&refs, cli.top, split_mode);
        render_multi_output(&multi_result, format, cli.no_color);
    }

    Ok(())
}

fn collect_filepaths(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for p in paths {
        if p.is_dir() {
            let entries = fs::read_dir(p)
                .with_context(|| format!("Failed to read directory: {}", p.display()))?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if !name.starts_with('.') {
                            files.push(path);
                        }
                    }
                } else if path.is_dir() {
                    let sub_files = collect_filepaths(&[path])?;
                    files.extend(sub_files);
                }
            }
        } else {
            files.push(p.clone());
        }
    }
    files.sort();
    Ok(files)
}
