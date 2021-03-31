#![feature(fn_traits)]

utils::setup!();

#[cfg(test)]
impl Solution {
    pub fn make_good(s: String) -> String {
        let mut chars = s.chars().collect::<Vec<char>>();
        loop {
            let len = chars.len();
            for n in 0..len - 1 {
                if chars[n] != chars[n + 1]
                    && (chars[n].to_ascii_uppercase() == chars[n + 1].to_ascii_uppercase())
                {
                    chars.remove(n);
                    chars.remove(n);
                    break;
                }
            }
            if chars.len() == len || chars.len() == 0 {
                break;
            }
        }
        chars.iter().collect()
    }
}

test! {
    make_good,
    ("leEeetcode".to_string(),) => "leetcode".to_string();
    ("abBAcC".to_string(),) => "".to_string();
    ("s".to_string(),) => "s".to_string();
}
