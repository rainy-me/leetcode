struct Solution {}

impl Solution {
  pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = Vec::new();
    for (dx, dy) in [
      (-1, 0),
      (-1, 1),
      (0, 1),
      (1, 1),
      (1, 0),
      (1, -1),
      (0, -1),
      (-1, -1),
    ]
    .iter()
    {
      let mut x_y = king.clone();
      loop {
        match (x_y[0], x_y[1]) {
          (-1, _) | (8, _) | (_, -1) | (_, 8) => break,
          _ => {
            x_y[0] += dx;
            x_y[1] += dy;
            if queens.contains(&x_y) {
              ans.push(x_y);
              break;
            }
          }
        }
      }
    }
    ans
  }
}

fn main() {
  let queens: Vec<Vec<i32>> = [[0, 0], [1, 1], [2, 2], [3, 4], [3, 5], [4, 4], [4, 5]]
    .iter()
    .map(|p| p.to_vec())
    .collect();
  let king: Vec<i32> = vec![3, 3];

  let mut ans = Solution::queens_attackthe_king(queens, king);
  ans.sort();
  assert_eq!(ans, [[2, 2], [3, 4], [4, 4]]);

  let queens: Vec<Vec<i32>> = [
    [5, 6],
    [7, 7],
    [2, 1],
    [0, 7],
    [1, 6],
    [5, 1],
    [3, 7],
    [0, 3],
    [4, 0],
    [1, 2],
    [6, 3],
    [5, 0],
    [0, 4],
    [2, 2],
    [1, 1],
    [6, 4],
    [5, 4],
    [0, 0],
    [2, 6],
    [4, 5],
    [5, 2],
    [1, 4],
    [7, 5],
    [2, 3],
    [0, 5],
    [4, 2],
    [1, 0],
    [2, 7],
    [0, 1],
    [4, 6],
    [6, 1],
    [0, 6],
    [4, 3],
    [1, 7],
  ]
  .iter()
  .map(|p| p.to_vec())
  .collect();

  let king: Vec<i32> = vec![3, 4];

  let mut ans2 = Solution::queens_attackthe_king(queens, king);
  let mut res2 = [[2, 3], [1, 4], [1, 6], [3, 7], [4, 3], [5, 4], [4, 5]];
  ans2.sort();
  res2.sort();
  assert_eq!(ans2, res2);
}
