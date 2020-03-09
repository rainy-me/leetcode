class Solution:
    def dailyTemperatures(self, T: List[int]) -> List[int]:
        """
        Time Limit Exceeded
        """
        l = len(T)
        ans = [0] * l
        counting = [0]*l
        pending = []
        for i, n in enumerate(T):
            for p in pending[:]:
                counting[p] += 1
                if n > T[p]:
                    ans[p] = counting[p]
                    pending.remove(p)
            pending.append(i)
        return ans


class Solution2:
    def dailyTemperatures(self, T: List[int]) -> List[int]:
        n, right_max = len(T), float('-inf')
        res = [0] * n
        for i in range(n-1, -1, -1):
            t = T[i]
            if right_max <= t:
                right_max = t
            else:
                temp = 1
                while T[i+temp] <= t:
                    temp += res[i+temp]
                res[i] = temp
        return res
