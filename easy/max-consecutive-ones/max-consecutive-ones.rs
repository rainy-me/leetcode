impl Solution {
  pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut ans = 0;
    for &n in nums.iter() {
      if n == 1 {
        count += 1;
        ans = ans.max(count);
      } else {
        count = 0;
      }
    }
    ans
  }
}

struct Solution {}

fn main() {
  assert_eq!(0, 0);
}
