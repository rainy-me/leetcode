import math
import collections


class Solution:
    def hasGroupsSizeX(self, deck: List[int]) -> bool:
        c = collections.Counter(deck)
        _num = c[0]
        for k in c:
            _num = math.gcd(c[k], _num)
            if _num <= 1:
                return False
        return True
