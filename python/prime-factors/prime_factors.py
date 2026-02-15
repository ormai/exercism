def factors(value: int) -> list[int]:
    """Finds the prime factors of the given integer."""

    prime_factors = []
    div = 2
    while value > 1:
        if value % div == 0:
            prime_factors.append(div)
            value //= div
        else:
            div += 1
    return prime_factors
