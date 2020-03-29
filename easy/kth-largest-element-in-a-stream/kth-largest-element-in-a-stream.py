import heapq
import bisect


class KthLargest:

    def __init__(self, k: int, nums: List[int]):
        self.k = k
        self.nums = sorted(nums)

    def add(self, val: int) -> int:
        bisect.insort(self.nums, val)
        return self.nums[-self.k]

        # Your KthLargest object will be instantiated and called as such:
        # obj = KthLargest(k, nums)
        # param_1 = obj.add(val)


class KthLargest2:

    def __init__(self, k, nums):
        self.nums = nums
        self.k = k
        heapq.heapify(self.nums)
        while len(self.nums) > k:
            heapq.heappop(self.nums)

    def add(self, val):
        if len(self.nums) < self.k:
            heapq.heappush(self.nums, val)
        else:
            heapq.heappushpop(self.nums, val)
        return self.nums[0]
