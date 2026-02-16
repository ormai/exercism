# We want multiple keys with the same values. We define the inverted mapping
# were the unique values are mapped to the multiple respective keys. Then we
# reverse the mapping.
# Cfr. https://stackoverflow.com/a/76850966/15481080
SCORES = {
    v: k
    for k, values in {
        1: ["a", "e", "i", "o", "u", "l", "n", "r", "s", "t"],
        2: ["d", "g"],
        3: ["b", "c", "m", "p"],
        4: ["f", "h", "v", "w", "y"],
        5: ["k"],
        8: ["j", "x"],
        10: ["q", "z"],
    }.items()
    for v in values
}


def score(word: str) -> int:
    return sum(SCORES[letter] for letter in word.lower())
