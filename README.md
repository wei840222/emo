# emo - Emoji Sentiment Analyzer CLI

A fast and zero-dependency Rust CLI tool for analyzing emoji usage, sentiment scores, and sentiment intensity in text files or standard input (`stdin`).

Powered by the **Emoji Sentiment Ranking 1.0** dataset.

> **Disclaimer**: Built with AI.

## Features

- ⚡ **Zero external runtime dependencies**: The entire dataset is embedded in the compiled binary at build time.
- 📊 **Comprehensive Sentiment & Intensity Metrics**: Calculates weighted sentiment scores (`-1.0` to `+1.0`), emotional intensity (`0.0` to `1.0`), usage density, Shannon entropy, and positive/neutral/negative breakdowns.
- ⚖️ **Polarization Index & Emotional Shift**: Detects emotional conflicts (contrasting positive and negative emojis) and tracks emotional progression across text timeline, paragraphs, or lines.
- 🔥 **Bursts & Combos Detection**: Identifies repeated emoji bursts (streaks like `🔥🔥🔥`) and frequent adjacent pairs (combos like `🔥🚀`).
- 🎨 **Beautiful Terminal Output**: Displays colorful progress, style ratings, and clean borderless list/card views (immune to terminal emoji width grid misalignment).
- 🤖 **Pipeline & JSON Ready**: Supports `--json` / `-j` and `--summary` / `-s` for script automation and CI/CD integration.
- 📥 **Flexible Inputs & Relative Paths**: Supports reading from multiple files, directories (displaying paths relative to the input folder), or stdin piping (`cat file.txt | emo`).

## Metrics & Output Definitions

| **Multi-File Benchmark** | Aggregated per file | When passing multiple files or directories, benchmarks each file side-by-side (`File Name`, `Emojis`, `Score`, `Intensity`, `Top Emoji`) using relative path display. |
| **Placement Bias** | Relative position ($0.0$ to $1.0$) | Evaluates whether emojis are `Front-loaded` ($0-33\%$), `Balanced Placement`, or `Trailing / End-loaded` ($66-100\%$). |
| **Sentiment Volatility (\(\sigma\))** | Standard Deviation of scores | Measures emotional stability vs mood swings (`Monotone / Consistent` $\sigma < 0.2$ vs `High Volatility` $\sigma \ge 0.4$). |
| **Ambiguity Index (%)** | Neutral emoji ratio | Measures neutral/reserved expression (`Subtle / Ambiguous 💭` $> 50\%$ vs `Direct & Explicit 🎯` $< 20\%$). |
| **Overall Sentiment Score** | \(\frac{\sum (\text{Score}_i \times \text{Count}_i)}{\sum \text{Count}_i}\) | **Weighted Average Sentiment** (`-1.0` to `+1.0`). Single emoji score is \(\frac{\text{Positive} - \text{Negative}}{\text{Total}}\). |
| **Sentiment Intensity** | \(\frac{\sum (\text{Intensity}_i \times \text{Count}_i)}{\sum \text{Count}_i}\) | **Emotional Intensity / Non-Neutrality** (`0.0` to `1.0`). Single emoji intensity is \(\frac{\text{Positive} + \text{Negative}}{\text{Total}}\). Measures emotional involvement regardless of positive/negative polarity. |
| **Polarization Index** | \(4 \times P_{\text{pos}} \times P_{\text{neg}}\) | **Emotional Conflict Index** (`0.0` to `1.0`). Detects whether a text contains contrasting positive and negative emojis simultaneously (`Harmonious` vs `Highly Polarized`). |
| **Unicode Block Breakdown** | Aggregated by `Unicode block` | Groups emoji usage by official Unicode category (e.g. `Emoticons`, `Transport and Map Symbols`) with average sentiment per category. |
| **Sentiment Progression Arc** | 4-Quarter Timeline / Segmented | Tracks the emotional trajectory across the text to detect trends (`Warming Up 📈`, `Cooling Down 📉`, `Consistently Positive`, or `Fluctuating 🌊`). |
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
echo "Great news today! 🎉😍 Rocket launch 🚀! But oh no, servers crashed 😭💔..." | emo
```

### Analyze files or directories:

```bash
# Single file
emo document.txt

# Multiple files or entire folder (paths shown relative to input path)
emo README.md AGENTS.md
emo path/to/logs_folder/
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
  -j, --json              Output results as formatted JSON
  -s, --summary           Output short one-line summary
  -t, --top <TOP>         Number of top emojis and categories to display [default: 10]
      --by-paragraph      Calculate sentiment progression by paragraph (\n\n)
      --by-line           Calculate sentiment progression by line (\n)
      --no-color          Disable terminal color output
  -h, --help              Print help
  -V, --version           Print version
```

## Sample Report Preview

```text
================================================
  EMOJI SENTIMENT ANALYSIS REPORT 📊
================================================
 📄 Total Text Characters : 84
 🔤 Total Words Scanned   : 15
 😊 Emojis Found         : 5 (5 Unique)
 🎯 Matched in Dataset    : 5 (Unmatched: 0)
 📈 Overall Score        : 0.345 (Positive 😊) [-1.0 ~ +1.0]
 ⚡ Sentiment Intensity  : 0.742 [0.0 ~ 1.0]
 🌊 Sentiment Volatility  : σ = 0.409 (High Volatility 🌊) [≥ 0.0]
 ⚖️  Polarization Index   : 0.960 (Highly Polarized 🔥❄️) [0.0 ~ 1.0]
 💭 Ambiguity & Neutral   : 0.0% Neutral (Direct & Explicit 🎯) [0.0% ~ 100.0%]

📐 Expression Metrics & Placement
 🏷️  Text Style Level     : Heavy Emoji / Social
 📏 Emoji Density        : 59.52 per 1,000 chars / 33.33 per 100 words
 🌀 Diversity & Entropy   : 2.322 bits (Unique ratio: 100.0%)
 📍 Placement Bias        : Avg Pos: 0.53 (Trailing / End-loaded Preferred)
    └─ Distribution       : Front: 0.0% | Mid: 50.0% | End: 50.0%

 Breakdown Distribution
 [██████████████████████████████]
 Positive: 3 (60.0%) | Neutral: 0 (0.0%) | Negative: 2 (40.0%)

📂 Top Unicode Emoji Categories
    • Miscellaneous Symbols and Pictographs — Count: 2    ( 40.0%) | Avg Score: +0.309
    • Emoticons                — Count: 2    ( 40.0%) | Avg Score: +0.292
    • Transport and Map Symbols — Count: 1    ( 20.0%) | Avg Score: +0.525

📈 Sentiment Progression Arc
 Trend Status: Cooling Down 📉 (Positive → Negative)
    • Q1 (Beginning)   — Emojis: 2   | Score: +0.709 | Intensity: 0.799
    • Q2 (Early Mid)   — Emojis: 2   | Score: +0.216 | Intensity: 0.702
    • Q3 (Late Mid)    — Emojis: 1   | Score: -0.122 | Intensity: 0.707

📌 Most Used Emojis
   1. 🎉 (PARTY POPPER)       — Count: 1    | Score: +0.740 (Positive)
   2. 💔 (BROKEN HEART)       — Count: 1    | Score: -0.122 (Negative)
   3. 😍 (SMILING FACE WITH HEART-SHAPED EYES) — Count: 1    | Score: +0.678 (Positive)
   4. 😭 (LOUDLY CRYING FACE) — Count: 1    | Score: -0.093 (Negative)
   5. 🚀 (ROCKET)             — Count: 1    | Score: +0.525 (Positive)

🔗 Frequent Emoji Combos
    • 🎉😍     — 1 occurrences
    • 😭💔     — 1 occurrences
    • 😍🚀     — 1 occurrences
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

## Related Emoji Sentiment & Emotion Datasets

While `emo` relies on **Emoji Sentiment Ranking 1.0** for compiled zero-dependency runtime evaluation, the NLP research community offers several notable benchmark datasets for emoji sentiment and emotion analysis:

1. **Emoji Sentiment Ranking 1.0** *(Built-in)*
   - **Authors**: Petra Kralj Novak et al. (2015)
   - **Focus**: Evaluates sentiment polarity scores (`-1.0` to `+1.0`) for 751 common emojis based on 1.6M+ annotated tweets.
   - **Resource**: [CLARIN.SI hdl:11356/1048](http://hdl.handle.net/11356/1048) / [PLOS ONE Paper](https://doi.org/10.1371/journal.pone.0144296)

2. **TweetEval / SemEval-2018 Task 2 (Emoji Prediction)**
   - **Authors**: Barbieri et al. (2018)
   - **Focus**: Standard NLP benchmark for predicting appropriate emoji usage from text context across 20 common emoji classes.
   - **Resource**: Available on Hugging Face Datasets (`tweet_eval` dataset, `emoji` subset).

3. **DeepMoji Dataset (Distant Supervision Corpus)**
   - **Authors**: Felbo et al. (EMNLP 2017)
   - **Focus**: 1.2 billion Twitter posts categorized across 64 emoji labels. Widely used for transfer learning, sarcasm detection, and sentiment representations.
   - **Resource**: [GitHub - bfelbo/DeepMoji](https://github.com/bfelbo/DeepMoji)

4. **EmojiNet (Emoji Sense & Polarity Knowledge Base)**
   - **Authors**: Knoesis Institute (Sanjaya et al.)
   - **Focus**: Structured knowledge base linking over 2,300+ emojis to BabelNet synsets, sentiment polarities, and word sense disambiguation (e.g. 🙏 as praying vs high-five).
   - **Resource**: [emojinet.knoesis.org](http://emojinet.knoesis.org/)

5. **Google GoEmotions (Fine-Grained Emotion Dataset)**
   - **Authors**: Google Research (Demszky et al., 2020)
   - **Focus**: 58k Reddit comments labeled across 27 fine-grained emotions plus Neutral, commonly mapped 1-to-1 to emoji representations.
   - **Resource**: Available on Hugging Face (`google/go_emotions`).

6. **Social Media Slang & Emoji Sentiment Corpus** *(Built-in Multilingual Slang Dataset)*
   - **Focus**: UGC social media corpus containing slang, acronyms, CJK internet vocabulary (Traditional/Simplified Chinese, Japanese, Korean), elongated words, and explicit sarcasm (`Positive`, `Negative`, `Neutral`, `Sarcastic`).
   - **Resource**: Kaggle (`Social Media Slang & Emoji Sentiment`) & `assets/Multilingual_Slang_v1.0.csv`.

The source code is licensed under the MIT License (`LICENSE`).
