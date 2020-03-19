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
  pub fn search_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
  ) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
      let node_ref = node.borrow();

      match node_ref.val - val {
        0 => Some(node.clone()),
        n if n < 0 => Self::search_bst(node_ref.right.clone(), val),
        n => Self::search_bst(node_ref.left.clone(), val),
      }
    } else {
      None
    }
  }
}
struct Solution {}

fn main() {
  assert_eq!(0, 0);
}
