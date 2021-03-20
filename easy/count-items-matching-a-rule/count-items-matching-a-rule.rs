#[cfg(test)]
use utils::vec_of_strings;

#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let key = match rule_key.as_str() {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => unreachable!(),
        };
        items.iter().fold(0, |acc, item| {
            if &item[key] == &rule_value {
                acc + 1
            } else {
                acc
            }
        })
    }
}

fn main() {}

#[test]
fn test() {
    assert_eq!(
        Solution::count_matches(
            vec![
                vec_of_strings!["phone", "blue", "pixel"],
                vec_of_strings!["computer", "silver", "lenovo"],
                vec_of_strings!["phone", "gold", "iphone"]
            ],
            "color".to_string(),
            "silver".to_string()
        ),
        1
    );
    assert_eq!(
        Solution::count_matches(
            vec![
                vec_of_strings!["phone", "blue", "pixel"],
                vec_of_strings!["computer", "silver", "phone"],
                vec_of_strings!["phone", "gold", "iphone"]
            ],
            "type".to_string(),
            "phone".to_string()
        ),
        2
    );
}
