class Solution:
    def groupThePeople(self, groupSizes: List[int]) -> List[List[int]]:
        g = {}
        ans = []
        for i, size in enumerate(groupSizes):
            if size in g:
                g[size].append(i)
            else:
                g[size] = [i]
            if len(g[size]) == size:
                ans.append(g[size])
                g[size] = []

        return ans
