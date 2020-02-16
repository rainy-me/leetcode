class Solution:
    def convert(self, s: str, numRows: int) -> str:
        if numRows == 1:
            return s
        rows = ["" for i in range(numRows)]
        loopLen = 2 * numRows - 2
        for i, char in enumerate(s):
            pos = i % loopLen
            if pos >= numRows:
                pos = loopLen - pos
            print(pos, char)
            rows[pos] += char
        return "".join(rows)


if __name__ == '__main__':
    print(Solution().convert("A", 1))\\
