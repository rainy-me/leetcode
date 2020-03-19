impl Solution {
  pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
    let mut m = manager;
    let mut t = inform_time;
    (0..n)
      .map(|i| Self::dfs(i as usize, &mut m, &mut t))
      .max()
      .unwrap()
  }

  fn dfs(i: usize, m: &mut Vec<i32>, t: &mut Vec<i32>) -> i32 {
    if m[i] != -1 {
      t[i] += Self::dfs(m[i] as usize, m, t);
      m[i] = -1;
    }
    t[i]
  }
}
struct Solution {}

fn main() {
  assert_eq!(Solution::num_of_minutes(1, 0, vec![-1], vec![0]), 0);

  assert_eq!(
    Solution::num_of_minutes(6, 2, vec![2, 2, -1, 2, 2, 2], vec![0, 0, 1, 0, 0, 0]),
    1
  );

  assert_eq!(
    Solution::num_of_minutes(7, 6, vec![1, 2, 3, 4, 5, 6, -1], vec![0, 6, 5, 4, 3, 2, 1]),
    21
  );

  assert_eq!(
    Solution::num_of_minutes(
      15,
      0,
      vec![-1, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6],
      vec![1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]
    ),
    3
  );

  assert_eq!(
    Solution::num_of_minutes(4, 2, vec![3, 3, -1, 2], vec![0, 0, 162, 914]),
    1076
  );
}
