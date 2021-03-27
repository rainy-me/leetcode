#[cfg(test)]
use utils::matrix;

#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        box_types.sort_unstable_by_key(|box_type| -box_type[1]);
        let mut sum = 0;
        for box_type in box_types {
            let (num, size) = (box_type[0], box_type[1]);
            if truck_size > num {
                sum += num * size;
                truck_size -= num;
            } else {
                sum += truck_size * size;
                return sum;
            }
        }
        sum
    }
}

fn main() {}

#[test]
fn test() {
    for (i, o) in vec![
        ((matrix![[1, 3], [2, 2], [3, 1]], 4), 8),
        ((matrix![[5, 10], [2, 5], [4, 7], [3, 9]], 10), 91),
    ] {
        assert_eq!(Solution::maximum_units(i.0, i.1), o);
    }
}
