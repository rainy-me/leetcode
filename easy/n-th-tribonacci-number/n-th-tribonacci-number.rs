use std::collections::HashMap;
impl Solution {
  pub fn tribonacci(n: i32) -> i32 {
    match n {
      0 => 0,
      1 => 1,
      2 => 1,
      _ => {
        let mut a = 0;
        let mut b = 1;
        let mut c = 1;
        let mut d = 0;
        for i in (0..n - 2) {
          d = a + b + c;
          a = b;
          b = c;
          c = d;
        }
        d
      }
    }
  }
}
struct Solution {}

fn main() {
  assert_eq!(0, 0);
}
