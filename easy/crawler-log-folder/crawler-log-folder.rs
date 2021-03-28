#![feature(fn_traits)]

use utils::setup;
#[cfg(test)]
use utils::strings;

setup!();

#[cfg(test)]
impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        logs.iter().fold(0, |acc, log| match log.as_str() {
            "../" if acc > 0 => acc - 1,
            "../" | "./" => acc,
            _ => acc + 1,
        })
    }
}

test! {
    min_operations,
    (strings!["d1/","d2/","../","d21/","./"],) => 2;
    (strings!["d1/","d2/","./","d3/","../","d31/"],) => 3;
    (strings!["d1/","../","../","../"],) => 0;
}
