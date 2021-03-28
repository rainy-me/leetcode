#![feature(fn_traits)]

use utils::setup;

setup!();

#[cfg(test)]
impl Solution {
    pub fn reformat_number(number: String) -> String {
        let mut count = 0;
        let mut chars: Vec<char> = number
            .chars()
            .flat_map(|char| match char {
                ' ' | '-' => vec![],
                _ => {
                    count += 1;
                    let mut vec = vec![char];
                    if count > 0 && count % 3 == 0 {
                        vec.push('-')
                    }
                    vec
                }
            })
            .collect();
        let mut len = chars.len();
        if unsafe { *chars.get_unchecked(len - 1) } == '-' {
            chars.pop();
            len -= 1;
        }
        if unsafe { *chars.get_unchecked(len - 2) } == '-' {
            chars.swap(len - 2, len - 3);
        }
        chars.iter().collect()
    }
}

test! {
    reformat_number,
    ("1-23-45 6".to_string(),) => "123-456".to_string();
    ("123 4-567".to_string(),) => "123-45-67".to_string();
    ("123 4-5678".to_string(),) => "123-456-78".to_string();
    ("12".to_string(),) => "12".to_string();
    ("--17-5 229 35-39475 ".to_string(),) => "175-229-353-94-75".to_string();
}
