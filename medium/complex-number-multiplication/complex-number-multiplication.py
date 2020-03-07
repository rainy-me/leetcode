class Solution:
    @staticmethod
    def parse(num: str):
        [c, i] = num.split('+')
        return [*map(int, [c, i[:-1]])]

    def complexNumberMultiply(self, a: str, b: str) -> str:
        [ai, aj] = self.parse(a)
        [bi, bj] = self.parse(b)
        ci = ai*bi - aj*bj
        cj = ai*bj + bi*aj
        return f'{ci}+{cj}i'
