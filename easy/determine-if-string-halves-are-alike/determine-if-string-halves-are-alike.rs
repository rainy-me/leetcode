#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let middle = s.len() / 2;
        Solution::collect(&s[..middle]) == Solution::collect(&s[middle..])
    }

    fn collect(slice: &str) -> i32 {
        slice.chars().fold(0, |acc, curr| match curr {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => acc + 1,
            _ => acc,
        })
    }
}

fn main() {}

#[test]
fn test() {
    for (i, o) in vec![
        ("book".to_string(), true),
        ("textbook".to_string(), false),
        ("MerryChristmas".to_string(), false),
        ("AbCdEfGh".to_string(), true),
    ] {
        assert_eq!(Solution::halves_are_alike(i), o);
    }
}
