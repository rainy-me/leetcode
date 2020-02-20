from typing import List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        h = 0
        gap = 0
        for price in prices[::-1]:
            h = max(price, h)
            gap = max(h - price, gap)
        return gap
