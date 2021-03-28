#![feature(fn_traits)]

use utils::setup;

setup!();

#[cfg(test)]
impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut diff = (None, None);
        let mut matched = false;
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 == c2 {
                continue;
            }
            match diff {
                (None, None) => {
                    diff = (Some(c1), Some(c2));
                }
                (Some(_2), Some(_1)) if !matched && c1 == _1 && c2 == _2 => {
                    matched = true;
                }
                _ => return false,
            }
        }
        diff == (None, None) || matched
    }
}

test! {
    are_almost_equal,
    ("aa".to_string(),"ac".to_string()) => false;
    ("bank".to_string(),"kanb".to_string()) => true;
    ("kelb".to_string(),"kelb".to_string()) => true;
    ("abcd".to_string(),"dcba".to_string()) => false;
}
