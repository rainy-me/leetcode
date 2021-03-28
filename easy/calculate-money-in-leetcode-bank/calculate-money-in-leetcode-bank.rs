#![feature(fn_traits)]

use utils::setup;

setup!();

#[cfg(test)]
impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let weeks = n / 7;
        let rest = n % 7;
        weeks * (weeks - 1) * 7 / 2
            + if weeks > 0 {
                weeks * 28 + weeks * rest
            } else {
                0
            }
            + match rest {
                0 => 0,
                1 => 1,
                2 => 3,
                3 => 6,
                4 => 10,
                5 => 15,
                6 => 21,
                7 => 28,
                _ => unreachable!(),
            }
    }
}

test! {
    total_money,
    (4,) => 10;
    (10,) => 37;
    (20,) => 96;
}
