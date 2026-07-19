"""Tests for the public emoji sentiment API and bundled dataset."""

from importlib.resources import files

import pytest

from emoji_sentiment import EmojiSentiment


def test_emoji_sentiment_get():
    """Look up known names case-insensitively and reject unknown names."""
    emoji_sentiment = EmojiSentiment()

    # test valid emoji name
    smile_emoji = emoji_sentiment.get("smile")
    assert smile_emoji is not None
    assert hasattr(smile_emoji, "name")
    assert len(smile_emoji.name) > 0
    assert hasattr(smile_emoji, "short_names")
    assert len(smile_emoji.short_names) > 0
    assert hasattr(smile_emoji, "char")
    assert len(smile_emoji.char) == 1
    assert hasattr(smile_emoji, "samples")
    assert smile_emoji.samples > 0
    assert hasattr(smile_emoji, "score")
    assert smile_emoji.score >= -1 and smile_emoji.score <= 1

    # test different case of emoji name
    assert emoji_sentiment.get("SMILE") is not None
    assert emoji_sentiment.get("Smile") is not None

    # test invalid emoji name
    assert emoji_sentiment.get("not_exist_emoji") is None
    assert emoji_sentiment.get("") is None

    # test special characters
    assert emoji_sentiment.get("@#$%") is None


def test_emoji_sentiment_initialization():
    """Load every row from the bundled dataset."""
    emoji_sentiment = EmojiSentiment()
    assert len(emoji_sentiment.all) == 969


def test_bundled_dataset_is_available_as_package_data():
    """Expose the CSV through the installed package resources."""
    dataset = files("emoji_sentiment").joinpath(
        "data", "Emoji_Sentiment_Data_v1.0.csv"
    )

    assert dataset.is_file()
    assert isinstance(EmojiSentiment.CSV_FILE_PATH, str)


def test_csv_file_path_override_is_used(monkeypatch, tmp_path):
    """Honor the legacy CSV path override used by subclasses and tests."""
    missing_dataset = tmp_path / "missing.csv"
    monkeypatch.setattr(EmojiSentiment, "CSV_FILE_PATH", str(missing_dataset))

    with pytest.raises(FileNotFoundError):
        EmojiSentiment()


@pytest.mark.parametrize("short_names", ["smile", "eyes", "thumbsup", "joy", "sunglasses"])
def test_common_emojis(short_names):
    """Resolve representative short names from the underlying emoji catalog."""
    emoji_sentiment = EmojiSentiment()
    emoji = emoji_sentiment.get(short_names)
    assert emoji is not None
