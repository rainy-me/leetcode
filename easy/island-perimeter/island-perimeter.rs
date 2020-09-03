struct Solution {}

impl Solution {
  pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let mut perimeter = 0;
    let (w, h) = (grid[0].len() - 1, grid.len() - 1);

    for (i, row) in grid.iter().enumerate() {
      for (j, &cell) in row.iter().enumerate() {
        if cell == 0 {
          continue;
        }
        let mut p = 4;
        if i > 0 {
          p -= grid[i - 1][j];
        }
        if j > 0 {
          p -= row[j - 1];
        }
        if i < h {
          p -= grid[i + 1][j];
        }
        if j < w {
          p -= row[j + 1];
        }
        perimeter += p;
      }
    }
    perimeter
  }
}

fn main() {
  assert_eq!(
    Solution::island_perimeter(vec![
      vec![0, 1, 0, 0],
      vec![1, 1, 1, 0],
      vec![0, 1, 0, 0],
      vec![1, 1, 0, 0]
    ]),
    16
  );
}
