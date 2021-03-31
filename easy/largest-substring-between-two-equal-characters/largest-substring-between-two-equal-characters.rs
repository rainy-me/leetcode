#![feature(fn_traits)]

utils::setup!(
    use std::collections::HashMap;
);

#[cfg(test)]
impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut max = 0;
        for (i, c) in s.chars().enumerate() {
            if let Some(last) = map.get(&c) {
                max = max.max(i as i32 - last);
            } else {
                map.insert(c, i as i32);
            }
        }
        max - 1
    }
}

test! {
    max_length_between_equal_characters,
    ("mgntdygtxrvxjnwksqhxuxtrv".to_string(),) => 18;
    ("aa".to_string(),) => 0;
    ("abca".to_string(),) => 2;
    ("cbzxy".to_string(),) => -1;
    ("cabbac".to_string(),) => 4;
}
