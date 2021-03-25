#[cfg(test)]
use utils::vec_of_strings;

#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        arr.iter().enumerate().fold(0, |acc, (index, curr)| {
            acc + curr * (((index + 1) * (arr.len() - index) + 1) / 2) as i32
        })
    }
}

fn main() {}

#[test]
fn test() {
    assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
    assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 2]), 3);
    assert_eq!(Solution::sum_odd_length_subarrays(vec![10, 11, 12]), 66);
}
