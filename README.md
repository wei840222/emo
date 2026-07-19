# Emoji Sentiment

A Python package for looking up sentiment scores from the Emoji Sentiment
Ranking 1.0 dataset.

<img width="531" alt="image" src="https://github.com/user-attachments/assets/5b53da6f-be97-4ec2-a2d5-dc09bd6f9917" />

The package provides a score from `-1` (negative) to `1` (positive), together
with the emoji name, character, short names, and sample count. Scores come from
the bundled research dataset; the package does not run a predictive model.

## Installation

Requires Python 3.14 or newer.

```bash
pip install emoji-sentiment
```

## Usage

```python
from emoji_sentiment import EmojiSentiment

emoji_sentiment = EmojiSentiment()
smile_emoji = emoji_sentiment.get("smile")
print(smile_emoji)
```

`get()` is case-insensitive and returns `None` when the short name is unknown or
not present in the dataset. Use `emoji_sentiment.all` to retrieve all 969
entries.

## Demo

```sh
uv run --extra demo streamlit run examples/streamlit_demo.py
```

## Development

Install the locked development environment:

```sh
uv sync --locked --dev
```

Run tests and lint checks:

```sh
uv run pytest --cov=emoji_sentiment --cov-report=term-missing -v tests
uv run pylint src/emoji_sentiment tests/test_emoji_sentiment.py tests/smoke_test.py
```

Build and validate the distribution archives:

```sh
uv build
uv run twine check --strict dist/*
```

## Data source and license

The bundled `Emoji_Sentiment_Data_v1.0.csv` is from:

> Kralj Novak, Petra; Smailović, Jasmina; Sluban, Borut; and Mozetič,
> Igor (2015), *Emoji Sentiment Ranking 1.0*, Slovenian language resource
> repository CLARIN.SI, ISSN 2820-4042.

- Resource: http://hdl.handle.net/11356/1048
- Paper: https://doi.org/10.1371/journal.pone.0144296
- Dataset license: Creative Commons Attribution-ShareAlike 4.0 International
  (CC BY-SA 4.0)

The package source code is licensed under the MIT License in `LICENSE`. The
bundled dataset remains under CC BY-SA 4.0. See
https://github.com/wei840222/emoji-sentiment/blob/main/NOTICE.md for full
attribution and third-party licensing details.
