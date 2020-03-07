import collections
import itertools


class Solution:
    def smallerNumbersThanCurrent(self, nums: List[int]) -> List[int]:
        c = collections.Counter(nums)
        ks = sorted(c)
        cache = {}
        for i, v in enumerate(itertools.accumulate([0]+[c[k] for k in ks][:-1])):
            cache[ks[i]] = v
        return [cache[x] for x in nums]
