class Solution:
    def reformat(self, s: str) -> str:
        a, n, ans = [], [], ""
        for c in s:
            if c.isalpha():
                a.append(c)
            else:
                n.append(c)
        ln, la = len(n), len(a)
        if abs(ln-la) > 1:
            return ans
        if la > ln:
            n, a = a, n
            la, ln = ln, la
        for i in range(ln):
            ans += n[i]
            if i < la:
                ans += a[i]
        return ans
