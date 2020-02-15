class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        return sorted(nums)[len(nums)//2]

    def majorityElement2(self, nums: List[int]) -> int:
        m = {}
        half = len(nums)/2
        for num in nums:
            count = m.get(num, 0) + 1
            if count > half:
                return num
            m[num] = count
