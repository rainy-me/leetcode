impl Solution {
  pub fn transpose2(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    match a.len() {
      0 => vec![],
      _ => (0..a[0].len())
        .map(|i| a.iter().map(|v| v[i]).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>(),
    }
  }

  pub fn transpose(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    a.iter().enumerate().fold(vec![] |(i,mut v)|)
  }
}

struct Solution {}

fn main() {
  let v: Vec<Vec<i32>> = vec![];
  assert_eq!(Solution::transpose(v.clone()), v.clone());

  assert_eq!(
    Solution::transpose(
      [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        .iter()
        .map(|v| v.to_vec())
        .collect::<Vec<Vec<i32>>>()
    ),
    [[1, 4, 7], [2, 5, 8], [3, 6, 9]]
      .iter()
      .map(|v| v.to_vec())
      .collect::<Vec<Vec<i32>>>()
  );

  assert_eq!(
    Solution::transpose(
      [[1, 2, 3], [4, 5, 6]]
        .iter()
        .map(|v| v.to_vec())
        .collect::<Vec<Vec<i32>>>()
    ),
    [[1, 4], [2, 5], [3, 6]]
      .iter()
      .map(|v| v.to_vec())
      .collect::<Vec<Vec<i32>>>()
  );
}
