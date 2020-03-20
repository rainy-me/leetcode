impl Solution {
  pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut h = heights.clone();
    h.sort_unstable();
    heights
      .iter()
      .zip(h.iter())
      .map(|(a, b)| if a == b { 0 } else { 1 })
      .sum()
  }
}
struct Solution {}

fn main() {
  assert_eq!(Solution::height_checker(vec![5, 1, 2, 3, 4]), 5);
  assert_eq!(Solution::height_checker(vec![1, 2, 3, 4, 5]), 0);
}
