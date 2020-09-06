struct Solution {}

impl Solution {
  pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
    for (i, word) in sentence.split_whitespace().enumerate() {
      if word.starts_with(&search_word) {
        return i as i32 + 1;
      }
    }
    -1
  }
}

fn main() {
  assert_eq!(
    Solution::is_prefix_of_word("i love eating burger".to_string(), "burg".to_string()),
    4
  );
  assert_eq!(
    Solution::is_prefix_of_word(
      "this problem is an easy problem".to_string(),
      "pro".to_string()
    ),
    2
  );
  assert_eq!(
    Solution::is_prefix_of_word("i am tired".to_string(), "you".to_string()),
    -1
  );
  assert_eq!(
    Solution::is_prefix_of_word("i use triple pillow".to_string(), "pill".to_string()),
    4
  );
  assert_eq!(
    Solution::is_prefix_of_word("hello from the other side".to_string(), "they".to_string()),
    -1
  );
}
