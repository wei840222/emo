"""Verify that an installed distribution contains its runtime data."""

from importlib.metadata import version
from importlib.resources import files

from emoji_sentiment import EmojiSentiment


package_version = version("emoji-sentiment")
dataset = files("emoji_sentiment").joinpath(
    "data", "Emoji_Sentiment_Data_v1.0.csv"
)
sentiment = EmojiSentiment()

assert package_version != "0.0.0"
assert dataset.is_file()
assert len(sentiment.all) == 969
assert sentiment.get("smile") is not None

print(f"emoji-sentiment {package_version} smoke test passed")
