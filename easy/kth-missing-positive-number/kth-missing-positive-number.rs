#![feature(fn_traits)]

utils::setup!();

#[cfg(test)]
impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut slice = &arr[..];
        for n in 1..i32::MAX {
            if slice.len() > 0 && n == slice[0] {
                slice = &slice[1..]
            } else {
                k -= 1;
                if k == 0 {
                    return n;
                }
            }
        }
        0
    }
}

test! {
    find_kth_positive,
    (vec![2,3,4,7,11],5) => 9;
    (vec![1,2,3,4],2) => 6;
}
