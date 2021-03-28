#![feature(fn_traits)]

use utils::setup;

setup!();

#[cfg(test)]
impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut count = 0;
        for i in 0..nums.len() {
            if nums[i] > nums[(i + 1) % nums.len()] {
                count += 1;
                if count > 1 {
                    return false;
                }
            }
        }
        true
    }
}

test! {
    check,
    (vec![7,9,1,1,1,3],) => true;
    (vec![6,10,6],) => true;
    (vec![5,5,6,6,6,9,1,2],) => true;
    (vec![3,4,5,1,2],) => true;
    (vec![2,1,3,4],) => false;
    (vec![1,2,3],) => true;
    (vec![1,1,1],) => true;
    (vec![2,1],) => true;
}
