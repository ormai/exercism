POWER_OF_TENS = [
    (1e18, "quintillion"),
    (1e15, "quadrillion"),
    (1e12, "trillion"),
    (1e9, "billion"),
    (1e6, "million"),
    (1e3, "thousand"),
    (1e2, "hundred"),
]

TENS = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
]

UNITS = [
    "",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
]


def say(number: int) -> str:
    """Converts the given number in English words."""

    if number < 0 or number > 999_999_999_999:
        raise ValueError("input out of range")

    if number == 0:
        return "zero"
    return power_of_ten(number).rstrip("- ")


def power_of_ten(n: int) -> str:
    """Converts the largest power of ten of the provided integer recursively."""
    for p, name in POWER_OF_TENS:
        div, mod = divmod(n, p)
        if n // p != 0:
            return f"{power_of_ten(div)} {name} {power_of_ten(mod)}"
    if 1 <= n < 10:
        return UNITS[int(n)]
    if 10 <= n < 20:
        return TENS[n % 10]
    return f"{TENS[int(8 + n // 10)] if n // 10 else ''}-{UNITS[int(n % 10)]}"
