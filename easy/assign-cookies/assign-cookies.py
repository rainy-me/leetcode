from typing import List


class Solution:
    def findContentChildren(self, g: List[int], s: List[int]) -> int:
        g.sort()
        s.sort()
        i, j = 0, 0
        len_g, len_s = len(g), len(s)
        while i < len_g and j < len_s:
            if s[j] >= g[i]:
                i += 1
            j += 1
        return i
