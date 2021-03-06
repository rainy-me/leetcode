#[cfg(test)]
use utils::strings;

#[cfg(test)]
struct OrderedStream {
    ptr: usize,
    buffer: Vec<Option<String>>,
}

#[cfg(test)]
impl OrderedStream {
    fn new(n: i32) -> Self {
        Self {
            ptr: 1,
            buffer: vec![None; (n + 1) as usize],
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        self.buffer[id_key as usize] = Some(value);
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
        (3, "ccccc", strings![]),
        (1, "aaaaa", strings!["aaaaa"]),
        (2, "bbbbb", strings!["bbbbb", "ccccc"]),
        (5, "eeeee", strings![]),
        (4, "ddddd", strings!["ddddd", "eeeee"]),
    ] {
        assert_eq!(os.insert(id_key, value.to_string()), expect)
    }
}
