class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        if not strs:
            return ""
        n, ans = len(strs[0]), ""
        for i in range(n):
            try:
                char = strs[0][i]
                for s in strs[1:]:
                    if s[i] != char:
                        return ans
                ans += char
            except:
                return ans
        return ans
