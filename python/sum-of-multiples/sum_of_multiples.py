def sum_of_multiples(limit: int, multiples: list[int]) -> int:
    return sum(
        {
            n
            for multiple in filter(lambda m: m > 0, multiples)
            for n in range(multiple, limit, multiple)
        }
    )
