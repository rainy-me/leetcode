struct Solution {}

impl Solution {
  pub fn build_array(mut target: Vec<i32>, n: i32) -> Vec<String> {
    target.reverse();
    let mut ret: Vec<String> = Vec::new();
    let mut current = target.pop();
    for i in 1..n + 1 {
      match current {
        Some(x) if x == i => {
          ret.push("Push".to_string());
          current = target.pop();
        }
        Some(_) => {
          ret.push("Push".to_string());
          ret.push("Pop".to_string());
        }
        None => return ret,
      }
    }
    ret
  }
}

fn main() {
  assert_eq!(
    Solution::build_array(vec![1, 3], 3),
    ["Push", "Push", "Pop", "Push"]
      .iter()
      .map(|v| v.to_string())
      .collect::<Vec<String>>()
  );

  assert_eq!(
    Solution::build_array(vec![1, 2, 3], 3),
    ["Push", "Push", "Push"]
      .iter()
      .map(|v| v.to_string())
      .collect::<Vec<String>>()
  );

  assert_eq!(
    Solution::build_array(vec![1, 2], 4),
    ["Push", "Push"]
      .iter()
      .map(|v| v.to_string())
      .collect::<Vec<String>>()
  );

  assert_eq!(
    Solution::build_array(vec![2, 3, 4], 4),
    ["Push", "Pop", "Push", "Push", "Push"]
      .iter()
      .map(|v| v.to_string())
      .collect::<Vec<String>>()
  );
}
