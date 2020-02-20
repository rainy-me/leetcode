from typing import List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        _sum = 0
        for i in range(1, len(prices)):
            _sum += max(0, prices[i] - prices[i-1])
        return _sum
