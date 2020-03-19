use std::cell::RefCell;
use std::rc::Rc;

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

// creating new tree seems to be slower than mutating one of the origin
impl Solution {
  pub fn merge_trees(
    t1: Option<Rc<RefCell<TreeNode>>>,
    t2: Option<Rc<RefCell<TreeNode>>>,
  ) -> Option<Rc<RefCell<TreeNode>>> {
    let tree = match (t1, t2) {
      (None, None) => None,
      (node, None) => node,
      (None, node) => node,
      (Some(t1_node), Some(t2_node)) => {
        {
          let mut t1_ref = t1_node.borrow_mut();
          let mut t2_ref = t2_node.borrow_mut();
          t1_ref.val += t2_ref.val;
          t1_ref.left = Self::merge_trees(t1_ref.left.take(), t2_ref.left.take());
          t1_ref.right = Self::merge_trees(t1_ref.right.take(), t2_ref.right.take());
        }
        Some(t1_node)
      }
    };
    tree
  }
}

impl Solution {
  pub fn merge_trees2(
    t1: Option<Rc<RefCell<TreeNode>>>,
    t2: Option<Rc<RefCell<TreeNode>>>,
  ) -> Option<Rc<RefCell<TreeNode>>> {
    let tree = match (t1, t2) {
      (None, None) => None,
      (node, None) => node,
      (None, node) => node,
      (Some(t1_node), Some(t2_node)) => {
        let t1_ref = t1_node.borrow();
        let t2_ref = t2_node.borrow();
        let mut t = TreeNode::new(t1_ref.val + t2_ref.val);
        t.left = Self::merge_trees(t1_ref.left.clone(), t2_ref.left.clone());
        t.right = Self::merge_trees(t1_ref.right.clone(), t2_ref.right.clone());
        Some(Rc::new(RefCell::new(t)))
      }
    };
    tree
  }
}

struct Solution {}

fn main() {
  assert_eq!(0, 0);
}
