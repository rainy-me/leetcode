class Solution:
    def createTargetArray(self, nums: List[int], index: List[int]) -> List[int]:
        target = []
        for i, v in zip(index, nums):
            target = target[:i] + [v] + target[i:]
        return target


class Solution2:
    def createTargetArray(self, nums: List[int], index: List[int]) -> List[int]:
        target = []
        for i, v in zip(index, nums):
            target.insert(i, v)
        return target
