codes = [
    "black",
    "brown",
    "red",
    "orange",
    "yellow",
    "green",
    "blue",
    "violet",
    "grey",
    "white",
]


def label(colors):
    n = (codes.index(colors[0]) * 10 + codes.index(colors[1])) * (10 ** codes.index(colors[2]))
    if n > 1e9:
        return f"{n // 1e9:.0f} gigaohms"
    if n > 1e6:
        return f"{n // 1e6:.0f} megaohms"
    if n > 1e3:
        return f"{n // 1e3:.0f} kiloohms"
    return f"{n} ohms"

