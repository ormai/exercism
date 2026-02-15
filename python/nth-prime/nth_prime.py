def prime(number: int) -> int:
    """
    Generates the nth prime.

    Adapted from: https://stackoverflow.com/a/25706914/15481080
    """

    if not number:
        raise ValueError("there is no zeroth prime")

    n = 0
    d = {}
    q = 2
    while True:
        if q not in d:
            n += 1
            if n == number:
                return q
            d[q * q] = [q]
        else:
            for p in d[q]:
                d.setdefault(p + q, []).append(p)
            del d[q]
        q += 1
