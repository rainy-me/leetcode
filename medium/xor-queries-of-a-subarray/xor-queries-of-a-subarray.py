from operator import xor
from functools import reduce


class Solution:
    def xorQueries(self, arr: List[int], queries: List[List[int]]) -> List[int]:
        cache = {}

        def calc(a, b):
            if a == b:
                return arr[a]
            key = f"{a}-{b}"
            if key in cache:
                return cache[key]
            val = reduce(xor, arr[a:b+1])
            cache[key] = val
            return val
        return [calc(a, b) for [a, b] in queries]


class Solution2:
    def xorQueries(self, A, queries):
        for i in range(len(A) - 1):
            A[i + 1] ^= A[i]
        return [A[j] ^ A[i - 1] if i else A[j] for i, j in queries]
