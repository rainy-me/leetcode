class Solution(object):
    def fairCandySwap(self, A, B):
        diff = (sum(A) - sum(B)) / 2
        set_a = set(A)
        for b in set(B):
            if b + diff in set_a:
                return [diff+b, b]


if __name__ == '__main__':
    print(Solution().fairCandySwap([1, 1], [2, 2]))
    print(Solution().fairCandySwap([1, 2], [2, 3]))
    print(Solution().fairCandySwap([2], [1, 3]))
    print(Solution().fairCandySwap([1, 2, 5], [2, 4]))
    print(Solution().fairCandySwap([2, 4], [1, 2, 5]))
