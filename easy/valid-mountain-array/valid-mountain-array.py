class Solution:
    def validMountainArray(self, A: List[int]) -> bool:
        if len(A) < 3:
            return False
        status = 1
        last = A[0]
        start = 0
        for n in A[1:]:
            diff = n - last
            if diff == 0:
                return False
            if diff < 0 and status > 0:
                if not start:
                    return False
                status = -1
            if diff > 0 and status < 0:
                return False
            last = n
            if not start:
                start = 1

        if status > 0:
            return False
        return True
