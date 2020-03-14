import collections


class Solution:
    def singleNumber(self, nums: List[int]) -> List[int]:
        c = collections.Counter(nums)
        return [x for x in c if c[x] == 1]
