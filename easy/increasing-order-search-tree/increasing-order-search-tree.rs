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
  pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    Self::increasing_bst_with_tail(root, None)
  }

  pub fn increasing_bst_with_tail(
    root: Option<Rc<RefCell<TreeNode>>>,
    tail: Option<Rc<RefCell<TreeNode>>>,
  ) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
      Some(root_node) => {
        let mut root_node_ref = root_node.borrow_mut();
        let new_node =
          Self::increasing_bst_with_tail(root_node_ref.left.take(), Some(root_node.clone()));
        root_node_ref.right = Self::increasing_bst_with_tail(root_node_ref.right.take(), tail);
        new_node
      }
      None => tail,
    }
  }
}
struct Solution {}

fn main() {
  assert_eq!(0, 0);
}
