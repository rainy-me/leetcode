#![feature(fn_traits)]
use utils::test;
#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn count_students(mut students: Vec<i32>, mut sandwiches: Vec<i32>) -> i32 {
        sandwiches.reverse();
        loop {
            let start_len = students.len();
            students.retain(|prefer| {
                let sandwich_on_top = sandwiches.last().unwrap();
                let will_take = prefer == sandwich_on_top;
                if will_take {
                    sandwiches.pop();
                }
                !will_take
            });
            let end_len = students.len();
            if start_len == end_len {
                break end_len as i32;
            }
        }
    }
}

fn main() {}

test!(
    count_students,
    (vec![1, 1, 0, 0], vec![0, 1, 0, 1]) => 0;
    (vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]) => 3;
);
