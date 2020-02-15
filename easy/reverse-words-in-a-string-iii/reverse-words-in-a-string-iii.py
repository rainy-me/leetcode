class Solution:
    def reverseWords(self, s: str) -> str:
        ret = []
        for w in s.split(" "):
            ret.append(w[::-1])
        return " ".join(ret)
