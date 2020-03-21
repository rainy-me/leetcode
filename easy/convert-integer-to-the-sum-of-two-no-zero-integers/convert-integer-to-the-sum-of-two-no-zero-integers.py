class Solution:
    def getNoZeroIntegers(self, n: int) -> List[int]:
        return next([a, n-a] for a in range(n) if '0' not in f'{a}{n-a}')


class Solution2:
    def getNoZeroIntegers(self, n: int) -> List[int]:
        a, b = 0, 0

        rest = n % 10
        power = 1

        while n > 1:
            n = n // 10
            if rest > 1:
                a += 1*power
                b += (rest - 1)*power
            else:
                a += (rest+1)*power
                b += 9*power
                n -= 1
            rest = n % 10
            power *= 10

        if n == 1:
            a += power

        return [a, b]
