class Solution:
    def isPerfectSquare(self, num: int) -> bool:
        s = num ** .5
        return s == int(s)
