#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn _count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut m = vec![0; 46];
        for mut n in low_limit..=high_limit {
            let mut no = 0;
            while n != 0 {
                no += n % 10;
                n /= 10;
            }
            m[no as usize] += 1;
        }
        *m.iter().max().unwrap()
    }

    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let slots = {
            if high_limit > 10000 {
                46
            } else if high_limit > 1000 {
                37
            } else {
                28
            }
        };
        let (mut low, mut id) = (low_limit, 0);
        let mut m = vec![0; slots];
        while low != 0 {
            id += low % 10;
            low /= 10;
        }
        m[id as usize] += 1;
        for mut n in (low_limit + 1)..=high_limit {
            while n % 10 == 0 {
                id -= 9;
                n /= 10;
            }
            id += 1;
            m[id as usize] += 1
        }
        *m.iter().max().unwrap()
    }
}

fn main() {}

#[test]
fn test() {
    for (i, o) in vec![((1, 10), 2), ((5, 15), 2), ((19, 28), 2)] {
        assert_eq!(Solution::count_balls(i.0, i.1), o);
    }
}
