from collections import Counter


class Solution:
    def canConstruct(self, ransomNote: str, magazine: str) -> bool:
        r = Counter(ransomNote)
        m = Counter(magazine)
        for k in r:
            if r[k] > m.get(k, 0):
                return False
        return True


class FasterSolution:
    def canConstruct(self, ransomNote: str, magazine: str) -> bool:
        return all(ransomNote.count(x) <= magazine.count(x) for x in set(ransomNote))
