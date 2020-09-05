struct Solution {}

impl Solution {
  pub fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
    target.sort();
    arr.sort();
    for (&a, &b) in target.iter().zip(arr.iter()) {
      if a != b {
        return false;
      }
    }
    true
  }
}

fn main() {
  assert_eq!(
    Solution::can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3]),
    true
  );
  assert_eq!(Solution::can_be_equal(vec![7], vec![7]), true);
  assert_eq!(Solution::can_be_equal(vec![1, 12], vec![12, 1]), true);
  assert_eq!(Solution::can_be_equal(vec![3, 7, 9], vec![3, 7, 11]), false);
  assert_eq!(
    Solution::can_be_equal(vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1]),
    true
  );
  assert_eq!(
    Solution::can_be_equal(vec![1, 2, 2, 3], vec![1, 1, 2, 3]),
    false
  );
}
