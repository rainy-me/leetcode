class Solution:
    def minimumAbsDifference(self, arr):
        arr.sort()
        _min = arr[1] - arr[0]
        ans = [arr[0], arr[1]]
        for i in range(2, len(arr)):
            pair = [arr[i-1], arr[i]]
            diff = arr[i] - arr[i-1]
            if diff < _min:
                ans = [pair]
                _min = diff
                continue
            if diff == _min:
                ans.append(pair)
        return ans


if __name__ == '__main__':
    s = Solution().minimumAbsDifference([3, 8, -10, 23, 19, -4, -14, 27])
    print(s)
