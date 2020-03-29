# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None


class Solution:
    def isSubtree(self, s: TreeNode, t: TreeNode) -> bool:
        def isSameTree(l, r):
            if l == r == None:
                return True
            if not l or not r:
                return False
            return l.val == r.val and isSameTree(l.left, r.left) and isSameTree(l.right, r.right)
        stack = [s]
        while stack:
            node = stack.pop()
            if node and node.val == t.val:
                if isSameTree(node, t):
                    return True
            if node.left:
                stack.append(node.left)
            if node.right:
                stack.append(node.right)
        return False


class Solution2:
    """
    Merkle hashing
    """

    def isSubtree(self, s, t):
        from hashlib import sha256

        def hash_(x):
            S = sha256()
            S.update(x)
            return S.hexdigest()

        def merkle(node):
            if not node:
                return '#'
            m_left = merkle(node.left)
            m_right = merkle(node.right)
            node.merkle = hash_(m_left + str(node.val) + m_right)
            return node.merkle

        merkle(s)
        merkle(t)

        def dfs(node):
            if not node:
                return False
            return (node.merkle == t.merkle or
                    dfs(node.left) or dfs(node.right))

        return dfs(s)
