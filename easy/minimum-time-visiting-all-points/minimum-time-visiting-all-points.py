class Solution:
    def minTimeToVisitAllPoints(self, points):
        _sum = 0
        for (ax, ay), (bx, by) in zip(points, points[1:]):
            _sum += max(abs(ax - bx), abs(ay - by))
        return _sum
