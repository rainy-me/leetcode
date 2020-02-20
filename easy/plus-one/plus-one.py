from typing import List


class Solution:
    def plusOne(self, digits: List[int]) -> List[int]:
        carry = 1
        ans = []
        for digit in digits[::-1]:
            _s = digit+carry
            if _s == 10:
                carry = 1
                ans.append(0)
            else:
                ans.append(_s)
                carry = 0
        if carry:
            ans.append(1)
        return ans[::-1]
