class Solution:
    def findDuplicates(self, nums: List[int]) -> List[int]:
        seen = set()
        ans = []
        for n in nums:
            if n in seen:
                ans.append(n)
            else:
                seen.add(n)
        return ans
