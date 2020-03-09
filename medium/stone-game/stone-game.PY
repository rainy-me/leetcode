class Solution:
    def stoneGame(self, piles: List[int]) -> bool:
        return True


class Solution2:
    def stoneGame(self, piles: List[int]) -> bool:
        """
        TODO: DP
        """
        n = len(piles)
        dp = piles[:]
        for d in range(1, n):
            for i in range(n - d):
                dp[i] = max(piles[i] - dp[i + 1], piles[i + d] - dp[i])
        return dp[0] > 0
