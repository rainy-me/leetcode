import collections


class Solution:

    def subarraySum(self, nums: List[int], k: int) -> int:
        count = total = 0
        cache = collections.defaultdict(int)
        cache[0] = 1
        for num in nums:
            total += num
            count += cache[total - k]
            cache[total] += 1
        return count
