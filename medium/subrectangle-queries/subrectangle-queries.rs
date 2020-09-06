struct SubrectangleQueries {
  rectangle: Vec<Vec<i32>>,
  updates: Vec<(i32, i32, i32, i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SubrectangleQueries {
  fn new(rectangle: Vec<Vec<i32>>) -> Self {
    SubrectangleQueries {
      rectangle,
      updates: Vec::new(),
    }
  }
  fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
    self.updates.push((row1, col1, row2, col2, new_value))
  }

  fn get_value(&self, row: i32, col: i32) -> i32 {
    for &(r1, c1, r2, c2, val) in self.updates.iter().rev() {
      if r1 <= row && row <= r2 && c1 <= col && col <= c2 {
        return val;
      }
    }
    self.rectangle[row as usize][col as usize]
  }
}

/**
 * Your SubrectangleQueries object will be instantiated and called as such:
 * let obj = SubrectangleQueries::new(rectangle);
 * obj.update_subrectangle(row1, col1, row2, col2, newValue);
 * let ret_2: i32 = obj.get_value(row, col);
 */
fn main() {
  assert_eq!(0, 0);
}
