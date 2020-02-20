class Solution:

    def mySqrt(self, x):
        l, r = 0, x

        while l <= r:
            mid = (l + r) // 2

            if mid * mid > x:
                r = mid - 1
            elif mid * mid < x:
                l = mid + 1
            else:
                return mid
        return r
