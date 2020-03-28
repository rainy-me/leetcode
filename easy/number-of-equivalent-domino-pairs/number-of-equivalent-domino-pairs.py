import collections


class Solution:
    def numEquivDominoPairs(self, dominoes: List[List[int]]) -> int:
        c = collections.Counter([tuple(sorted(dominoe))
                                 for dominoe in dominoes])
        return sum([n*(n-1)//2 for n in c.values()])
