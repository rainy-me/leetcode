#![feature(fn_traits)]

utils::setup!(
    use utils::TreeNode;
    use std::rc::Rc;
    use std::cell::RefCell;
);

#[cfg(test)]
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::convert(&nums, 0, (nums.len() - 1) as i32)
    }
    fn convert(nums: &Vec<i32>, start: i32, end: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if start > end {
            return None;
        }

        let mid = (start + end) / 2;
        let mut node = TreeNode::new(nums[mid as usize]);
        node.left = Solution::convert(nums, start, mid - 1);
        node.right = Solution::convert(nums, mid + 1, end);
        Some(Rc::new(RefCell::new(node)))
    }
}
