class Solution:
    def reverseWords(self, s: str) -> str:
        ret = []
        for word in s.split(" "):
            if word:
                ret.insert(0, word)
        return " ".join(word)
