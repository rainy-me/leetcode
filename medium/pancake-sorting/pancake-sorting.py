class Solution:
    def pancakeSort(self, A: List[int]) -> List[int]:
        ans = []
        for val in range(len(A), 1, -1):
            index = A.index(val)
            ans += [index+1, val]
            A = A[:index:-1] + A[:index]
        return ans
