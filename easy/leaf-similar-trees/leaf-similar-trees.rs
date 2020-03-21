// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: MaybeTreeNodeRef,
  pub right: MaybeTreeNodeRef,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None,
    }
  }
}

use std::cell::RefCell;
use std::rc::Rc;
type MaybeTreeNodeRef = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
  pub fn leaf_similar(root1: MaybeTreeNodeRef, root2: MaybeTreeNodeRef) -> bool {
    let mut stack1: Vec<MaybeTreeNodeRef> = vec![root1];
    let mut stack2: Vec<MaybeTreeNodeRef> = vec![root2];
    while !stack1.is_empty() && !stack2.is_empty() {
      if Self::dfs(&mut stack1) != Self::dfs(&mut stack2) {
        return false;
      }
    }
    stack1.is_empty() && stack2.is_empty()
  }
  fn dfs(stack: &mut Vec<MaybeTreeNodeRef>) -> i32 {
    loop {
      if let Some(Some(node)) = stack.pop() {
        let mut node_ref = node.borrow_mut();
        match (node_ref.left.take(), node_ref.right.take()) {
          (None, None) => {
            return node_ref.val;
          }
          (maybe_right, maybe_left) => {
            if maybe_left.is_some() {
              stack.push(maybe_left)
            }
            if maybe_right.is_some() {
              stack.push(maybe_right)
            }
          }
        }
      }
    }
  }
}

struct Solution {}

fn main() {
  assert_eq!(0, 0);
}

/*
     bool leafSimilar(TreeNode* root1, TreeNode* root2) {
        stack<TreeNode*> s1 , s2;
        s1.push(root1); s2.push(root2);
        while (!s1.empty() && !s2.empty())
            if (dfs(s1) != dfs(s2)) return false;
        return s1.empty() && s2.empty();
    }

    int dfs(stack<TreeNode*>& s) {
        while (true) {
            TreeNode* node = s.top(); s.pop();
            if (node->right) s.push(node->right);
            if (node->left) s.push(node->left);
            if (!node->left && !node->right) return node->val;
        }
    }
*/
