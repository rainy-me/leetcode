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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut node_ref = node.borrow_mut();
            let right = node_ref.right.take();
            let left = node_ref.left.take();
            node_ref.left = Self::invert_tree(right);
            node_ref.right = Self::invert_tree(left);
            Some(node.clone())
        } else {
            None
        }
    }
}

struct Solution {}

fn main() {
    assert_eq!(0, 0);
}
