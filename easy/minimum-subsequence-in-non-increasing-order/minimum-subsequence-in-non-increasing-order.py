class Solution:
    def minSubsequence(self, nums: List[int]) -> List[int]:
        nums.sort(reverse=True)
        s1, s2 = 0, sum(nums)
        for i, num in enumerate(nums):
            s2 -= num
            s1 += num
            if s1 > s2:
                return nums[:i + 1]
