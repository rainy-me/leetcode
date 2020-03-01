class Solution:
    def checkPossibility(self, nums: List[int]) -> bool:
        drop_after, drop_before = nums[:], nums[:]
        for i in range(len(nums) - 1):
            if nums[i] > nums[i + 1]:
                drop_after[i] = nums[i + 1]
                drop_before[i + 1] = nums[i]
                break
        return drop_after == sorted(drop_after) or drop_before == sorted(drop_before)
