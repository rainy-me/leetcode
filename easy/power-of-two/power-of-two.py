class Solution:
    def isPowerOfTwo(self, n: int) -> bool:
        if n < 1:
            return False
        while not n & 1:
            n //= 2
        return n == 1


class Solution2:
    def isPowerOfTwo(self, n: int) -> bool:
        return n > 0 and not (n & n-1)
