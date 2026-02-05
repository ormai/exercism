from collections import Counter


def total(basket):
    counts = sorted(Counter(basket).values(), reverse=True)

    for i in range(len(counts) - 1):
        counts[i] -= counts[i + 1]

    if len(counts) >= 5:
        m = min(counts[2], counts[4])
        counts[2] -= m
        counts[4] -= m
        counts[3] += 2 * m

    discount = sum(map(lambda a: a[0] * a[1], zip(counts, [0, 10, 30, 80, 125])))

    return (len(basket) * 100 - discount) * 8
