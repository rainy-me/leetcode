#![feature(fn_traits)]

#[macro_export]
macro_rules! strings {
  ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[macro_export]
macro_rules! mat {
  (
    $(
      [$($y:expr),*]
    );*
  ) => (vec![
    $(
      vec![$($y),*]
    ),*
  ]);
  (
    $(
      [$($y:expr),*]
    ),*
  ) => (vec![
    $(
      vec![$($y),*]
    ),*
  ]);
}

#[macro_export]
macro_rules! test {
  (
    $fn:ident,
    $(
      ($($args:tt)*) => $result:expr
    );*;
  ) => {
    #[test]
    fn test() {
      $(
        assert_eq!(
          std::ops::Fn::call(&Solution::$fn, ($($args)*)),
          $result
        );
      )*
    }
  };
}

#[macro_export]
macro_rules! setup {
    () => {
        use utils::test;

        #[cfg(test)]
        struct Solution {}

        fn main() {}
    };
    ($($item:item)*) => {
      $(
        #[cfg(test)]
        $item
      )*
      utils::setup!();
    };
}
