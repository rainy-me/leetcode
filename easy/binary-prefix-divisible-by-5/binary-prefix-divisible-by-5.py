class Solution:
    def prefixesDivBy5(self, A: List[int]) -> List[bool]:
        s = "".join(map(str, A))
        return [int(s[:i], 2) % 5 == 0 for i in range(1, len(A)+1)]


class Solution2:
    def prefixesDivBy5(self, A: List[int]) -> List[bool]:
        ans, b = [], 0
        for a in A:
            b = b << 1 | a
            ans.append(b % 5 == 0)
        return ans


class Solution3:
    def prefixesDivBy5(self, A):
        """
        @see https://leetcode.com/problems/binary-prefix-divisible-by-5/discuss/265509/Python-Calculate-Prefix-Mod/256604
        """
        for i in range(1, len(A)):
            A[i] += A[i - 1] * 2 % 5  # %5 to save space and calculation
        return [a % 5 == 0 for a in A]
