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

  pub fn leading_comments(&self, lo: BytePos) -> CommentsIterator<'a> {
    let previous_token_hi = self.tokens.get_previous_token_hi(lo).unwrap_or(BytePos(0));
    let trailing = self.get_trailing(previous_token_hi);
    let leading = self.get_leading(lo);
    combine_comment_vecs(trailing, leading)
  }

  pub fn trailing_comments(&self, hi: BytePos) -> CommentsIterator<'a> {
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
) -> CommentsIterator<'a> {
  let length = if a.is_some() { 1 } else { 0 } + if b.is_some() { 1 } else { 0 };
  let mut comment_vecs = Vec::with_capacity(length);
  if let Some(a) = a {
    comment_vecs.push(a);
  }
  if let Some(b) = b {
    comment_vecs.push(b);
  }
  CommentsIterator::new(comment_vecs)
}

#[derive(Clone)]
pub struct CommentsIterator<'a> {
  comment_vecs: Vec<&'a Vec<Comment>>,
  outer_index: usize,
  inner_index: usize,
}

impl<'a> CommentsIterator<'a> {
  pub fn new(comment_vecs: Vec<&'a Vec<Comment>>) -> CommentsIterator<'a> {
    CommentsIterator {
      comment_vecs,
      outer_index: 0,
      inner_index: 0,
    }
  }

  pub fn reset(&mut self) {
    self.outer_index = 0;
    self.inner_index = 0;
  }

  pub fn extend(&mut self, iterator: CommentsIterator<'a>) {
    self.comment_vecs.extend(iterator.comment_vecs);
  }

  pub fn is_empty(&self) -> bool {
    for comments in self.comment_vecs.iter() {
      if !comments.is_empty() {
        return false;
      }
    }

    true
  }

  pub fn peek_last_comment(&self) -> Option<&'a Comment> {
    if let Some(comments) = self.comment_vecs.last() {
      comments.last()
    } else {
      None
    }
  }
}

impl<'a> Iterator for CommentsIterator<'a> {
  type Item = &'a Comment;

  fn next(&mut self) -> Option<&'a Comment> {
    loop {
      if let Some(comments) = self.comment_vecs.get(self.outer_index) {
        if let Some(comment) = comments.get(self.inner_index) {
          self.inner_index += 1;
          return Some(comment);
        } else {
          self.inner_index = 0;
          self.outer_index += 1;
        }
      } else {
        return None;
      }
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let mut next_inner_index = self.inner_index;
    let mut count = 0;
    for comment_vec in &self.comment_vecs[self.outer_index..] {
      count += comment_vec.len() - next_inner_index;
      next_inner_index = 0;
    }
    (count, Some(count))
  }
}
