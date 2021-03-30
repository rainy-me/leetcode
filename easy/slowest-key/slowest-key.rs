#![feature(fn_traits)]

use utils::setup;

setup!();

#[cfg(test)]
impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut ret = None;
        let mut slowest = 0;
        for (index, c) in keys_pressed.chars().enumerate() {
            let span = unsafe { release_times.get_unchecked(index) }
                - if index == 0 {
                    0
                } else {
                    *release_times.get(index - 1).unwrap()
                };
            if span == slowest {
                if ret.is_none() || ret.unwrap() < c {
                    ret = Some(c)
                }
            };
            if span > slowest {
                ret = Some(c);
                slowest = span
            }
        }
        ret.unwrap()
    }
}

test! {
    slowest_key,
    (vec![9,29,49,50],"cbcd".to_string()) => 'c';
    (vec![12,23,36,46,62],"spuda".to_string()) => 'a';
}
