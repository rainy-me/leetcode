#![feature(fn_traits)]

use utils::setup;

setup!();

#[cfg(test)]
use std::cmp::Ordering;

#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
impl Solution {
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let mut counter = HashMap::new();
        nums.iter()
            .for_each(|&num| *counter.entry(num).or_insert(0) += 1);
        nums.sort_unstable_by(|a, b| {
            let fa = counter.get(a).unwrap();
            let fb = counter.get(b).unwrap();
            match fa.cmp(fb) {
                Ordering::Equal => b.cmp(a),
                cmp => cmp,
            }
        });
        nums
    }
}

test! {
    frequency_sort,
    (vec![1,1,2,2,2,3],) => vec![3,1,1,2,2,2];
    (vec![2,3,1,3,2],) => vec![1,3,3,2,2];
    (vec![-1,1,-6,4,5,-6,1,4,1],) => vec! [5,-1,4,4,-6,-6,1,1,1];
}
