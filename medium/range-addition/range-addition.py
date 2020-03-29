class Solution:
    def toHex(self, num):
        return ''.join(
            '0123456789abcdef'[(num >> 4 * i) & 15]
            for i in range(8)
        )[::-1].lstrip('0') or '0'


class Solution2:
    def toHex(self, num):

        if num < 0:
            num += 2 ** 32

        stack = []
        s = "0123456789abcdef"

        while num:
            stack.append(s[num % 16])
            num //= 16

        if not stack:
            return "0"

        stack.reverse()
        return "".join(stack)
