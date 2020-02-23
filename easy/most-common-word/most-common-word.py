import re


class Solution:
    def mostCommonWord(self, paragraph: str, banned: List[str]) -> str:
        m = {}
        c = 0
        ans = ""
        b = set(banned)
        for word in re.findall('[a-zA-Z]+', paragraph):
            word = word.lower()
            if word in b:
                continue
            if word in m:
                m[word] += 1
            else:
                m[word] = 1
            if m[word] > c:
                ans = word
        return ans
