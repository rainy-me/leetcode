import collections


class Solution:
    def sortString(self, s: str) -> str:
        c = collections.Counter(s)
        chars = sorted(set(s))
        ans = []
        i = 0
        while len(ans) < len(s):
            if i & 1:
                ans += chars[::-1]
            else:
                ans += chars
            for k in [*c.keys()]:
                if c[k] == 1:
                    chars.remove(k)
                    del c[k]
                else:
                    c[k] -= 1
            i += 1
        return "".join(ans)
