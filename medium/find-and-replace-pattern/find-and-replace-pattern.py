from typing import List


class Solution:
    def __init__(self):
        self.pattern = ''
        self.pattern_len = 0

    def compare(self, word: str) -> bool:
        if len(word) != self.pattern_len:
            return False
        m = {}
        for i, char in enumerate(word):
            if char in m:
                if m[char] != self.pattern[i]:
                    return False
            else:
                if self.pattern[i] in m.values():
                    return False
                else:
                    m[char] = self.pattern[i]

        return True

    def findAndReplacePattern(self, words: List[str], pattern: str) -> List[str]:
        self.pattern = pattern
        self.pattern_len = len(pattern)
        return [*filter(self.compare, words)]


class Solution2:
    def findAndReplacePattern(self, words: List[str], pattern: str) -> List[str]:
        def translate(pattern):
            trans = {}
            return tuple(trans.setdefault(c, len(trans)) for c in pattern)

        to_match = translate(pattern)
        return [word for word in words if to_match == translate(word)]
