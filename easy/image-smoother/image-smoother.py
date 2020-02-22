class Solution(object):
    def imageSmoother(self, M):
        m, n = len(M), len(M[0])

        def calc(i, j):
            s = [
                M[i + di][j + dj] & 0xff
                for di in (-1, 0, 1)
                for dj in (-1, 0, 1)
                if 0 <= i + di < m and 0 <= j + dj < n
            ]
            return sum(s)//len(s)
        for i in range(m):
            for j in range(n):
                M[i][j] = M[i][j] | calc(i, j) << 8
        for i in range(m):
            for j in range(n):
                M[i][j] = M[i][j] >> 8
        return M
