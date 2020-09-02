struct Solution {}
use std::collections::HashMap;

impl Solution {
  pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    nums.iter().fold(0, |mut acc, c| {
      let count = map.entry(c).or_insert(0);
      acc += *count;
      *count += 1;
      acc
    })
  }
}

fn main() {
  assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
  assert_eq!(Solution::num_identical_pairs(vec![1, 1, 1, 1]), 6);
  assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3]), 0);
}
