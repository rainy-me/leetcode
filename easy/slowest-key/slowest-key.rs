#![feature(fn_traits)]

use utils::setup;

setup!();

#[cfg(test)]
impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut chars = keys_pressed.chars();
        let mut ret = chars.next().unwrap();
        let mut slowest = release_times[0];
        for (index, c) in chars.enumerate() {
            let span = unsafe {
                release_times.get_unchecked(index + 1) - release_times.get_unchecked(index)
            };
            if span == slowest && ret < c {
                ret = c
            };
            if span > slowest {
                ret = c;
                slowest = span
            }
        }
        ret
    }
}

test! {
    slowest_key,
    (vec![9,29,49,50],"cbcd".to_string()) => 'c';
    (vec![12,23,36,46,62],"spuda".to_string()) => 'a';
}
