struct Solution {}

impl Solution {
  pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
    let mut count = 3;
    for num in arr {
      match num & 1 {
        1 => {
          count -= 1;
          if count == 0 {
            return true;
          }
        }
        _ => count = 3,
      }
    }
    false
  }
}
fn main() {
  assert_eq!(Solution::three_consecutive_odds(vec![2, 6, 4, 1]), false);
  assert_eq!(
    Solution::three_consecutive_odds(vec![1, 2, 34, 3, 4, 5, 7, 23, 12]),
    true
  );
}
