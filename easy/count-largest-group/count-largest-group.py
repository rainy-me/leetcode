import statistics


class Solution:
    @staticmethod
    def sum_digits(num):
        s = 0
        while num:
            s += num % 10
            num = num // 10
        return s

    def countLargestGroup(self, n: int) -> int:
        m = {}
        for i in range(1, n+1):
            s = self.sum_digits(i)
            m[s] = m.get(s, 0) + 1
        _max, _count = 0, 0
        for v in m.values():
            if v > _max:
                _max = v
                _count = 1
            elif v == _max:
                _count += 1
        return _count


class Solution2:
    def countLargestGroup(self, n: int) -> int:
        return len(statistics.multimode(sum(map(int, str(i))) for i in range(1, n+1)))
