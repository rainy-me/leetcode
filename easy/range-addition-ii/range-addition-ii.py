class Solution:
    def maxCount(self, m: int, n: int, ops: List[List[int]]) -> int:
        _max_count = m*n
        _min_m = m
        _min_n = n
        for ([a, b]) in ops:
            if a * b:
                _min_m = min(_min_m, a)
                _min_n = min(_min_n, b)
                _max_count = _min_m * _min_n
        return _max_count


class Solution2:
    def maxCount(self, m: int, n: int, ops: List[List[int]]) -> int:
        if not ops or not ops[0]:
            return m * n
        return min(list(zip(*ops))[0]) * min(list(zip(*ops))[1])


class Solution3:
    def maxCount(self, m: int, n: int, ops: List[List[int]]) -> int:
        """
        timeout...
        """
        M = [[0 for _ in range(n)] for _ in range(m)]
        _max_n = 0
        _max_count = m*n

        for ([a, b]) in ops:
            _count = 0
            for i in range(m):
                for j in range(n):
                    if 0 <= i < a and 0 <= j < b:
                        v = M[i][j] + 1
                        M[i][j] = v
                        if v > _max_n:
                            _max_n = v
                            _count = 1
                        elif _max_n == v:
                            _count += 1
            _max_count = _count
        return _max_count
