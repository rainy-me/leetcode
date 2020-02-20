class Solution:
    def convertToTitle(self, n: int) -> str:
        ans = ""
        while n > 0:
            y = (n-1) % 26
            n = (n-1) // 26
            s = chr(y+65)
            ans = s + ans
        return ans
