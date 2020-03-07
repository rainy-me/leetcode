class Solution:
    def maxIncreaseKeepingSkyline(self, grid):
        N = len(grid)
        row_maxes = [max(row) for row in grid]
        col_maxes = [max([grid[i][j] for i in range(N)])for j in range(N)]
        return sum([min(row_maxes[i], col_maxes[j]) - grid[i][j] for i in range(N) for j in range(N)])
