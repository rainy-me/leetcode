#![feature(fn_traits)]

use utils::setup;

setup!();

#[cfg(test)]
use std::collections::HashSet;

#[cfg(test)]
impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        Solution::lns(&s[..]).to_owned()
    }

    pub fn lns(s: &str) -> &str {
        if s.len() < 2 {
            return &"";
        }
        let mut set = HashSet::new();
        for s in s.chars() {
            set.insert(s);
        }
        for (i, char) in s.chars().enumerate() {
            let case_swapped = if char.is_ascii_lowercase() {
                char.to_ascii_uppercase()
            } else {
                char.to_ascii_lowercase()
            };
            if set.get(&case_swapped).is_none() {
                let left = Solution::lns(&s[..i]);
                let right = Solution::lns(&s[i + 1..]);
                return if left.len() >= right.len() {
                    left
                } else {
                    right
                };
            }
        }
        s
    }
}

test! {
    longest_nice_substring,
    ("YazaAay".to_string(),) => "aAa".to_string();
    ("Bb".to_string(),) => "Bb".to_string();
    ("c".to_string(),) => "".to_string();
    ("dDzeE".to_string(),) => "dD".to_string();
}
