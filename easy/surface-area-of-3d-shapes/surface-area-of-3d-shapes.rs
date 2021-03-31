#![feature(fn_traits)]

utils::setup!(
    use utils::mat;
);

#[cfg(test)]
impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut ret = 0;
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] > 0 {
                    ret += grid[i][j] * 4 + 2;
                }
                if i > 0 {
                    ret -= grid[i][j].min(grid[i - 1][j]) * 2
                }
                if j > 0 {
                    ret -= grid[i][j].min(grid[i][j - 1]) * 2
                }
            }
        }
        ret
    }
}

test! {
    surface_area,
    (mat![[2]],) => 10;
    (mat![[1,2],[3,4]],) => 34;
    (mat![[1,0],[0,2]],) => 16;
    (mat![[1,1,1],[1,0,1],[1,1,1]],) => 32;
    (mat![[2,2,2],[2,1,2],[2,2,2]],) => 46;
}
