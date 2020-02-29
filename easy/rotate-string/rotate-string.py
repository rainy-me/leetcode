class Solution:
    def rotateString(self, A: str, B: str) -> bool:
        la, lb = len(A), len(B)
        if A == B:
            return True
        for n in range(la):
            if A == B[-n:] + B[:-n]:
                return True
        return False


class Solution2:
    def rotateString(self, A: str, B: str) -> bool:
        return len(A) == len(B) and B in A + A
