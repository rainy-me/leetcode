class Solution:
    def printVertically(self, s: str) -> List[str]:
        ans = []
        for i, w in enumerate(s.split()):
            for j, char in enumerate(w):
                if j >= len(ans):
                    ans.append(" "*i + char)
                else:
                    ans[j] += " "*(i - len(ans[j])) + char
        return ans


class Solution2:
    def printVertically(self, s):
        return [''.join(a).rstrip() for a in itertools.zip_longest(*s.split(), fillvalue=' ')]
