class Solution:
    def longestPalindrome(self, s: str) -> str:
        ans = ""
        reverse = s[::-1]
        _len = len(s)
        if _len == 0:
            return ans
        for l, char in enumerate(s):
            if len(ans) > _len - l:
                continue
            r = s.rfind(char)
            if r == -1:
                continue
            while r >= l and r - l >= len(ans):
                sub = s[l:r + 1]
                subR = reverse[_len - r - 1:_len - l]
                if (sub == subR and len(sub) > len(ans)):
                    ans = sub
                r = s.rfind(char, 0, r)
        return ans


if __name__ == '__main__':
    s = Solution()
    print(s.longestPalindrome("abc"))
