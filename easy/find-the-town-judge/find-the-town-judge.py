class Solution:
    def findJudge(self, N: int, trust: List[List[int]]) -> int:
        arr = [0] * (N+1)
        m = set()
        for [a, b] in trust:
            m.add(a)
            arr[b] += 1
        for i, n in enumerate(arr):
            if i == 0:
                continue
            if n == N-1 and i not in m:
                return i
        return -1
