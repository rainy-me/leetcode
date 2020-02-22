class Solution:
    def convertToBase7(self, num: int) -> str:
        n, digits = abs(num), ""
        while n:
            digits = str(n % 7) + digits
            n //= 7
        return '-' * (num < 0) + digits or "0"
