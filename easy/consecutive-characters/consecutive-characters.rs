#![feature(fn_traits)]

use utils::setup;

setup!();

#[cfg(test)]
impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut last = ' ';
        let mut max = 0;
        let mut curr = 0;
        for c in s.chars() {
            if c == last {
                curr += 1;
            } else {
                max = max.max(curr);
                last = c;
                curr = 1;
            }
        }
        max.max(curr)
    }
}

test! {
    max_power,
    ("x".to_string(),) => 1;
    ("leetcode".to_string(),) => 2;
    ("abbcccddddeeeeedcba".to_string(),) => 5;
    ("triplepillooooow".to_string(),) => 5;
    ("hooraaaaaaaaaaay".to_string(),) => 11;
    ("tourist".to_string(),) => 1;
}
