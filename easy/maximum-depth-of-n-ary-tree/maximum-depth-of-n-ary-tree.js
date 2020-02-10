/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number}
 */
var maxDepth = function (root) {
  if (!root) return 0
  let ret = 1;
  const walk = (node, depth) => {
    if (node) {
      if (node.left) {
        walk(node.left, depth + 1)
      }
      if (node.right) {
        walk(node.right, depth + 1)
      }
    }
    if (depth > ret) ret = depth
  }
  walk(root, 1)
  return ret
};