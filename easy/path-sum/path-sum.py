# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None


class Solution:
    def __init__(self):
        self.done = False

    def hasPathSum(self, root: TreeNode, sum: int) -> bool:
        def traverse(node, val):
            if self.done:
                return True
            if not node:
                return False
            val += node.val
            if node.left == node.right:
                if val == sum:
                    self.done = True
                    return True
            return traverse(node.left, val) or traverse(node.right, val)
        return traverse(root, 0)


class Solution:
    """
    @see https://leetcode.com/problems/path-sum/discuss/36360/Short-Python-recursive-solution-O(n)
    doing subtract is much better here
    """
    # @param root, a tree node
    # @param sum, an integer
    # @return a boolean
    # 1:27

    def hasPathSum(self, root, sum):
        if not root:
            return False

        if not root.left and not root.right and root.val == sum:
            return True

        sum -= root.val

        return self.hasPathSum(root.left, sum) or self.hasPathSum(root.right, sum)
