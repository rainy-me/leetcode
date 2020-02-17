import collections


class Solution:
    def findShortestSubArray(self, nums: List[int]) -> int:
        dic = collections.Counter(nums)
        degree = max(dic.values())

        if degree <= 1:
            return 1
        num = [x for x in dic.keys() if dic[x] == degree]

        rst = float('inf')
        for i in num:
            p1, p2 = 0, len(nums)-1
            while nums[p1] != i:
                p1 += 1
            while nums[p2] != i:
                p2 -= 1
            rst = min(rst, p2-p1+1)

        return rst


if __name__ == '__main__':
    print(Solution().findShortestSubArray([1, 2, 2, 3, 1]))
    print(Solution().findShortestSubArray([1, 2, 2, 3, 1, 4, 2]))
