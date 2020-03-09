class Solution:
    def countBits(self, num: int) -> List[int]:
        return [bin(n).count("1") for n in range(num+1)]
