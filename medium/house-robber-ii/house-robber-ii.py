class Solution:

    def rob_seq(self, nums):

        last, now = 0, 0

        for i in nums:
            last, now = now, max(last + i, now)

        return now

    def rob(self, nums: List[int]) -> int:
        l = len(nums)
        if l == 0:
            return 0
        elif l == 1:
            return nums[0]
        return max(self.rob_seq(nums[1:]), self.rob_seq(nums[:-1]))
