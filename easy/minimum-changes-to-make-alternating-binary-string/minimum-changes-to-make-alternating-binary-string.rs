#![feature(fn_traits)]

utils::setup!();

#[cfg(test)]
impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut count = 0;
        for (i, char) in s.chars().enumerate() {
            if i & 1 == char.to_digit(10).unwrap() as usize {
                count += 1
            }
        }

        count.min(s.len() - count) as i32
    }
}

test! {
    min_operations,
    ("0100".to_string(),) => 1;
    ("10".to_string(),) => 0;
    ("1111".to_string(),) => 2;
}
