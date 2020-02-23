class Solution(object):
    def reverseVowels(self, s):
        vowels = {'a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'}
        S = list(s)
        l, r = 0, len(S) - 1
        while l < r:
            while l < r and S[l] not in vowels:
                l += 1
            while r > l and S[r] not in vowels:
                r -= 1
            S[l], S[r] = S[r], S[l]
            l += 1
            r -= 1
        return ''.join(S)
