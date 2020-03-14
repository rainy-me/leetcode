struct Solution {}

impl Solution {
  pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
    if 0 <= start && start < (arr.len() as i32) && arr[start as usize] >= 0 {
      let mut arr_clone = arr.clone();
      arr_clone[start as usize] *= -1;
      let arr_clone_start = arr_clone[start as usize];

      arr_clone[start as usize] == 0
        || Solution::can_reach(arr_clone.clone(), start + arr_clone_start)
        || Solution::can_reach(arr_clone.clone(), start - arr_clone_start)
    } else {
      false
    }
  }
}

fn main() {
  assert_eq!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5), true);
}
