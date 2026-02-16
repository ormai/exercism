from collections import Counter
from re import split as re_split


def count_words(sentence: str) -> dict[str, int]:
    return Counter(
        map(
            lambda w: w.strip(".:',!@&$%^"),
            filter(lambda s: s != "", re_split(r"[\s,_]", sentence.lower())),
        )
    )
