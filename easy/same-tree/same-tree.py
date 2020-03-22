class Solution:
    def isSameTree(self, p: TreeNode, q: TreeNode) -> bool:
        if bool(p) ^ bool(q):
            return False
        if p == q == None:
            return True
        if p.val == q.val:
            return self.isSameTree(p.left, q.left) and self.isSameTree(p.right, q.right)
        return self.isSameTree(p, q)
