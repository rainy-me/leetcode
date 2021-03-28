#![feature(fn_traits)]

#[cfg(test)]
use utils::{mat, test};

#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        points
            .into_iter()
            .enumerate()
            .fold((i32::MAX, -1), |acc, (index, point)| {
                let (xx, yy) = unsafe { (*point.get_unchecked(0), *point.get_unchecked(1)) };
                let (distance, _) = acc;
                if x == xx || y == yy {
                    let point_distance = (xx - x).abs() + (yy - y).abs();
                    if point_distance < distance {
                        return (point_distance, index as i32);
                    }
                }
                acc
            })
            .1
    }
}

fn main() {}

#[cfg(test)]
test! {
    nearest_valid_point,
    (3, 4, mat![[3,4]]) => 0;
    (3, 4, mat![[2,3]]) => -1;
    (3, 4, mat![[1,2],[3,1],[2,4],[2,3],[4,4]]) => 2;
}
