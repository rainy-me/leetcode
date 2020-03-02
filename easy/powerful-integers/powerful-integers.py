# The guess API is already defined for you.
# @return -1 if my number is lower, 1 if my number is higher, otherwise return 0
# def guess(num: int) -> int:


class Solution:
    def guessNumber(self, n: int) -> int:
        l, r = 1, n
        while l <= r:
            mid = (l + r)//2
            res = guess(mid)
            if res == 0:
                return mid
            elif res == -1:
                r = mid - 1
            else:
                l = mid + 1
