import re


def translate(text: str) -> str:
    words = text.split()
    for i, word in enumerate(words):
        if m := re.match(r"[aeiou]|xr|yt", word):
            words[i] += "ay"
        elif m := re.match(r"[^aeiou]*qu", word):
            words[i] = word[m.end(0) :] + m[0] + "ay"
        elif m := re.match(r"([^aeiou]+)y", word):
            words[i] = word[m.end(0) - 1 :] + m[0][:-1] + "ay"
        elif m := re.match(r"[^aeiou]+", word):
            words[i] = word[m.end(0) :] + m[0] + "ay"
    return " ".join(words)
