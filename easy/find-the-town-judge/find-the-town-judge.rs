use std::collections::HashSet;
impl Solution {
  pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let mut arr: Vec<i32> = vec![0; (n + 1) as usize];
    let mut s = HashSet::new();
    for t in trust.iter() {
      s.insert(t[0]);
      arr[t[1] as usize] += 1;
    }
    for i in 1..n + 1 {
      if arr[i as usize] == n - 1 && s.get(&i).is_none() {
        return i;
      }
    }
    -1
  }
}

impl Solution {
  pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    if trust.len() == 0 {
      return 1;
    }
    let mut arr: Vec<i32> = vec![0; (n + 1) as usize];
    for t in trust.iter() {
      arr[t[0] as usize] -= 1;
      arr[t[1] as usize] += 1;
    }
    match arr.iter().position(|&p| p == n - 1) {
      Some(index) => index as i32,
      _ => -1,
    }
  }
}

struct Solution {}

fn main() {
  assert_eq!(Solution::find_judge(1, vec![]), 1);
  assert_eq!(Solution::find_judge(2, vec![vec![1, 2]]), 2);
  assert_eq!(Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
  assert_eq!(
    Solution::find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]),
    -1
  );
  assert_eq!(Solution::find_judge(3, vec![vec![1, 2], vec![2, 3]]), -1);
  assert_eq!(
    Solution::find_judge(
      4,
      vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![4, 3]]
    ),
    3
  );
}

//class Solution:
// def findJudge(self, N: int, trust: List[List[int]]) -> int:
//     arr = [0] * (N+1)
//     m = set()
//     for [a, b] in trust:
//         m.add(a)
//         arr[b] += 1
//     for i, n in enumerate(arr):
//         if i == 0:
//             continue
//         if n == N-1 and i not in m:
//             return i
//     return -1
