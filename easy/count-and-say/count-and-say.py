class Solution:

    def get(self, n: str):
        tmp = ""
        curr = n[0]
        count = 0
        for c in n:
            if c != curr:
                tmp += f"{count}{curr}"
                curr = c
                count = 1
            else:
                count += 1
        tmp += f"{count}{curr}"
        return tmp

    def countAndSay(self, n: int) -> str:
        ans = "1"
        for _ in range(n-1):
            ans = self.get(ans)
        return ans
