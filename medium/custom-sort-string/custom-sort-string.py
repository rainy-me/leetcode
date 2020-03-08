class Solution:
    def customSortString(self, S: str, T: str) -> str:
        m = {}
        for i, c in enumerate(S):
            m[c] = i
        return "".join(sorted(T, key=lambda n: m.get(n, 100)))


class Solution2:
    def customSortString(self, S: str, T: str) -> str:
        ans = ""
        from collections import Counter
        T = Counter(T)
        for c in S:
            if c in T:
                ans = ans+c*T[c]
        for k, v in T.items():
            if k not in S:
                ans += k*v
        return ans
