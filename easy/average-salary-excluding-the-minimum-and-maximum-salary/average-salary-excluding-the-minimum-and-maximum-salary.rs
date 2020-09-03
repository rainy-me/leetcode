struct Solution {}
impl Solution {
  pub fn average(salary: Vec<i32>) -> f64 {
    let (mut min, mut max, mut sum, len) = (i32::max_value(), 0, 0, salary.len());
    for s in salary {
      min = min.min(s);
      max = max.max(s);
      sum += s;
    }
    (sum - min - max) as f64 / (len - 2) as f64
  }
}
fn main() {
  assert_eq!(Solution::average(vec![4000, 3000, 1000, 2000]), 2500f64);
  assert_eq!(Solution::average(vec![1000, 2000, 3000]), 2000f64);
  assert_eq!(
    Solution::average(vec![6000, 5000, 4000, 3000, 2000, 1000]),
    3500f64
  );
  assert_eq!(
    Solution::average(vec![8000, 9000, 2000, 3000, 6000, 1000]),
    4750f64
  );
}
