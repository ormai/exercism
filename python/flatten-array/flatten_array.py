def flatten(iterable: list):
    flat = []
    for element in iterable:
        if isinstance(element, list):
            for elem in flatten(element):
                flat.append(elem)
        elif element is not None:
            flat.append(element)
    return flat
