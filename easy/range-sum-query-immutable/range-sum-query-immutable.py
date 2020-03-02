import itertools


class NumArray:
    def __init__(self, nums: List[int]):
        self.acc = [0, *itertools.accumulate(nums)]

    def sumRange(self, i: int, j: int) -> int:
        return self.acc[j+1] - self.acc[i]
