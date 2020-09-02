struct Solution {}

impl Solution {
  pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    (0..2 * n)
      .map(|i| match i & 1 {
        1 => nums[(n + (i - 1) / 2) as usize],
        _ => nums[(i / 2) as usize],
      })
      .collect()
  }
}

fn main() {
  assert_eq!(
    Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3),
    vec![2, 3, 5, 4, 1, 7]
  );
  assert_eq!(
    Solution::shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
    vec![1, 4, 2, 3, 3, 2, 4, 1]
  );
  assert_eq!(Solution::shuffle(vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]);
}
