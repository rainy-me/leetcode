impl Solution {
  pub fn remove_duplicates(s: String, k: i32) -> String {
    let mut stack: Vec<(char, i32)> = vec![("#".chars().next().unwrap(), 0)];
    for c in s.chars() {
      let (ref mut last_c, ref mut count) = stack.last_mut().unwrap();
      if *last_c == c {
        *count += 1;
        if *count == k {
          stack.pop();
        }
      } else {
        stack.push((c, 1))
      }
    }
    stack.iter().fold("".to_string(), |mut ans, &(c, count)| {
      ans.push_str(&c.to_string().repeat(count as usize));
      ans
    })
  }
}

struct Solution {}
fn main() {
  assert_eq!(
    Solution::remove_duplicates("abcd".to_string(), 2),
    "abcd".to_string()
  );

  assert_eq!(
    Solution::remove_duplicates("deeedbbcccbdaa".to_string(), 3),
    "aa".to_string()
  );

  assert_eq!(
    Solution::remove_duplicates("pbbcggttciiippooaais".to_string(), 2),
    "ps".to_string()
  );
}
