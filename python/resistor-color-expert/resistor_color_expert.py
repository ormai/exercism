from functools import reduce

COLOR_BANDS = [
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


TOLERANCE = {
    "grey": 0.05,
    "violet": 0.1,
    "blue": 0.25,
    "green": 0.5,
    "brown": 1,
    "red": 2,
    "gold": 5,
    "silver": 10,
}


def label(colors):
    n = reduce(lambda acc, el: acc * 10 + COLOR_BANDS.index(el), colors[:-1], 0) * (
        10 ** COLOR_BANDS.index(colors[-1])
    )
    if n > 1e6:
        return f"{n / 1e6} megaohms"
    if n >= 1e3:
        n /= 1e3
        if n - int(n) == 0:
            n = int(n)
        return f"{n} kiloohms"
    return f"{n} ohms"


def resistor_label(colors):
    if len(colors) == 1:
        return "0 ohms"
    return f"{label(colors[:-1])} ±{TOLERANCE[colors[-1]]}%"
