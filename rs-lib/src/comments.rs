use crate::tokens::*;
use std::cell::Ref;
use swc_common::{
  comments::{Comment, SingleThreadedComments, SingleThreadedCommentsMapInner},
  BytePos,
};

pub struct CommentContainer<'a> {
  leading: Ref<'a, SingleThreadedCommentsMapInner>,
  trailing: Ref<'a, SingleThreadedCommentsMapInner>,
  tokens: &'a TokenContainer<'a>,
  source_file: &'a swc_common::SourceFile,
}

impl<'a> CommentContainer<'a> {
  pub fn new(
    comments: &'a SingleThreadedComments,
    tokens: &'a TokenContainer<'a>,
    source_file: &'a swc_common::SourceFile,
  ) -> Self {
    let (leading, trailing) = comments.borrow_all();
    CommentContainer {
      leading,
      trailing,
      tokens,
      source_file,
    }
  }

  pub fn leading_comments(&self, lo: BytePos) -> Vec<&'a Comment> {
    let previous_token_hi = self.tokens.get_previous_token_hi(lo).unwrap_or(BytePos(0));
    let trailing = self.get_trailing(previous_token_hi);
    let leading = self.get_leading(lo);
    combine_comment_vecs(trailing, leading)
  }

  pub fn trailing_comments(&self, hi: BytePos) -> Vec<&'a Comment> {
    let next_token_lo = self
      .tokens
      .get_next_token_lo(hi)
      .unwrap_or(self.source_file.end_pos);
    let trailing = self.get_trailing(hi);
    let leading = self.get_leading(next_token_lo);
    combine_comment_vecs(trailing, leading)
  }

  fn get_leading(&self, lo: BytePos) -> Option<&'a Vec<Comment>> {
    let leading = self.leading.get(&lo);
    // todo: how to not do this?
    let leading =
      unsafe { std::mem::transmute::<Option<&Vec<Comment>>, Option<&'a Vec<Comment>>>(leading) };
    leading
  }

  fn get_trailing(&self, hi: BytePos) -> Option<&'a Vec<Comment>> {
    let trailing = self.trailing.get(&hi);
    // todo: how to not do this?
    let trailing =
      unsafe { std::mem::transmute::<Option<&Vec<Comment>>, Option<&'a Vec<Comment>>>(trailing) };
    trailing
  }
}

fn combine_comment_vecs<'a>(
  a: Option<&'a Vec<Comment>>,
  b: Option<&'a Vec<Comment>>,
) -> Vec<&'a Comment> {
  let length = a.map(|t| t.len()).unwrap_or(0) + b.map(|t| t.len()).unwrap_or(0);
  let mut comments = Vec::with_capacity(length);
  if let Some(a) = a {
    comments.extend(a);
  }
  if let Some(b) = b {
    comments.extend(b);
  }
  comments
}
