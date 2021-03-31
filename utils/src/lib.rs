#![feature(fn_traits)]

#[macro_export]
macro_rules! strings {
  ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[macro_export]
macro_rules! mat {
  (
    $(
      [$($y:expr),*]
    );*
  ) => (vec![
    $(
      vec![$($y),*]
    ),*
  ]);
  (
    $(
      [$($y:expr),*]
    ),*
  ) => (vec![
    $(
      vec![$($y),*]
    ),*
  ]);
}

#[macro_export]
macro_rules! test {
  (
    $fn:ident,
    $(
      ($($args:tt)*) => $result:expr
    );*;
  ) => {
    #[test]
    fn test() {
      $(
        assert_eq!(
          std::ops::Fn::call(&Solution::$fn, ($($args)*)),
          $result
        );
      )*
    }
  };
  (
    $(
      $block:block => $result:expr
    );*
  ) => {
    #[test]
    fn test() {
      $(
        assert_eq!(
          $block,
          $result
        );
      )*
    }
  }
}

#[macro_export]
macro_rules! setup {
    () => {
        use utils::test;

        #[cfg(test)]
        struct Solution {}

        fn main() {}
    };
    ($($item:item)*) => {
      $(
        #[cfg(test)]
        $item
      )*
      utils::setup!();
    };
}

use std::cell::RefCell;
use std::rc::Rc;
/// Definition for a binary tree node.
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
