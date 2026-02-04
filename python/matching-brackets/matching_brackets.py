def is_paired(input_string):
    pairs = {"]": "[", "}": "{", ")": "("}
    stack = []
    for c in input_string:
        if c in ("[", "{", "("):
            stack.append(c)
        if c in ("]", "}", ")") and (not stack or pairs[c] != stack.pop()):
            return False
    return not stack
