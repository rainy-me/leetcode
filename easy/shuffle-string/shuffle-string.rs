struct Solution {}

impl Solution {
  pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    let mut chars = s.chars().enumerate().collect::<Vec<(usize, char)>>();
    chars.sort_by(|&(a, _), &(b, _)| indices[a].cmp(&indices[b]));
    chars.into_iter().map(|(_, c)| c).collect::<String>()
  }
}

fn main() {
  assert_eq!(
    Solution::restore_string(String::from("codeleet"), vec![4, 5, 6, 7, 0, 2, 1, 3]),
    String::from("leetcode")
  );
  assert_eq!(
    Solution::restore_string(String::from("aiohn"), vec![3, 1, 4, 2, 0]),
    String::from("nihao")
  );
  assert_eq!(
    Solution::restore_string(String::from("aaiougrt"), vec![4, 0, 2, 6, 7, 3, 1, 5]),
    String::from("arigatou")
  );
  assert_eq!(
    Solution::restore_string(String::from("art"), vec![1, 0, 2]),
    String::from("rat")
  );
}
