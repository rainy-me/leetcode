import collections


class Solution:
    def findMode(self, root: TreeNode) -> List[int]:
        cache = collections.defaultdict(int)

        def next(node):
            if not node:
                return
            cache[node.val] += 1
            next(node.left)
            next(node.right)
        next(root)
        mv = max(cache.values())
        return [k for k in cache if cache[k] == mv]
