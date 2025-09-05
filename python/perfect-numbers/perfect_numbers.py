def classify(number):
    """A perfect number equals the sum of its positive divisors.

    :param number: int a positive integer
    :return: str the classification of the input integer
    """
    if number <= 0:
        raise ValueError("Classification is only possible for positive integers.")

    s = sum(f if number % f == 0 else 0 for f in range(1, number))

    return "perfect" if s == number else "abundant" if number < s else "deficient"
