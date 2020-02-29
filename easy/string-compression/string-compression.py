class Solution:
    def compress(self, chars: List[str]) -> int:
        char, count = "", 0
        ans = ""
        for c in chars:
            if c == char:
                count += 1
            else:
                if char:
                    ans += char
                if count > 1:
                    ans += str(count)
                count = 1
                char = c
        ans += char
        if count > 1:
            ans += str(count)
            ans = list(ans)
        chars[:] = ans
        return len(ans)
