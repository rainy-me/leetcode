use std::collections::HashMap;

impl Solution {
  pub fn num_equiv_domino_pairs(mut dominoes: Vec<Vec<i32>>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for d in &mut dominoes {
      d.sort();
      *map.entry(d[0] * 10 + d[1]).or_default() += 1;
    }
    map.values().fold(0, |s, &v| s + v * (v - 1) / 2)
  }
}

struct Solution {}

fn main() {
  assert_eq!(0, 0);
}
