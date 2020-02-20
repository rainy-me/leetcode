class Solution:
    def __init__(self):
        self.m = {1: 1, 2: 2}

    def climbStairs(self, n):
        if n not in self.m:
            self.m[n] = self.climbStairs(n-1) + self.climbStairs(n-2)
        return self.m[n]
