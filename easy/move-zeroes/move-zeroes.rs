#![feature(fn_traits)]

utils::setup!();

#[cfg(test)]
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let len = nums.len();
        nums.retain(|n| n != &0);
        nums.resize(len, 0);
    }
}

test! {
    {
        let mut nums = vec![0,1,0,3,12];
        Solution::move_zeroes(&mut nums);
        nums
    } => vec![1,3,12,0,0];
    {
        let mut nums = vec![0];
        Solution::move_zeroes(&mut nums);
        nums
    } => vec![0]
}
