from itertools import combinations
from math import sqrt


class Solution:
    def distance(self, p1, p2):
        return sqrt((p1[0]-p2[0])**2+(p1[1]-p2[1])**2)

    def heron(self, p1, p2, p3):
        a = self.distance(p1, p2)
        b = self.distance(p1, p3)
        c = self.distance(p2, p3)
        s = (a + b + c)/2
        res = s*(s-a)*(s-b)*(s-c)
        return 0 if res < 0 else sqrt(res)

    def largestTriangleArea(self, points: List[List[int]]) -> float:
        return max(self.heron(i, j, k) for i, j, k in combinations(points, 3))
