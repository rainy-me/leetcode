#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let (_, max) = rectangles.iter().fold((1, 0), |mut acc, curr| {
            let len = curr[0].min(curr[1]);
            if len > acc.0 {
                (len, 1)
            } else {
                if len == acc.0 {
                    acc.1 += 1;
                }
                acc
            }
        });
        max
    }
}

fn main() {}

#[test]
fn test() {
    for (i, o) in vec![
        (vec![vec![5, 8], vec![3, 9], vec![5, 12], vec![16, 5]], 3),
        (vec![vec![2, 3], vec![3, 7], vec![4, 3], vec![3, 7]], 3),
    ] {
        assert_eq!(Solution::count_good_rectangles(i), o);
    }
}
