class Solution:
    def maxDistToClosest(self, seats: List[int]) -> int:
        space, count = 0, 0
        start, start_count = 0, 0
        end_count = 0
        for i, seat in enumerate(seats):
            if seat:
                space = max(space, count)
                count = 0
                start = 0
            else:
                if i == 0:
                    start = 1
                if start:
                    start_count += 1
                else:
                    count += 1
        if count:
            end_count = count
        mid = space // 2 + 1 if space & 1 else space // 2
        return max(start_count, mid, end_count)
