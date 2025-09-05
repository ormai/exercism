def rotate(text, key):
    def rot(c):
        if c.isalpha():
            nc = ord(c) + key
            if c.isupper() and nc > ord("Z") or nc > ord("z"):
                nc -= 26
            return chr(nc)
        return c
    return "".join(map(rot, list(text)))
