# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None


class Solution:
    def findTilt(self, root: TreeNode) -> int:
        def tilt_and_sum(node):  # -> [sum,tilt]
            if not node:
                return 0, 0
            if node.left == node.right:
                return node.val, 0
            s1, t1 = tilt_and_sum(node.left)
            s2, t2 = tilt_and_sum(node.right)
            return s1+s2+node.val, t1 + t2 + abs(s2-s1)
        _, t = tilt_and_sum(root)
        return t
