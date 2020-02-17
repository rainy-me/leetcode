class Solution:
    def gcdOfStrings(self, str1: str, str2: str) -> str:
        l1, l2 = len(str1), len(str2)
        if l1 == l2:
            return str1 if str1 == str2 else ""
        if l1 < l2:
            str1, str2 = str2, str1
            l2 = len(str2)
        if str1[:l2] == str2:
            return self.gcdOfStrings(str1[l2:], str2)
        else:
            return ""


if __name__ == '__main__':
    print(Solution().gcdOfStrings("ABABAB", "ABAB"))
