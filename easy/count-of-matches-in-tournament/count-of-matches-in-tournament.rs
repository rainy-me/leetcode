#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        match n {
            1 => 0,
            _ if n & 1 == 0 => {
                let played = n / 2;
                played + Solution::number_of_matches(played)
            }
            _ => {
                let played = (n - 1) / 2;
                played + Solution::number_of_matches(played + 1)
            }
        }
    }
}

fn main() {}

#[test]
fn test() {
    for (input, output) in vec![(3, 2), (7, 6), (14, 13)] {
        assert_eq!(Solution::number_of_matches(input), output)
    }
}
