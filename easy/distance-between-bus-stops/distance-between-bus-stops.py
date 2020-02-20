from typing import List


class Solution:
    def distanceBetweenBusStops(self, distance: List[int], start: int, destination: int) -> int:
        if start > destination:
            start, destination = destination, start
        route_a = 0
        route_b = 0
        i = start
        loop_len = len(distance)
        while destination != i:
            route_a += distance[i]
            i += 1

        while i - start != loop_len:
            route_b += distance[i]
            i += 1
        return min(route_a, route_b)


if __name__ == '__main__':
    print(Solution().distanceBetweenBusStops([1, 2, 3, 4], 0, 3))
    print(Solution().distanceBetweenBusStops(
        [7, 10, 1, 12, 11, 14, 5, 0], 7, 2))
