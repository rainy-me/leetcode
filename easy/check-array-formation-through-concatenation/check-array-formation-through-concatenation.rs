#![feature(fn_traits)]

utils::setup!(
    use utils::mat;
    use std::collections::HashMap;
);

#[cfg(test)]
impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let mut search_map = HashMap::new();
        for piece in pieces {
            search_map.insert(piece[0], piece);
        }
        let mut result: Vec<i32> = vec![];
        for n in &arr[..] {
            result.extend(search_map.get(&n).unwrap_or(&vec![]));
        }
        result == arr
    }
}

test! {
    can_form_array,
    (vec![85],mat![[85]]) => true;
    (vec![15,88], mat![[88],[15]]) => true;
    (vec![49,18,16], mat![[16,18,49]]) => false;
    (vec![91,4,64,78], mat![[78],[4,64],[91]]) => true;
    (vec![1,3,5,7], mat![[2,4,6,8]]) => false;
}
