#![feature(fn_traits)]

use utils::setup;

setup!();

#[cfg(test)]
impl Solution {
    pub fn decrypt(mut code: Vec<i32>, k: i32) -> Vec<i32> {
        let len = code.len();
        if k == 0 {
            return vec![0; len];
        }
        let sum = code.iter().sum::<i32>();
        code.append(&mut code.clone());
        (0..len)
            .map(|index| {
                let sums = (k / len as i32) * sum;
                let diff = (k % len as i32).abs() as usize;
                let range = {
                    if k > 0 {
                        index + 1..index + 1 + diff
                    } else {
                        len + index - diff..len + index
                    }
                };
                let rest = code[range].iter().sum::<i32>();
                sums + rest
            })
            .collect()
    }
}

test! {
    decrypt,
    (vec![10,5,7,7,3,2,10,3,6,9,1,6], -4) => vec![22,26,22,28,29,22,19,22,18,21,28,19];
    (vec![5,7,1,4], 3) => vec![12,10,16,13];
    (vec![1,2,3,4], 0) => vec![0,0,0,0];
    (vec![2,4,9,3], -2) => vec![12,5,6,13];
}
