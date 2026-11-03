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
  fn leading_and_trailing_comments_include_the_comments_around_tokens() {
    use crate::common::RootNode;
    use crate::common::SourcePos;
    use crate::common::SourceRangedForSpanned;
    use crate::common::SourceTextInfoProvider;

    let text = concat!(
      "// a\n",
      "/* b */ const /* c */ x /* d */ = // e\n",
      "  [1, /* f */ 2] /* g */; // h\n",
      "/* i */\n",
      "function f(/* j */) {} // k\n",
      "`${y /* l */}`;\n",
      "// m",
    );
    run_test(text, |program| {
      let tokens = program.token_container().tokens;
      let container = program.comment_container();
      let text_range = program.text_info().range();
      let comments_at = |comments: &SingleThreadedCommentsMapInner, pos: SourcePos| {
        comments
          .get(&pos.as_byte_pos())
          .map(|c| c.iter().map(|c| c.span).collect::<Vec<_>>())
          .unwrap_or_default()
      };
      for (index, token) in tokens.iter().enumerate() {
        // the trailing comments of the previous token followed by the leading comments of this one
        let previous_end = if index == 0 {
          text_range.start.as_source_pos()
        } else {
          tokens[index - 1].end()
        };
        let mut expected = comments_at(container.trailing, previous_end);
        expected.extend(comments_at(container.leading, token.start()));
        let actual = token.start().leading_comments_fast(program).map(|c| c.span).collect::<Vec<_>>();
        assert_eq!(actual, expected, "leading comments of token {index}");

        // the trailing comments of this token followed by the leading comments of the next one
        let next_start = tokens.get(index + 1).map(|t| t.start()).unwrap_or(text_range.end);
        let mut expected = comments_at(container.trailing, token.end());
        expected.extend(comments_at(container.leading, next_start));
        let actual = token.end().trailing_comments_fast(program).map(|c| c.span).collect::<Vec<_>>();
        assert_eq!(actual, expected, "trailing comments of token {index}");
      }
      // every comment is found at a token
      let found = tokens
        .iter()
        .flat_map(|t| t.start().leading_comments_fast(program).chain(t.end().trailing_comments_fast(program)))
        .map(|c| c.span)
        .collect::<std::collections::HashSet<_>>();
      assert_eq!(found.len(), 13);
      // the end of the file only has its own leading comments
      assert_eq!(
        text_range.end.leading_comments_fast(program).count(),
        comments_at(container.leading, text_range.end).len()
      );
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
