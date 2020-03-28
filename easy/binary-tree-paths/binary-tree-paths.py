# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None


class Solution:
    def binaryTreePaths(self, root: TreeNode) -> List[str]:
        paths = []

        def getPath(node: TreeNode, path):
            nonlocal paths
            if not node:
                return
            current_path = path + [node.val]
            if node.left == node.right:
                paths.append("->".join([str(n) for n in current_path]))
                return
            if node.left:
                getPath(node.left, current_path)
            if node.right:
                getPath(node.right, current_path)
        getPath(root, [])
        return paths
