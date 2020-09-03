struct Solution {}
use std::collections::HashSet;
impl Solution {
  pub fn dest_city(mut paths: Vec<Vec<String>>) -> String {
    let mut start = HashSet::new();
    let mut end = HashSet::new();
    for path in paths.iter_mut() {
      if let (Some(b), Some(a)) = (path.pop(), path.pop()) {
        start.insert(a);
        end.insert(b);
      }
    }
    end.difference(&start).next().cloned().unwrap()
  }
}

fn to_vec_string(arr: &[&str; 2]) -> Vec<String> {
  arr
    .to_vec()
    .iter()
    .map(|c| c.to_string())
    .collect::<Vec<String>>()
}

fn main() {
  assert_eq!(
    Solution::dest_city(
      [
        ["London", "New York"],
        ["New York", "Lima"],
        ["Lima", "Sao Paulo"]
      ]
      .iter()
      .map(to_vec_string)
      .collect::<Vec<Vec<String>>>()
    ),
    "Sao Paulo".to_owned()
  );

  assert_eq!(
    Solution::dest_city(
      [["B", "C"], ["D", "B"], ["C", "A"]]
        .iter()
        .map(to_vec_string)
        .collect::<Vec<Vec<String>>>()
    ),
    "A".to_owned()
  );

  assert_eq!(
    Solution::dest_city(
      [["A", "Z"]]
        .iter()
        .map(to_vec_string)
        .collect::<Vec<Vec<String>>>()
    ),
    "Z".to_owned()
  );
}
