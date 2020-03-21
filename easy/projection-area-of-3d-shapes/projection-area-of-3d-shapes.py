class Solution:
    def projectionArea(self, grid: List[List[int]]) -> int:
        front_to_back = sum(map(max, grid))
        right_to_left = sum(map(max, zip(*grid)))
        top_to_down = sum(v > 0 for row in grid for v in row)
        return front_to_back + right_to_left + top_to_down
