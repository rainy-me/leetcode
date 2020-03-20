"""
# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""


class Solution:
    def postorder(self, root: 'Node', ans=[]) -> List[int]:
        output = []
        stack = [root]

        if not root:
            return output

        while stack:
            last = stack.pop()
            stack += last.children
            output += [last.val]
        return output[::-1]
