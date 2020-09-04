struct Solution {}

impl Solution {
  pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let (mut ret, end) = (0, arr.len());
    for i in 0..end {
      for j in i + 1..end {
        if (arr[i] - arr[j]).abs() > a {
          continue;
        };
        for k in j + 1..end {
          // println!("{},{},{}", arr[i], arr[j], arr[k]);
          if (arr[j] - arr[k]).abs() <= b && (arr[i] - arr[k]).abs() <= c {
            // println!("{},{},{} <==", arr[i], arr[j], arr[k]);
            ret += 1;
          }
        }
      }
    }
    ret
  }
}

fn main() {
  assert_eq!(
    Solution::count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3),
    4
  );
  assert_eq!(
    Solution::count_good_triplets(vec![1, 1, 2, 2, 3], 0, 0, 1),
    0
  );
  assert_eq!(
    Solution::count_good_triplets(vec![7, 3, 7, 3, 12, 1, 12, 2, 3], 5, 8, 1),
    12
  );
}
