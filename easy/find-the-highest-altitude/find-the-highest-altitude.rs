#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let (_, max) = gain.iter().fold((0, 0), |mut acc, curr| {
            acc.0 += curr;
            (acc.0, acc.1.max(acc.0))
        });
        max
    }
}

fn main() {}

#[test]
fn test() {
    for (i, o) in vec![
        (vec![-5, 1, 5, 0, -7], 1),
        (vec![-4, -3, -2, -1, 4, 3, 2], 0),
    ] {
        assert_eq!(Solution::largest_altitude(i), o);
    }
}
