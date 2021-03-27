use std::fmt::Write;

#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut ret = vec![];
        let (mut w1, mut w2) = (word1.chars(), word2.chars());
        loop {
            match (w1.next(), w2.next()) {
                (None, None) => break,
                (c1, c2) => {
                    if let Some(c1) = c1 {
                        ret.push(c1);
                    }
                    if let Some(c2) = c2 {
                        ret.push(c2);
                    }
                }
            }
        }
        ret.iter().collect()
    }
}

fn main() {}

#[test]
fn test() {
    for (i, o) in vec![
        (("abc", "pqr"), "apbqcr"),
        (("ab", "pqrs"), "apbqrs"),
        (("abcd", "pq"), "apbqcd"),
    ] {
        assert_eq!(
            Solution::merge_alternately(i.0.to_string(), i.1.to_string()),
            o.to_string()
        )
    }
}
