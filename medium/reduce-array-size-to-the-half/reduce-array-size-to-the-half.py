import collections


class Solution:
    def minSetSize(self, arr: List[int]) -> int:
        c = collections.Counter(arr)
        half = (len(arr)+1) // 2
        acc = 0
        for i, n in enumerate(sorted(c.values(), reverse=True)):
            acc += n
            if acc >= half:
                return i + 1
