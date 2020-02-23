class Solution:
    def isIsomorphic(self, s: str, t: str) -> bool:
        m = {}
        used = set({})
        for i, c in enumerate(s):
            if c in m:
                if m[c] != t[i]:
                    return False
            else:
                if t[i] in used:
                    return False
                m[c] = t[i]
                used.add(t[i])
        return True
