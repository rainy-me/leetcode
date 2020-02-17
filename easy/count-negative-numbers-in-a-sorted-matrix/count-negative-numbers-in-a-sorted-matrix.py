class Solution:
    def countNegatives(self, grid):
        ans, n, m = 0, len(grid), len(grid[0])
        for nn in range(n):
            for mm in range(m)[::-1]:
                if grid[nn][mm] >= 0:
                    break
                ans += 1
        return ans


if __name__ == "__main__":
    print(Solution().countNegatives(
        [[4, 3, 2, -1], [3, 2, 1, -1], [1, 1, -1, -2], [-1, -1, -2, -3]]))
