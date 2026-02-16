ORDINALS = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth",
]

GIFTS = [
    "twelve Drummers Drumming, ",
    "eleven Pipers Piping, ",
    "ten Lords-a-Leaping, ",
    "nine Ladies Dancing, ",
    "eight Maids-a-Milking, ",
    "seven Swans-a-Swimming, ",
    "six Geese-a-Laying, ",
    "five Gold Rings, ",
    "four Calling Birds, ",
    "three French Hens, ",
    "two Turtle Doves, ",
    "a Partridge in a Pear Tree.",
]


def recite(start_verse: int, end_verse: int) -> list[str]:
    verses = []
    for i in range(start_verse, end_verse + 1):
        gifts = GIFTS[-i:]
        if len(gifts) > 1:
            gifts[-1] = "and " + gifts[-1]
        verses.append(
            "".join(
                [
                    f"On the {ORDINALS[i - 1]} day of Christmas my true love gave to me: ",
                    *gifts,
                ]
            )
        )
    return verses
