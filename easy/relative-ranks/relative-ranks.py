from typing import List


class Solution:
    def findRelativeRanks(self, nums: List[int]) -> List[str]:

        sort = sorted(nums)[::-1]
        rank = [
            "Gold Medal",
            "Silver Medal",
            "Bronze Medal",
            *map(str, range(4, len(nums)+1))
        ]
        return list(map(dict(zip(sort, rank)).get, nums))
