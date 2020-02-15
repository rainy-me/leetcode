class Solution:
    def sumEvenAfterQueries(self, A, queries):
        R, S = [], sum([i for i in A if not (i & 1)])
        for [v, i] in queries:
            a = A[i]
            A[i] += v
            if not (a & 1):
                if not (v & 1):
                    S += v
                else:
                    S -= a
            elif v & 1:
                S += A[i]

            R.append(S)
        return R


if __name__ == '__main__':
    print(Solution().sumEvenAfterQueries(
        [1, 2, 3, 4], [[1, 0], [-3, 1], [-4, 0], [2, 3]]))
