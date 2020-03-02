import bisect


class Solution:
    def searchInsert(self, nums: List[int], target: int) -> int:
        if target < nums[0]:
            return 0
        elif target > nums[-1]:
            return len(nums) - 1
        for i, n in enumerate(nums):
            if n == target:
                return i
            elif n > target:
                return i


class Solution2:
    searchInsert = bisect.bisect_left
