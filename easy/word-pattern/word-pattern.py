class Solution:
    def wordPattern(self, pattern: str, str: str) -> bool:
        words = str.split(' ')
        p = list(pattern)
        if len(words) != len(p):
            return False
        m = {}
        for i, word in enumerate(words):
            if word in m:
                if m[word] != p[i]:
                    return False
            else:
                if p[i] in m.values():
                    return False
                else:
                    m[word] = p[i]
        return True
