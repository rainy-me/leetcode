import collections


class Solution:
    def minSteps(self, s: str, t: str) -> int:
        counter_s = collections.Counter(s)
        counter_t = collections.Counter(t)
        diff = (counter_s | counter_t) - (counter_s & counter_t)
        return sum(diff.values())//2
