use std::collections::HashMap;

impl Solution {
  pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
    let mut map = HashMap::new();
    for row in matrix.iter() {
      let head = row[0];
      let pattern: Vec<i32> = row.iter().map(|cell| cell ^ head).collect();
      *map.entry(pattern).or_insert(0) += 1;
    }
    *map.values().max().unwrap_or(&0)
  }
}

struct Solution {}

fn main() {
  assert_eq!(
    Solution::max_equal_rows_after_flips(
      [[0, 0, 0], [0, 0, 1], [1, 1, 0]]
        .iter()
        .map(|v| v.to_vec())
        .collect()
    ),
    2
  )
}
