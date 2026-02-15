"""
Could easily be done with set, but I went with list for this exercise.
"""

SUBLIST = 0
SUPERLIST = 1
EQUAL = 2
UNEQUAL = 3


def sublist(list_one: list, list_two: list) -> int:
    """Compares two list and returns an enumerative constant as result."""

    if list_one == list_two:
        return EQUAL

    len_one, len_two = len(list_one), len(list_two)

    if len_one < len_two and any(list_two[i: i + len_one] == list_one for i in range(len_two)):
        return SUBLIST

    if len_one > len_two and any(list_one[i: i + len_two] == list_two for i in range(len_one)):
        return SUPERLIST

    return UNEQUAL
