struct Solution {}

impl Solution {
  pub fn print_vertically(s: String) -> Vec<String> {
    let mut ans: Vec<String> = Vec::new();
    for (i, w) in s
      .split_whitespace()
      .collect::<Vec<&str>>()
      .iter()
      .enumerate()
    {
      for (j, c) in w.chars().enumerate() {
        if j >= ans.len() {
          ans.push(format!("{:1$}", c, i));
        } else {
          let l = i - ans[j].len();
          ans[j].push_str(&format!("{:1$}", c, l));
        }
      }
    }
    ans
  }
}

fn main() {
  let ans = Solution::print_vertically(String::from("CONTEST IS COMING"));
  print!("{:?}", ans);
  assert_eq!(ans, vec!["CIC", "OSO", "NM", "TI", "EN", "SG", "T"])
}
