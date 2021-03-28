#[cfg(test)]
use utils::strings;

#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let (mut w1, mut w2) = (
            word1.iter().flat_map(|string| string.chars()),
            word2.iter().flat_map(|string| string.chars()),
        );
        loop {
            match (w1.next(), w2.next()) {
                (Some(a), Some(b)) if a == b => continue,
                (None, None) => return true,
                _ => return false,
            };
        }
    }
}

fn main() {}

#[test]
fn test() {
    assert_eq!(
        Solution::array_strings_are_equal(strings!["ab", "c"], strings!["a", "bc"],),
        true
    );
    assert_eq!(
        Solution::array_strings_are_equal(strings!["a", "cb"], strings!["ab", "c"],),
        false
    );
    assert_eq!(
        Solution::array_strings_are_equal(strings!["abc", "d", "defg"], strings!["abcddefg"],),
        true
    )
}
