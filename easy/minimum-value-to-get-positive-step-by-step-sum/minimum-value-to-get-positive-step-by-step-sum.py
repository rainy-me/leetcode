from itertools import accumulate


class Solution:
    def minStartValue(self, nums: List[int]) -> int:
        startVal, s = 1, 0
        for n in nums:
            diff = n + s
            if diff < 0:
                startVal -= diff
                s -= diff
            s += n
        return startVal


class Solution2:
    def minStartValue(self, nums: List[int]) -> int:
        return max(1, -min(accumulate(nums)) + 1)
