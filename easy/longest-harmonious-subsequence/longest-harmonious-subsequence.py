class Solution:
    def findLHS(self, nums: List[int]) -> int:
        m = {}
        for i in nums:
            if i not in m:
                m[i] = 1
            else:
                m[i] += 1
        ans = 0
        for i in m:
            if m.get(i+1):
                ans = max(ans, m[i]+m[i+1])
        return ans
