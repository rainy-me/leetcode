class Solution:
    def rotatedDigits(self, N: int) -> int:
        m = {"0", "1", "8", "2", "5", "6", "9"}
        ans = 0
        for n in range(1, N):
            _ok = True
            s = str(n)
            _len = len(s)
            tmpFor018 = 0
            for char in s:
                if char == '0' or char == '1' or char == '8':
                    tmpFor018 += 1
                if char not in m:
                    _ok = False
                    continue
            if tmpFor018 == _len:
                _ok = False
            if _ok:
                ans += 1
        return ans
