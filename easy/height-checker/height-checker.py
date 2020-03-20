class Solution:
    def heightChecker(self, heights: List[int]) -> int:
        return sum(map(operator.ne, heights, sorted(heights)))
