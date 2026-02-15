def at_bash(message: str) -> str:
    """
    Encodes or decodes a message with the Atbash cipher, which is symmetric.
    """
    return "".join(
        chr(ord("z") + ord("a") - ord(c)) if c.isalpha() else c
        for c in filter(str.isalnum, message)
    )


def space_at(msg: str, size: int) -> str:
    """Adds a space space after each chunk of given size in msg"""
    return " ".join(msg[i : i + size] for i in range(0, len(msg), size))


def encode(plain_text: str) -> str:
    """Encodes the plain text with the Atbash cipher."""
    return space_at(at_bash(plain_text.lower()), 5)


def decode(ciphered_text: str) -> str:
    """Decodes Atbash ecoded text."""
    return at_bash(ciphered_text)
