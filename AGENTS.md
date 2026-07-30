# AGENTS.md - Repository Guidelines & Instructions

This repository contains **`emo`**, a zero-dependency Rust CLI tool for analyzing emoji usage, sentiment scores, and sentiment intensity in text files or standard input (`stdin`).

## Project Architecture & Layout

- **`src/main.rs`**: CLI binary entry point. Parses CLI arguments using `clap` and handles directory traversal with relative path display.
- **`src/lib.rs`**: Re-exports core library modules (`dataset`, `analyzer`, `formatter`).
- **`src/dataset.rs`**: Loads and indexes the embedded CSV dataset (`include_str!`) into memory at compile time.
- **`src/analyzer.rs`**: Scans input text using `unicode-segmentation` grapheme clusters, matches emojis against dataset, calculates weighted sentiment scores (`-1.0` to `+1.0`) and sentiment intensity.
- **`src/formatter.rs`**: Formats output into colored borderless terminal card/list views, JSON, or summary mode.
- **`assets/Emoji_Sentiment_Data_v1.0.csv`**: Bundled Emoji Sentiment Ranking 1.0 research dataset. **Do not move or delete this file**.

## Development & Verification Commands

AI agents working on this codebase must verify changes using the following standard Cargo commands:

- **Run unit tests**:
  ```bash
  cargo test
  ```
- **Type check & build check**:
  ```bash
  cargo check
  ```
- **Build debug binary**:
  ```bash
  cargo build
  ```
- **Build release binary**:
  ```bash
  cargo build --release
  ```
- **Run clippy linter**:
  ```bash
  cargo clippy -- -D warnings
  ```

## Coding & Style Guidelines

1. **Idiomatic Rust**:
   - Use Rust 2024 edition conventions.
   - Use `anyhow::Result` for application error handling with descriptive `.context()`.
   - Keep module dependencies clean and modular.

2. **Emoji & Unicode Handling**:
   - Always iterate over grapheme clusters (`UnicodeSegmentation::graphemes(text, true)`) rather than raw `char` or byte slices when parsing emojis.

3. **Output Formats & Terminal Layout**:
   - Terminal output uses a **borderless clean card/list layout** (without grid borders like `comfy_table`) to prevent emoji width alignment issues across terminal fonts.
   - Directory scans output file paths relative to the input folder prefix.
   - Whenever adding or modifying sentiment metrics in `AnalysisResult` ([`src/analyzer.rs`](src/analyzer.rs)), update both:
     - Borderless terminal list renderer ([`src/formatter.rs`](src/formatter.rs))
     - JSON serialization output schema

4. **Zero External Runtime Data Dependency**:
   - The CSV dataset MUST remain compiled into the binary via `include_str!` in `src/dataset.rs`. Do not rely on external runtime file paths.
