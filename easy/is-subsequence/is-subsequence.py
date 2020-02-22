class Solution:
    def isSubsequence(self, s: str, t: str) -> bool:
        index = 0
        for c in s:
            i = t[index:].find(c)
            if i == -1:
                return False
            index += i+1
        return True
