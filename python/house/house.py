VERSES = [
    "the malt that lay",
    "the rat that ate",
    "the cat that killed",
    "the dog that worried",
    "the cow with the crumpled horn that tossed",
    "the maiden all forlorn that milked",
    "the man all tattered and torn that kissed",
    "the priest all shaven and shorn that married",
    "the rooster that crowed in the morn that woke",
    "the farmer sowing his corn that kept",
    "the horse and the hound and the horn that belonged to",
]


def recite(start_verse, end_verse):
    return [
        "This is "
        + ((" ".join(VERSES[v - 2 :: -1]) + " in ") if v != 1 else "")
        + "the house that Jack built."
        for v in range(start_verse, end_verse + 1)
    ]
