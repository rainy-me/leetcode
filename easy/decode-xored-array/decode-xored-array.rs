#[cfg(test)]
use std::ptr;

#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn decode(mut encoded: Vec<i32>, mut first: i32) -> Vec<i32> {
        for n in encoded.as_mut_slice() {
            unsafe {
                ptr::swap(n, &mut first);
            }
            first ^= *n;
        }
        encoded.push(first);
        encoded
    }

    pub fn _decode_origin(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut res = vec![first];
        for a in encoded {
            res.push(res.last().unwrap() ^ a)
        }
        res
    }
}

fn main() {
    assert_eq!(0, 0);
}

#[test]
fn test() {
    assert_eq!(Solution::decode(vec![1, 2, 3], 1), vec![1, 0, 2, 1]);
    assert_eq!(Solution::decode(vec![6, 2, 7, 3], 4), vec![4, 2, 0, 7, 4]);
}
