from typing import List


class Solution:
    def smallestRangeI(self, A: List[int], K: int) -> int:
        A.sort()
        _min, _max = A[0] + K, A[-1] - K
        if _min > _max:
            return 0
        else:
            return _max - _min
