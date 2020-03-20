// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
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

impl Solution {
  pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    Self::dfs(root, None)
  }
  fn dfs(maybe_node: Option<Rc<RefCell<TreeNode>>>, maybe_root_val: Option<i32>) -> bool {
    match maybe_node {
      Some(node) => {
        let mut node_ref = node.borrow_mut();
        if let Some(root_val) = maybe_root_val {
          return node_ref.val == root_val
            && Self::dfs(node_ref.left.take(), Some(root_val))
            && Self::dfs(node_ref.right.take(), Some(root_val));
        }
        Self::dfs(node_ref.left.take(), Some(node_ref.val))
          && Self::dfs(node_ref.right.take(), Some(node_ref.val))
      }
      None => true,
    }
  }
}

struct Solution {}

fn main() {
  assert_eq!(0, 0);
}
