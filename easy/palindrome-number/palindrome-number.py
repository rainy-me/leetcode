class Solution:
    def isPalindrome(self, x: int) -> bool:
        x = str(x)
        l = len(x)
        middle = l//2
        if l & 1:
            return x[:middle][::-1] == x[middle+1:]
        else:
            return x[middle:][::-1] == x[:middle]
