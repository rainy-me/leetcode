# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None


class Solution:
    def sumRootToLeaf(self, root: TreeNode) -> int:
        def calc(node, base=""):
            if not node:
                return 0
            if not (node.left or node.right):
                return int(base + str(node.val), 2)
            return calc(node.left, base + str(node.val)) + calc(node.right, base + str(node.val))
        return calc(root)
