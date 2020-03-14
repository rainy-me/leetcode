impl Solution {
  pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
    matrix
      .iter()
      .zip(matrix[1..].iter())
      .all(|(r1, r2)| match r1.len() {
        1 => true,
        l => r1[..=l - 2] == r2[1..],
      })
  }
}

struct Solution {}

fn main() {
  assert_eq!(
    Solution::is_toeplitz_matrix(
      [[1, 2, 3, 4], [5, 1, 2, 3], [9, 5, 1, 2]]
        .iter()
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<i32>>>()
    ),
    true
  );

  assert_eq!(Solution::is_toeplitz_matrix(vec![vec![1], vec![5]]), true);
  assert_eq!(
    Solution::is_toeplitz_matrix(vec![vec![57, 54], vec![84, 57]]),
    true
  );
}
