use crate::swc::parser::token::TokenAndSpan;

use super::comments::CommentContainer;
use super::text_info::*;
use super::tokens::TokenContainer;

/// A Module or Script node.
pub trait RootNode<'a> {
  fn maybe_text_info(&self) -> Option<&'a SourceTextInfo>;
  fn maybe_token_container(&self) -> Option<&'a TokenContainer<'a>>;
  fn maybe_comment_container(&self) -> Option<&'a CommentContainer<'a>>;

  fn token_at_index(&self, index: usize) -> Option<&'a TokenAndSpan> {
    self.token_container().get_token_at_index(index)
  }

  fn token_container(&self) -> &'a TokenContainer<'a> {
    self
      .maybe_token_container()
      .as_ref()
      .expect("The tokens must be provided to `with_view` in order to use this method.")
  }

  fn comment_container(&self) -> &'a CommentContainer<'a> {
    self
      .maybe_comment_container()
      .as_ref()
      .expect("The comments must be provided to `with_view` in order to use this method.")
  }
}

impl<'a, T> SourceTextInfoProvider<'a> for T
where
  T: RootNode<'a>,
{
  fn text_info(&self) -> &'a SourceTextInfo {
    self
      .maybe_text_info()
      .expect("The source file must be provided to `with_view` in order to use this method.")
  }
}
