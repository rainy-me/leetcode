class Solution(object):
    def getSum(self, a, b):
        _max = 0x7fffffff
        mask = 0xffffffff
        while b != 0:
            a, b = (a ^ b) & mask, ((a & b) << 1) & mask
        return a if a <= _max else ~(a ^ mask)
