use std::collections::HashSet;

impl Solution {
  pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![];
    let mut h = HashSet::new();
    for row in matrix.iter() {
      h.insert(row.iter().min().unwrap());
    }
    for col in 0..matrix[0].len() {
      let max = matrix.iter().map(|row| row[col]).collect::<Vec<i32>>();

      if let Some(val) = h.get(&max.iter().max().unwrap()) {
        ans.push(**val);
      }
    }
    ans
  }
}

struct Solution {}

fn main() {
  assert_eq!(
    Solution::lucky_numbers(
      [[3, 7, 8], [9, 11, 13], [15, 16, 17]]
        .iter()
        .map(|x| x.to_vec())
        .collect()
    ),
    vec![15]
  );

  assert_eq!(
    Solution::lucky_numbers(
      [[1, 10, 4, 2], [9, 3, 8, 7], [15, 16, 17, 12]]
        .iter()
        .map(|x| x.to_vec())
        .collect()
    ),
    vec![12]
  );

  assert_eq!(
    Solution::lucky_numbers([[7, 8], [1, 2]].iter().map(|x| x.to_vec()).collect()),
    vec![7]
  );
}
