class Solution:
    def canThreePartsEqualSum(self, A: List[int]) -> bool:
        e = sum(A) / 3
        tmp = 0
        count = 0
        for i, n in enumerate(A):
            tmp += n
            if tmp == e:
                tmp = 0
                count += 1
                if count == 2:
                    return i != len(A)-1 and sum(A[i+1:]) == e
        return False


class Solution2:
    def canThreePartsEqualSum(self, A: List[int]) -> bool:
        return (lambda x, y: x in y and y.index(2*x) and y.index(2*x) != len(y)-1)(sum(A)//3, list(itertools.accumulate(A)))
