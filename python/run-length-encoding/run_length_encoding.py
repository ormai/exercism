def decode(string: str) -> str:
    """Decodes the given string encoded with Run-Length Encoding."""

    decoded = ""
    i = 0
    while i < len(string):
        num = 0
        while string[i].isnumeric():
            num = num * 10 + int(string[i])
            i += 1
        decoded += "".join(string[i] for _ in range(num)) if num else string[i]
        i += 1
    return decoded


def encode(string: str) -> str:
    """Encodes the given string with Run-Length Encoding."""

    if len(string) < 2:
        return string

    encoded = ""
    start = 0
    for i in range(len(string) - 1):
        if string[i] != string[i + 1]:
            encoded += f"{i - start + 1 if i - start > 0 else ''}{string[i]}"
            start = i + 1
    encoded += f"{len(string) - start if len(string) - start > 1 else ''}{string[-1]}"
    return encoded
