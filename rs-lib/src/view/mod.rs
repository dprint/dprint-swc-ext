mod custom;
mod types;

#[cfg(test)]
mod test_helpers;

#[allow(clippy::all)]
#[rustfmt::skip]
mod generated;

pub use generated::*;
pub use types::*;

#[cfg(test)]
mod test {
  use crate::common::SourceRanged;
  use crate::common::SourceTextInfo;
  use crate::swc::ast::Module;
  use crate::swc::common::comments::SingleThreadedCommentsMapInner;
  use crate::swc::parser::token::TokenAndSpan;
  use crate::view::test_helpers::*;
  use crate::view::*;

  #[test]
  fn program_info_provider() {
    struct MyParsedSource {
      module: Module,
      text_info: SourceTextInfo,
      tokens: Vec<TokenAndSpan>,
      leading: SingleThreadedCommentsMapInner,
      trailing: SingleThreadedCommentsMapInner,
    }

    impl ProgramInfoProvider for MyParsedSource {
      fn program_info(&self) -> ProgramInfo<'_> {
        ProgramInfo {
          program: ProgramRef::Module(&self.module),
          text_info: Some(&self.text_info),
          tokens: Some(&self.tokens),
          comments: Some(Comments {
            leading: &self.leading,
            trailing: &self.trailing,
          }),
        }
      }
    }

    let (module, tokens, text_info, comments) = crate::test_helpers::get_swc_module(
      std::path::Path::new("test.ts"),
      "// 1
1 + 1",
    );
    let (leading, trailing) = comments.take_all();
    let source = MyParsedSource {
      module,
      text_info,
      tokens,
      leading: std::rc::Rc::try_unwrap(leading).unwrap().into_inner(),
      trailing: std::rc::Rc::try_unwrap(trailing).unwrap().into_inner(),
    };

    source.with_view(|program| {
      let children = program.children();
      assert_eq!(children.len(), 1);
      assert_eq!(children[0].text_fast(program), "1 + 1");
      assert_eq!(children[0].start().leading_comments_fast(program).count(), 1);
    });
  }

  #[test]
  fn trailing_comments_start_of_file_no_match() {
    run_test(r#"5 // test"#, |program| {
      // previously there was a bug here where it would return the
      // comments after the token
      let trailing_comments = program.range().start.trailing_comments_fast(program);
      assert!(trailing_comments.is_empty());
    });
  }
}
