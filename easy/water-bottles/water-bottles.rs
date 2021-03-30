#![feature(fn_traits)]

use utils::setup;

setup!();

#[cfg(test)]
impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        num_bottles + (num_bottles - 1) / (num_exchange - 1)
    }
}

test! {
    num_water_bottles,
    (9,3) => 13;
    (15,4) => 19;
    (5,5) => 6;
    (2,3) => 2;
}
