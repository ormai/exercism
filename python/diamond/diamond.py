def rows(letter: str) -> list[str]:
    """Creates the diamond for the given letter."""

    diamond = []
    size = (ord(letter) - ord("A") + 1) * 2 - 1
    mask = [
        *enumerate(range(ord("A"), ord(letter))),
        *zip(
            range(ord(letter) - ord("A"), -1, -1), range(ord(letter), ord("A") - 1, -1)
        ),
    ]

    for i, r in mask:
        row = list(" " for _ in range(size))
        char = chr(r)
        if i > 0:
            row[size // 2 + i] = row[size // 2 - i] = char
        else:
            row[size // 2] = char
        diamond.append("".join(row))

    return diamond
