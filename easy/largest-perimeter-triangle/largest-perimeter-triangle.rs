#![feature(fn_traits)]

utils::setup!();

#[cfg(test)]
impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        for n in (2..(nums.len())).rev() {
            unsafe {
                let large = nums.get_unchecked(n);
                let left = nums.get_unchecked(n - 1) + nums.get_unchecked(n - 2);
                if large < &left {
                    return large + left;
                }
            }
        }
        0
    }
}

test! {
    largest_perimeter,
    (vec![2,1,2],) => 5;
    (vec![1,2,1],) => 0;
    (vec![3,2,3,4],) => 10;
    (vec![3,6,2,3],) => 8;
}
