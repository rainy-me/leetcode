use std::collections::HashMap;

impl Solution {
  pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
    let mut nums = 0;
    for p in points.iter() {
      let mut m: HashMap<i32, i32> = HashMap::with_capacity(points.len());
      for q in points.iter() {
        let key = (q[0] - p[0]).pow(2) + (q[1] - p[1]).pow(2);
        *m.entry(key).or_default() += 1;
      }
      nums += m.values().fold(0, |s, &v| s + v * (v - 1));
    }
    nums
  }
}
struct Solution {}

fn main() {
  assert_eq!(
    Solution::number_of_boomerangs(
      [[0, 0], [1, 0], [2, 0]]
        .iter()
        .map(|v| v.to_vec())
        .collect::<Vec<Vec<i32>>>()
    ),
    2
  );
}
