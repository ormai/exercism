
def is_anamgram(word, candidate):
    word = list(word.lower())
    candidate = list(candidate.lower())
    if word == candidate:
        return False
    for c in word:
        if c in candidate:
            candidate.remove(c)
        else:
            return False
    return len(candidate) == 0


def find_anagrams(word, candidates):
    return list(filter(lambda candidate: is_anamgram(word, candidate) , candidates))
