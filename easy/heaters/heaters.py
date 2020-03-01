class Solution:
    def findRadius(self, houses: List[int], heaters: List[int]) -> int:
        houses.sort()
        heaters.sort()
        heaters_number, i, max_radius = len(heaters), 0, 0
        for house in houses:
            while i+1 < heaters_number and heaters[i+1] < house:
                i += 1
            max_radius = max(max_radius, min(
                [abs(heat - house) for heat in heaters[i:i+2]]))
        return max_radius
