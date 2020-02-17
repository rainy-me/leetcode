from typing import List


class Solution:
    def isAlienSorted(self, words: List[str], order: str) -> bool:
        m = dict([[char, i] for i, char in enumerate(order)])
        for i in range(1, len(words)+1):
            if m[words[i-1][0]] >= m[words[i][0]]:
                return False
        return True
