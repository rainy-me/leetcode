#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let (mut l, mut max) = (0, 0);
        for char in s.chars() {
            match char {
                '(' => l += 1,
                ')' if l > 0 => {
                    max = max.max(l);
                    l -= 1;
                }
                _ => {}
            }
        }
        max
    }
}
fn main() {
    assert_eq!(0, 0);
}

#[test]
fn test() {
    assert_eq!(Solution::max_depth("(1+(2*3)+((8)/4))+1".to_string()), 3);
    assert_eq!(Solution::max_depth("(1)+((2))+(((3)))".to_string()), 3);
    assert_eq!(Solution::max_depth("1+(2*3)/(2-1)".to_string()), 1);
    assert_eq!(Solution::max_depth("1".to_string()), 0);
}
