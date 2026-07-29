# emo - Emoji Sentiment Analyzer CLI

A fast and zero-dependency Rust CLI tool for analyzing emoji usage, sentiment scores, and sentiment intensity in text files or standard input (`stdin`).

Powered by the **Emoji Sentiment Ranking 1.0** dataset.

> **Disclaimer**: Built with AI.

## Features

- ⚡ **Zero external runtime dependencies**: The entire dataset is embedded in the compiled binary at build time.
- 📊 **Comprehensive Sentiment & Intensity Metrics**: Calculates weighted sentiment scores (`-1.0` to `+1.0`), emotional intensity (`0.0` to `1.0`), usage density, Shannon entropy, and positive/neutral/negative breakdowns.
- 🔥 **Bursts & Combos Detection**: Identifies repeated emoji bursts (streaks like `🔥🔥🔥`) and frequent adjacent pairs (combos like `🔥🚀`).
- 🎨 **Beautiful Terminal Output**: Displays colorful progress, style ratings, and formatted tables in terminal mode.
- 🤖 **Pipeline & JSON Ready**: Supports `--json` / `-j` and `--summary` / `-s` for script automation and CI/CD integration.
- 📥 **Flexible Inputs**: Supports reading from multiple files, directories, or stdin piping (`cat file.txt | emo`).

## Metrics & Output Definitions

| Metric | Formula / Calculation | Description |
| :--- | :--- | :--- |
| **Overall Sentiment Score** | \(\frac{\sum (\text{Score}_i \times \text{Count}_i)}{\sum \text{Count}_i}\) | **Weighted Average Sentiment** (`-1.0` to `+1.0`). Single emoji score is \(\frac{\text{Positive} - \text{Negative}}{\text{Total}}\). |
| **Sentiment Intensity** | \(\frac{\sum (\text{Intensity}_i \times \text{Count}_i)}{\sum \text{Count}_i}\) | **Emotional Intensity / Non-Neutrality** (`0.0` to `1.0`). Single emoji intensity is \(\frac{\text{Positive} + \text{Negative}}{\text{Total}}\). Measures emotional involvement regardless of positive/negative polarity. |
| **Emoji Density** | \(\frac{\text{Emojis}}{\text{Chars}} \times 1000\) / \(\frac{\text{Emojis}}{\text{Words}} \times 100\) | Frequency density of emojis per 1,000 characters and per 100 words. |
| **Style Level** | Based on Emoji Density | Categorizes text expression style: `Text Only`, `Formal / Minimal`, `Balanced / Casual`, `Expressive / Interactive`, or `Heavy Emoji / Social`. |
| **Shannon Entropy** | \(H = -\sum p_i \log_2(p_i)\) | Measures vocabulary diversity in bits. Higher entropy indicates a broader variety of emojis used rather than repeating a single emoji. |
| **Diversity Ratio** | \(\frac{\text{Unique Emojis}}{\text{Total Emojis}}\) | Ratio of unique emoji characters to total emoji occurrences (`0.0` to `1.0`). |
| **Bursts & Streaks** | Consecutive repeats (\(\ge 2\)) | Detects consecutive repeats of the same emoji (e.g. `🔥🔥🔥` $\rightarrow$ Max Streak: 3). |
| **Emoji Combos** | Bigram pair counting | Detects frequent adjacent emoji pairs in sequence (e.g. `🔥🚀` or `🎉😍`). |

## Installation & Building

Requires **Rust 1.80+**.

```bash
# Build release binary
cargo build --release

# Install binary to ~/.cargo/bin
cargo install --path .
```

## Usage

### Analyze text via stdin:

```bash
echo "Rust is amazing! 🎉🚀 But debugging can be tough 😭" | emo
```

### Analyze files:

```bash
emo document.txt README.md
```

### Output JSON for automation:

```bash
emo --json log.txt
```

### Options

```
Usage: emo [OPTIONS] [FILE]...

Arguments:
  [FILE]...  File(s) to analyze. If empty or '-', reads from stdin.

Options:
  -j, --json          Output results as formatted JSON
  -s, --summary       Output short one-line summary
  -t, --top <TOP>     Number of top emojis to display in report [default: 10]
      --no-color      Disable terminal color output
  -h, --help          Print help
  -V, --version       Print version
```

## Development & Testing

Run unit tests:

```bash
cargo test
```

## Data Source & License

The bundled `Emoji_Sentiment_Data_v1.0.csv` is from:

> Kralj Novak, Petra; Smailović, Jasmina; Sluban, Borut; and Mozetič,
> Igor (2015), *Emoji Sentiment Ranking 1.0*, Slovenian language resource
> repository CLARIN.SI, ISSN 2820-4042.

- Resource: http://hdl.handle.net/11356/1048
- Paper: https://doi.org/10.1371/journal.pone.0144296
- Dataset License: Creative Commons Attribution-ShareAlike 4.0 International (CC BY-SA 4.0)

The source code is licensed under the MIT License (`LICENSE`).
