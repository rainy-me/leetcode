class Solution:
    def repeatedSubstringPattern(self, s: str) -> bool:
        return any(s == s[k:] + s[:k] for k in range(1, len(s)//2 + 1))


class Solution2:
    def repeatedSubstringPattern(self, s: str) -> bool:
        return s in (s + s)[1:-1]
