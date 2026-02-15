def square_root(number: int) -> int:
    low = 0
    while (low + 1) * (low + 1) <= number:
        low += 1
    return low
