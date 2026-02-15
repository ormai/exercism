"""
I know that int.bit_count() exists, but what would be the point?
"""


def egg_count(display_value: int) -> int:
    """Computes the 'popcount' or 'bitcount' of the given integer"""

    count = 0
    bit = 1
    while bit < 2 << 30:
        if display_value & bit != 0:
            count += 1
        bit <<= 1
    return count
