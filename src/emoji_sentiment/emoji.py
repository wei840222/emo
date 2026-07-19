"""Module for emoji sentiment analysis and data structures."""

import csv
from importlib.resources import as_file, files
from pathlib import Path
from typing import Union, Dict, Annotated, List
from pydantic import BaseModel, Field
import emoji_data_python


class Emoji(BaseModel):
    """
    Emoji is a class that represents an emoji.
    """

    name: Annotated[str, Field(description="unicode name of the emoji")]
    short_names: Annotated[List[str], Field(
        description="short names of the emoji")]
    char: Annotated[str, Field(description="character of the emoji")]
    samples: Annotated[int, Field(
        description="number of samples used to calculate the sentiment score", gt=0)]
    score: Annotated[float, Field(
        description="sentiment score of the emoji", ge=-1, le=1)]


class EmojiSentiment():
    """
    EmojiSentiment is a class that provides a sentiment score for a given emoji.
    """

    CSV_RESOURCE = files("emoji_sentiment").joinpath(
        "data"
    ).joinpath("Emoji_Sentiment_Data_v1.0.csv")
    CSV_FILE_PATH = str(CSV_RESOURCE)

    def __init__(self, round_to: int = 3):
        self._data: Dict[str, Emoji] = {}
        csv_resource = self.CSV_RESOURCE
        if self.CSV_FILE_PATH != str(self.CSV_RESOURCE):
            csv_resource = Path(self.CSV_FILE_PATH)
        with as_file(csv_resource) as csv_path, csv_path.open(
            newline="", encoding="utf-8"
        ) as f:
            for row in csv.DictReader(f):
                samples = int(row["Negative"]) + \
                    int(row["Neutral"]) + int(row["Positive"])
                self._data[row["Unicode codepoint"]] = Emoji(
                    name=row["Unicode name"],
                    short_names=[name for emoji_char in emoji_data_python.find_by_name(
                        row["Unicode name"])[:1] for name in emoji_char.short_names],
                    char=row["Emoji"],
                    samples=samples,
                    score=round(int(row["Positive"]) / samples -
                                int(row["Negative"]) / samples, round_to)
                )

    @property
    def all(self) -> List[Emoji]:
        """Return a list of all emojis with their sentiment scores."""
        return list(self._data.values())

    def get(self, short_name: str) -> Union[Emoji, None]:
        """Return the emoji with the given short name."""
        if (emoji_char := emoji_data_python.emoji_short_names.get(short_name.lower())) is None:
            return None
        if (unicode := f"0x{emoji_char.unified.lower()}") not in self._data:
            return None
        return self._data[unicode]
