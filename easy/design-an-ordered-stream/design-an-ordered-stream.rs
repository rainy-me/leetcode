use std::ops::Index;

#[cfg(test)]
use utils::vec_of_strings;

#[cfg(test)]
struct Solution {}

#[cfg(test)]
struct OrderedStream {
    ptr: usize,
    buffer: Vec<Option<String>>,
}

#[cfg(test)]
impl OrderedStream {
    fn new(n: i32) -> Self {
        Self {
            ptr: 0,
            buffer: vec![None; n as usize],
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        let key = (id_key - 1) as usize;
        self.buffer[key] = Some(value);
        let new_ptr = self.ptr
            + self.buffer[self.ptr..self.buffer.len()]
                .iter()
                .position(|item| item.is_none())
                .unwrap_or(self.buffer.len() - self.ptr);
        let available = self.buffer[self.ptr..new_ptr]
            .iter_mut()
            .map(|item| item.take().unwrap())
            .collect();
        self.ptr = new_ptr;
        return available;
    }
}

/**
 * Your OrderedStream object will be instantiated and called as such:
 * let obj = OrderedStream::new(n);
 * let ret_1: Vec<String> = obj.insert(idKey, value);
 */

fn main() {}

#[test]
fn test() {
    let mut os = OrderedStream::new(5);
    for (id_key, value, expect) in vec![
        (3, "ccccc", vec_of_strings![]),
        (1, "aaaaa", vec_of_strings!["aaaaa"]),
        (2, "bbbbb", vec_of_strings!["bbbbb", "ccccc"]),
        (5, "eeeee", vec_of_strings![]),
        (4, "ddddd", vec_of_strings!["ddddd", "eeeee"]),
    ] {
        assert_eq!(os.insert(id_key, value.to_string()), expect)
    }
}
