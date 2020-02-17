class Solution:
    def __init__(self):
        self.cache = {2, 3, 5, 7, 11, 13, 17, 19}

    def check(self, n):
        if n in self.cache:
            return 1
        return 0

    def countPrimeSetBits(self, L, R):
        ans = 0
        for n in range(L, R+1):
            c = bin(n)[2:].count('1')
            if self.check(c):
                ans += 1
        return ans


if __name__ == '__main__':
    print(Solution().countPrimeSetBits(842, 888))
