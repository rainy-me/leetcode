#[cfg(test)]
use utils::mat;

#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut sum = 0;
        let w = mat.len() - 1;
        for y in 0..=w {
            for x in 0..=w {
                if x == y || w - x == y {
                    sum += mat[x][y]
                }
            }
        }
        sum
    }
}

fn main() {}

#[test]
fn test() {
    for (i, o) in vec![
        (
            mat![
                [1, 2, 3];
                [4, 5, 6];
                [7, 8, 9]
            ],
            25,
        ),
        (
            mat![
             [1, 1, 1, 1];
             [1, 1, 1, 1];
             [1, 1, 1, 1];
             [1, 1, 1, 1]
            ],
            8,
        ),
        (mat![[5]], 5),
    ] {
        assert_eq!(Solution::diagonal_sum(i), o);
    }
}
