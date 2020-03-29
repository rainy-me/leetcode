import heapq


class Solution:
    def maximumProduct(self, nums: List[int]) -> int:
        nums.sort()
        [a, b, *_] = nums
        [*_, c, d, e] = nums

        return max(a*b*e, c*d*e)


class Solution2:

    def maximumProduct(self, nums):
        nums.sort()
        return max(nums[-1] * nums[-2] * nums[-3], nums[0] * nums[1] * nums[-1])


class Solution3:
    def maximumProduct(self, nums):
        a, b = heapq.nlargest(3, nums), heapq.nsmallest(2, nums)
        return max(a[0] * a[1] * a[2], b[0] * b[1] * a[0])
