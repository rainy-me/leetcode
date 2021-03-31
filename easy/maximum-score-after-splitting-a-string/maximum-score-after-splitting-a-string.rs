#![feature(fn_traits)]

utils::setup!();

#[cfg(test)]
impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut zeros = 0;
        let mut ones = 0;
        let mut max = i32::MIN;
        let chars = s.chars().collect::<Vec<char>>();
        for n in 0..chars.len() {
            match chars.get(n).unwrap() {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => unreachable!(),
            }
            if n != chars.len() - 1 {
                // dbg!(n, zeros, ones);
                max = max.max(zeros - ones);
            }
        }
        max + ones
    }
}

test! {
    max_score,
    ("011101".to_string(),) => 5;
    ("00111".to_string(),) => 5;
    ("1111".to_string(),) => 3;
}
