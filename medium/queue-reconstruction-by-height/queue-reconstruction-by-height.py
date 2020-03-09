class Solution:

    def reconstructQueue(self, people):
        people.sort(key=lambda p: (-p[0], p[1]))
        ans = []
        for p in people:
            ans.insert(p[1], p)
        return ans
