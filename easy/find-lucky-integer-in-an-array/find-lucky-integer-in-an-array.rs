struct Solution {}
use std::collections::HashMap;

impl Solution {
  pub fn find_lucky(arr: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for num in arr.iter() {
      *map.entry(num).or_insert(0) += 1;
    }
    map.retain(|&k, v| k == v);
    match map.values().max() {
      Some(&num) => num,
      None => -1,
    }
  }
}

fn main() {
  assert_eq!(Solution::find_lucky(vec![2, 2, 3, 4]), 2);
  assert_eq!(Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]), 3);
  assert_eq!(Solution::find_lucky(vec![2, 2, 2, 3, 3]), -1);
  assert_eq!(Solution::find_lucky(vec![5]), -1);
  assert_eq!(Solution::find_lucky(vec![7, 7, 7, 7, 7, 7, 7]), 7);
}
