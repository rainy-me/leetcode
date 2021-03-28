#![feature(fn_traits)]

use utils::setup;

setup!();

#[cfg(test)]
impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut last = usize::MAX;
        for (index, &n) in nums.iter().enumerate() {
            if n == 1 {
                if last != usize::MAX && index - last <= k as usize {
                    return false;
                }
                last = index;
            }
        }
        true
    }
}

test! {
    k_length_apart,
    (vec![1,0,0,0,1,0,0,1], 2) => true;
    (vec![1,0,0,1,0,1], 2) => false;
    (vec![1,1,1,1,1], 0) => true;
    (vec![0,1,0,1], 1) => true;
}
