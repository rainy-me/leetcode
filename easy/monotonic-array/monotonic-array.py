class Solution:
    def isMonotonic(self, A: List[int]) -> bool:
        delta_all = 1
        t = 0
        for i in range(1, len(A)):
            delta = A[i] - A[i-1]
            if delta:
                if not t:
                    delta_all = delta
                    t = 1
            if delta_all * delta < 0:
                return False
        return True


class Solution2:
    def isMonotonic(self, A):
        return not {(i > j)-(i < j) for i, j in zip(A, A[1:])} >= {1, -1}
