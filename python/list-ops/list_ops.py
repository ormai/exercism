def append(list1, list2):
    return [*list1, *list2]


def concat(lists):
    return [element for lst in lists for element in lst]


def filter(function, lst):
    return [element for element in lst if function(element)]


def length(lst):
    return sum(1 for _ in lst)


def map(function, lst):
    return [function(element) for element in lst]


def foldl(function, lst, initial):
    acc = initial
    for element in lst:
        acc = function(acc, element)
    return acc


def foldr(function, lst, initial):
    acc = initial
    for element in reverse(lst):
        acc = function(acc, element)
    return acc


def reverse(lst):
    return lst[::-1]
