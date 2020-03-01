class Solution:
    def repeatedStringMatch(self, A: str, B: str) -> int:
        lowerBound = len(B)//len(A)
        for i in range(0, 3):
            if B in A*(lowerBound+i):
                return lowerBound+i
        return -1


# TODO: read this
class Solution2:
    def repeatedStringMatch(self, A: str, B: str) -> int:
        if set(B) - set(A):
            return -1
        if len(B) >= len(A):
            for i in range(len(B)//len(A), (len(B)//len(A)) + len(set(B))+1):
                if B in (A * i):
                    return i
 8        else:
            if B in A:
                return 1
            elif B in A+A:
                return 2
        return -1
