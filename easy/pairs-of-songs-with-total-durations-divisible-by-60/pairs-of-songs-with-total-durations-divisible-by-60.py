import collections


class Solution:
    def numPairsDivisibleBy60(self, time: List[int]) -> int:
        c = collections.Counter([n % 60 for n in time])
        ans = 0
        for n in c:
            v = c[n]
            if n > 30:
                continue
            if n == 30 or n == 0:
                ans += v * (v-1) // 2
            else:
                ans += c.get(60-n, 0) * v
        return ans


class Solution2:
    def numPairsDivisibleBy60(self, time):
        c = [0] * 60
        res = 0
        for t in time:
            res += c[-t % 60]
            c[t % 60] += 1
        return res
