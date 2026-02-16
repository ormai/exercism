def roman(number: int) -> str:
    m = ''.join('M' for _ in range(number % 10_000 // 1000))
    c = arab_to_roman(number % 1000 // 100, 'C', 'D', 'M')
    x = arab_to_roman(number % 100 // 10, 'X', 'L', 'C')
    i = arab_to_roman(number % 10, 'I', 'V', 'X')
    return f"{m}{c}{x}{i}"


def arab_to_roman(digit: int, one: str, five: str, ten: str) -> str:
    match digit:
        case 0:
            return ""
        case n if 1 <= n <= 3:
            return "".join(one for _ in range(n))
        case 4:
            return f"{one}{five}"
        case 5:
            return five
        case n if 6 <= n <= 8:
            return f"{five}{''.join(one for _ in range(n - 5))}"
        case 9:
            return f"{one}{ten}"
        case _:
            raise ValueError(f"{n} must be between 0 and 9")
