#![feature(fn_traits)]

#[cfg(test)]
use std::collections::HashMap;
use utils::setup;

setup!();

#[cfg(test)]
impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut s: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            match s.get(&n) {
                None => {
                    sum += n;
                    s.insert(n, 1);
                }
                Some(1) => {
                    sum -= n;
                    *s.entry(n).or_default() += 1;
                }
                _ => {}
            }
        }
        sum
    }
}

test! {
    sum_of_unique,
    (vec![1, 2, 3, 2],) => 4;
    (vec![1, 1, 1, 1, 1, 1],) => 0;
    (vec![1, 2, 3, 4, 5],) => 15;
}
