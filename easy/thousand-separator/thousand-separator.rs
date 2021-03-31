#![feature(fn_traits)]

utils::setup!();

#[cfg(test)]
impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        n.to_string()
            .chars()
            .rev()
            .enumerate()
            .flat_map(|(index, char)| {
                if index % 3 == 0 && index > 0 {
                    vec!['.', char]
                } else {
                    vec![char]
                }
            })
            .collect::<Vec<char>>()
            .iter()
            .rev()
            .collect()
    }
}

test! {
    thousand_separator,
    (987,) => "987".to_string();
    (1234,) => "1.234".to_string();
    (123456789,) => "123.456.789".to_string();
    (0,) => "0".to_string();
}
