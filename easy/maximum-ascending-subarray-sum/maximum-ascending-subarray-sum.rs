#![feature(fn_traits)]

use utils::setup;

setup!();

#[cfg(test)]
impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut last = 0;
        let mut span = 0;

        for n in nums {
            if n > last {
                span += n
            } else {
                max = max.max(span);
                span = n;
            }
            last = n;
        }
        max.max(span)
    }
}

test! {
    max_ascending_sum,
    (vec![10,20,30,5,10,50],) => 65;
    (vec![10,20,30,40,50],) => 150;
    (vec![12,17,15,13,10,11,12],) => 33;
    (vec![100,10,1],) => 100;
}
