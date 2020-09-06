struct Solution {}
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
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
  pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    Self::sum_node(root, false, false)
  }

  pub fn sum_node(root: Option<Rc<RefCell<TreeNode>>>, p: bool, gp: bool) -> i32 {
    if let Some(node) = root {
      let mut node_ref = node.borrow_mut();
      let even = node_ref.val & 1 == 0;
      return Self::sum_node(node_ref.left.take(), even, p)
        + Self::sum_node(node_ref.right.take(), even, p)
        + if gp { node_ref.val } else { 0 };
    }
    0
  }
}

fn main() {
  assert_eq!(0, 0);
}
