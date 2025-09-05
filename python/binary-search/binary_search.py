def find(search_list, value):
    lo, hi = 0, len(search_list)
    while lo < hi:
        mid = (lo + hi) // 2
        if search_list[mid] > value:
            hi = mid
        elif search_list[mid] < value:
            lo = mid + 1
        else:
            return mid
    raise ValueError("value not in array")
