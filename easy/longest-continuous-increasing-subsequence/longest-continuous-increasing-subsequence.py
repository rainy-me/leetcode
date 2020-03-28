class Solution:
    def findLengthOfLCIS(self, nums: List[int]) -> int:
        ans = 0
        count = 0
        last = 0
        for n in nums:
            if n > last:
                last = n
                count += 1
            else:
                ans = max(count, ans)
                last = n
                count = 1
        ans = max(count, ans)

        return ans
