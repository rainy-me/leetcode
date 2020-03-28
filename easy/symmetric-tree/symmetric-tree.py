# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None


class Solution:
    def __init__(self):
        pass
        #self.stop = False

    def isSymmetric(self, root: TreeNode) -> bool:
        if not root:
            return True

        def traverse(a, b):
            # if self.stop:
            #    return False
            if a == b == None:
                return True
            if not (a and b):
                self.stop = True
                return False
            if a.val != b.val:
                return False
            return traverse(a.left, b.right) and traverse(a.right, b.left)
        return traverse(root.left, root.right)
