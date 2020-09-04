struct Solution {}
use std::collections::HashSet;
impl Solution {
  pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
    let mut pending: HashSet<usize> = HashSet::new();
    for j in 0..prices.len() {
      pending.retain(|&i| {
        if !(j > i && prices[j] <= prices[i]) {
          return true;
        }
        prices[i] -= prices[j];
        false
      });
      pending.insert(j);
    }
    prices
  }
}
fn main() {
  assert_eq!(
    Solution::final_prices(vec![8, 4, 6, 2, 3]),
    vec![4, 2, 4, 2, 3]
  );
  assert_eq!(
    Solution::final_prices(vec![1, 2, 3, 4, 5]),
    vec![1, 2, 3, 4, 5]
  );
  assert_eq!(Solution::final_prices(vec![10, 1, 1, 6]), vec![9, 0, 1, 6]);
}
