struct Solution {}

impl Solution {
  pub fn max_product(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let len = nums.len();
    (nums[len - 1] - 1) * (nums[len - 2] - 1)
  }
}

fn main() {
  assert_eq!(Solution::max_product(vec![3, 4, 5, 2]), 12);
  assert_eq!(Solution::max_product(vec![1, 5, 4, 5]), 16);
  assert_eq!(Solution::max_product(vec![3, 7]), 12);
}
