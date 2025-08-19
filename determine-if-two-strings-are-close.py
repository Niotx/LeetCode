from collections import Counter

class Solution:
    def closeStrings(self, word1: str, word2: str) -> bool:
        return cons(word1, word2)


def cons(word1: str, word2: str) -> bool:
    # 1. Must be the same length
    if len(word1) != len(word2):
        return False

    # 2. Must have the same set of unique characters
    if set(word1) != set(word2):
        return False

    # 3. Must have the same frequency counts (ignoring which char)
    return sorted(Counter(word1).values()) == sorted(Counter(word2).values())
