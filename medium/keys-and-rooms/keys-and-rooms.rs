use std::collections::HashSet;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut dfs: Vec<i32> = vec![0];
        let mut seen = HashSet::new();
        seen.insert(0);
        while let Some(i) = dfs.pop() {
            for j in &rooms[i as usize] {
                if !seen.contains(j) {
                    dfs.push(*j);
                    seen.insert(*j);
                    if seen.len() == rooms.len() {
                        return true;
                    }
                }
            }
        }
        seen.len() == rooms.len()
    }
}

struct Solution {}

fn main() {
    let rooms = vec![vec![1], vec![2], vec![3], vec![]];
    assert_eq!(Solution::can_visit_all_rooms(rooms), true);
}
