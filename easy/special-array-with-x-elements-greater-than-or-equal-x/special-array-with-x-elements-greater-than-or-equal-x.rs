#![feature(fn_traits)]

use utils::setup;

setup!();

#[cfg(test)]
impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let len = nums.len() as i32;
        for (index, n) in nums.iter().enumerate() {
            let left = len - index as i32;
            if n >= &left && &left > nums.get((index as i32 - 1) as usize).unwrap_or(&0) {
                return left;
            }
        }
        -1
    }
}

test! {
    special_array,
    (vec![3,5],) => 2;
    (vec![0,0],) => -1;
    (vec![0,4,3,0,4],) => 3;
    (vec![3,6,7,7,0],) => -1;
}
