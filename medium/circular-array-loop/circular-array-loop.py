class Solution:
    def circularArrayLoop(self, nums: List[int]) -> bool:
        l = len(nums)
        for i, n in enumerate(nums):
            d = 1 if n > 0 else -1
            visited = set()
            visited.add(i)
            step = n
            last_i = i
            while len(visited) <= l:
                index = (last_i+step) % l
                step = nums[index]
                if index in visited:
                    if len(visited) > 1 and index == i:
                        return True
                    else:
                        break
                visited.add(index)
                last_i = index
                if step * d < 0:
                    break
        return False


class Solution2:
    def circularArrayLoop(self, nums: List[int]) -> bool:
        N = len(nums)
        for i in range(len(nums)):
            if type(nums[i]) != int:
                continue

            if nums[i] % N == 0:
                continue

            direction = nums[i] > 0
            mark = str(i)
            while type(nums[i]) == int and direction == (nums[i] > 0) and nums[i] % N != 0:
                jump = nums[i]
                nums[i] = mark
                i = (i + jump) % N
            if nums[i] == mark:
                return True
        return False
