import itertools


class Solution:
    def pivotIndex(self, nums: List[int]) -> int:
        l = len(nums)
        if l == 1:
            return 0
        acc = [0, *list(itertools.accumulate(nums))]
        for i in range(1, l+1):
            if acc[i-1] == acc[-1] - acc[i]:
                return i - 1
        return -1
