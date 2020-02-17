class Solution(object):
    def reverseOnlyLetters(self, S):
        ans = ""
        R = [*S]
        for k in S:
            if k.isalpha():
                print(k)
                c = R.pop()
                while not c.isalpha():
                    c = R.pop()
                ans += c
            else:
                ans += k
        return ans


if __name__ == '__main__':
    print(Solution().reverseOnlyLetters('7_28]'))
    print(Solution().reverseOnlyLetters('a=b=c=d'))
