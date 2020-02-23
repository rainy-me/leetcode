class Solution:
    def addToArrayForm(self, A: List[int], K: int) -> List[int]:
        i = len(A) - 1
        carry = 0
        while K or carry:
            tmp = carry + K % 10
            if i >= 0:
                tmp += A[i]
            if tmp >= 10:
                tmp -= 10
                carry = 1
            else:
                carry = 0
            if i < 0:
                A = [tmp] + A
            else:
                A[i] = tmp
            K //= 10
            i -= 1
        return A
