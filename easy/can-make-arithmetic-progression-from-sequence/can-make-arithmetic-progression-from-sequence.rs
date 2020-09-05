struct Solution {}

impl Solution {
  pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
    arr.sort_unstable();
    let d = arr[1] - arr[0];
    for i in 1..arr.len() {
      if arr[i] - d != arr[i - 1] {
        return false;
      }
    }
    true
  }
}

fn main() {
  assert_eq!(
    Solution::can_make_arithmetic_progression(vec![3, 5, 1]),
    true
  );

  assert_eq!(
    Solution::can_make_arithmetic_progression(vec![1, 2, 4]),
    false
  );
}
