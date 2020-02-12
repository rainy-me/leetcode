# @see https://leetcode.com/problems/find-n-unique-integers-sum-up-to-zero/discuss/465585/JavaC%2B%2BPython-Find-the-Rule
def sumZero(self, n):
    return range(1 - n, n, 2)
