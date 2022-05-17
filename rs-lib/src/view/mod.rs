mod custom;
mod types;

#[cfg(test)]
mod test_helpers;

#[allow(clippy::all)]
#[rustfmt::skip]
mod generated;

pub use custom::*;
pub use generated::*;
pub use types::*;

#[cfg(test)]
mod test {
  use crate::common::SourceRanged;
  use crate::view::test_helpers::*;

  #[test]
  fn trailing_comments_start_of_file_no_match() {
    run_test(r#"5 // test"#, |program| {
      // previously there was a bug here where it would return the
      // comments after the token
      let trailing_comments = program.range().start.trailing_comments_fast(&program);
      assert!(trailing_comments.is_empty());
    });
  }
}
