#![feature(fn_traits)]

utils::setup!(
    use std::collections::HashMap;
);

#[cfg(test)]
impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut m = HashMap::new();
        let mut current = (0, 0);
        m.insert(current, 1);
        for d in path.chars() {
            let delta = match d {
                'N' => (0, 1),
                'S' => (0, -1),
                'E' => (1, 0),
                'W' => (-1, 0),
                _ => unreachable!(),
            };
            current = (current.0 + delta.0, current.1 + delta.1);
            match m.get(&current) {
                Some(_) => {
                    return true;
                }
                _ => {
                    m.insert(current, 1);
                }
            }
        }
        false
    }
}

test! {
    is_path_crossing,
    ("SN".to_string(),) => true;
    ("NES".to_string(),) => false;
    ("NESWW".to_string(),) => true;
}
