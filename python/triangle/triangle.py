def is_valid(sides):
    a, b, c = sides
    return a + b >= c and b + c >= a and c + a >= b


def equilateral(sides):
    return is_valid(sides) and sides[0] != 0 and len(set(sides)) == 1


def isosceles(sides):
    return is_valid(sides) and len(set(sides)) <= 2


def scalene(sides):
    return is_valid(sides) and len(set(sides)) == 3
