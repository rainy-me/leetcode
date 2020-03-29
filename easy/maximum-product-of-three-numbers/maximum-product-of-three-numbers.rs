impl Solution {
  pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    let p1 = nums[0] * nums[1] * nums[n - 1];
    let p2 = nums[n - 1] * nums[n - 2] * nums[n - 3];
    if p1 > p2 {
      p1
    } else {
      p2
    }
  }
}

impl Solution {
  pub fn maximum_product(nums: Vec<i32>) -> i32 {
    use std::cmp;
    let mut min1 = i32::max_value();
    let mut min2 = i32::max_value();
    let mut max1 = i32::min_value();
    let mut max2 = i32::min_value();
    let mut max3 = i32::min_value();

    for n in nums.iter() {
      if n <= &min1 {
        min2 = min1;
        min1 = *n;
      } else if n <= &min2 {
        min2 = *n;
      }
      if n >= &max1 {
        max3 = max2;
        max2 = max1;
        max1 = *n;
      } else if n >= &max2 {
        max3 = max2;
        max2 = *n;
      } else if n >= &max3 {
        max3 = *n;
      }
    }
    cmp::max(min1 * min2 * max1, max1 * max2 * max3)
  }
}

struct Solution {}

fn main() {
  assert_eq!(0, 0);
}
