#![feature(fn_traits)]

#[cfg(test)]
use utils::mat;
use utils::setup;
setup!();

#[cfg(test)]
impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let (h, w) = (mat.len(), mat[0].len());
        let x = (0..w)
            .map(|i| mat.iter().map(|row| row[i]).sum())
            .collect::<Vec<i32>>();
        let y = (0..h).map(|i| mat[i].iter().sum()).collect::<Vec<i32>>();
        let mut ret = 0;
        for row in 0..h {
            for cell in 0..w {
                if mat[row][cell] == 1 && x[cell] == 1 && y[row] == 1 {
                    ret += 1;
                }
            }
        }
        ret
    }
}

test! {
    num_special,
    (mat![[0,0],
          [0,0],
          [1,0]],
    ) => 1;
    (mat![[0,0,1,0],
          [0,0,0,0],
          [0,0,0,0],
          [0,1,0,0]],
    ) => 2;
    (mat![[1,0,0],
          [0,0,1],
          [1,0,0]],
    ) => 1;
    (mat![[1,0,0],
          [0,1,0],
          [0,0,1]],
    ) => 3;
    (mat![[0,0,0,1],
          [1,0,0,0],
          [0,1,1,0],
          [0,0,0,0]],
    ) => 2;
    (mat![[0,0,0,0,0],
          [1,0,0,0,0],
          [0,1,0,0,0],
          [0,0,1,0,0],
          [0,0,0,1,1]],
    ) => 3;
}
