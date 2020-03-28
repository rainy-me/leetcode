# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None


class Solution(object):
    def sumOfLeftLeaves(self, root):

        def is_leaf(root):
            return root.left == root.right

        _sum = 0
        if not root:
            return _sum

        if root.left:
            if is_leaf(root.left):
                _sum += root.left.val
            else:
                _sum += self.sumOfLeftLeaves(root.left)
        _sum += self.sumOfLeftLeaves(root.right)
        return _sum
