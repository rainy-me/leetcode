impl Solution {
  pub fn min_start_value(nums: Vec<i32>) -> i32 {
    let mut start = 1;
    let mut sum = 0;
    for n in nums.iter() {
      let diff = n + sum;
      if diff < 0 {
        start -= diff;
        sum -= diff;
      }
      sum += n;
    }
    start
  }
}

struct Solution {}

fn main() {
  assert_eq!(0, 0);
}
