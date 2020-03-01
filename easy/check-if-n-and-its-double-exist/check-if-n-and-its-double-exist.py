class Solution:
    def checkIfExist(self, arr: List[int]) -> bool:
        m = set()
        for n in arr:
            if n in m:
                return True
            else:
                m.add(2*n)
                if not (n & 1):
                    m.add(n/2)
        return False
