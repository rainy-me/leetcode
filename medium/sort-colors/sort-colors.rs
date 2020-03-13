struct Solution {}

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut red = 0;
        let mut white = 0;
        let mut blue = nums.len() - 1;
        while white <= blue {
            match nums[white] {
                0 => {
                    nums.swap(white, red);
                    white += 1;
                    red += 1;
                }
                1 => {
                    white += 1;
                }
                _ => {
                    nums.swap(white, blue);
                    match blue {
                        0 => break,
                        _ => blue -= 1,
                    };
                }
            }
        }
    }
}

fn main() {
    let mut i: Vec<i32> = vec![2, 0, 2, 1, 1, 0];
    let o = [0, 0, 1, 1, 2, 2];
    Solution::sort_colors(&mut i);
    assert_eq!(i, o);

    let mut ii: Vec<i32> = vec![2];
    let oo = [2];
    Solution::sort_colors(&mut ii);
    assert_eq!(ii, oo);

    let mut iii: Vec<i32> = vec![2, 0, 1];
    let ooo = [0, 1, 2];
    Solution::sort_colors(&mut iii);
    assert_eq!(iii, ooo);
}
