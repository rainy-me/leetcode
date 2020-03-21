# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None


class Solution:
    def averageOfLevels(self, root: TreeNode) -> List[float]:
        ans = []
        level = [root]
        while level:
            ans.append(sum([n.val for n in level]) / len(level))
            tmp = []
            for n in level:
                if n.left:
                    tmp.append(n.left)
                if n.right:
                    tmp.append(n.right)
            level = tmp
        return ans
