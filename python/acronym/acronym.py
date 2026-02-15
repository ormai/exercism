def abbreviate(words: str) -> str:
    return "".join(
        map(lambda w: w[0], words.replace("_", "").replace("-", " ").upper().split())
    )
