class Solution:
    def generate(self, numRows: int) -> List[List[int]]:
        if numRows == 0:
            return []
        ans = [[1]]
        row = []

        def next():
            nonlocal row
            tmp = []
            for i in range(len(row)-1):
                tmp.append(row[i]+row[i+1])
            row = [1, *tmp, 1]

        for _ in range(numRows-1):
            next()
            ans.append(row)

        return ans


class Solution2:

    def generate(self, numRows):
        pascal = [[1]*(i+1) for i in range(numRows)]
        for i in range(numRows):
            for j in range(1, i):
                pascal[i][j] = pascal[i-1][j-1] + pascal[i-1][j]
        return pascal


class Solution3:

    def generate(self, numRows):
        res = [[1]]
        for _ in range(1, numRows):
            res += [map(lambda x, y: x+y, res[-1] + [0], [0] + res[-1])]
        return res[:numRows]
