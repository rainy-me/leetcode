"""
# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""


class Solution:
    def maxDepth(self, root: 'Node') -> int:
        if not root:
            return 0

        def bfs(node, d=1):
            if not node or not node.children:
                return d
            return max([bfs(n, d+1) for n in node.children])
        return bfs(root)


class Solution2(object):
    def maxDepth(self, root):
        if root == None:
            return 0
        depth = 0
        stack = [root]
        while stack:
            next_level = []
            while stack:
                node = stack.pop()
                if node.children:
                    next_level += node.children
            stack = next_level
            depth += 1
        return depth
