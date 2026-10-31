use super::pos::*;
use super::text_info::*;
use super::tokens::*;
use crate::swc::common::comments::Comment;
use crate::swc::common::comments::SingleThreadedCommentsMapInner;

pub struct CommentContainer<'a> {
  pub leading: &'a SingleThreadedCommentsMapInner,
  pub trailing: &'a SingleThreadedCommentsMapInner,
  tokens: &'a TokenContainer<'a>,
  text_info: &'a SourceTextInfo,
}

impl<'a> CommentContainer<'a> {
  pub fn new(
    leading: &'a SingleThreadedCommentsMapInner,
    trailing: &'a SingleThreadedCommentsMapInner,
    tokens: &'a TokenContainer<'a>,
    text_info: &'a SourceTextInfo,
  ) -> Self {
    CommentContainer {
      leading,
      trailing,
      tokens,
      text_info,
    }
  }

  pub fn all_comments(&'a self) -> CommentsIterator<'a> {
    let mut iterator = CommentsIterator::empty();
    for comments in self.leading.values().chain(self.trailing.values()) {
      iterator.push_slice(comments);
    }
    iterator
  }

  pub fn leading_comments(&'a self, start: SourcePos) -> CommentsIterator<'a> {
    let previous_token_hi = self.tokens.get_token_index_at_start(start).map(|index| {
      if index == 0 {
        self.text_info.range().start.as_source_pos()
      } else {
        self.tokens.get_token_at_index(index - 1).unwrap().end()
      }
    });
    let leading = self.get_leading(start);
    if let Some(previous_token_hi) = previous_token_hi {
      let trailing = self.get_trailing(previous_token_hi);
      combine_comment_vecs(trailing, leading)
    } else {
      combine_comment_vecs(leading, None)
    }
  }

  pub fn trailing_comments(&'a self, end: SourcePos) -> CommentsIterator<'a> {
    let next_token_lo = self.tokens.get_token_index_at_end(end).map(|index| {
      self
        .tokens
        .get_token_at_index(index + 1)
        .map(|t| t.start())
        .unwrap_or_else(|| self.text_info.range().end)
    });
    let trailing = self.get_trailing(end);
    if let Some(next_token_lo) = next_token_lo {
      let leading = self.get_leading(next_token_lo);
      combine_comment_vecs(trailing, leading)
    } else {
      combine_comment_vecs(trailing, None)
    }
  }

  fn get_leading(&'a self, start: SourcePos) -> Option<&'a Vec<Comment>> {
    self.leading.get(&start.as_byte_pos())
  }

  fn get_trailing(&'a self, end: SourcePos) -> Option<&'a Vec<Comment>> {
    self.trailing.get(&end.as_byte_pos())
  }
}

fn combine_comment_vecs<'a>(a: Option<&'a Vec<Comment>>, b: Option<&'a Vec<Comment>>) -> CommentsIterator<'a> {
  let mut iterator = CommentsIterator::empty();
  if let Some(a) = a {
    iterator.push_slice(a);
  }
  if let Some(b) = b {
    iterator.push_slice(b);
  }
  iterator
}

/// The number of comment slices stored without allocating, which is enough
/// for the comments before or after a token (the trailing comments of the
/// previous token and the leading comments of the next one) along with some
/// more when extending.
const INLINE_SLICES_LEN: usize = 4;

/// The slices of comments being iterated, which are stored inline until there are
/// more than [`INLINE_SLICES_LEN`] since most positions only have one or two.
#[derive(Clone)]
enum CommentSlices<'a> {
  Inline { slices: [&'a [Comment]; INLINE_SLICES_LEN], len: usize },
  Heap(Vec<&'a [Comment]>),
}

impl<'a> CommentSlices<'a> {
  fn as_slice(&self) -> &[&'a [Comment]] {
    match self {
      CommentSlices::Inline { slices, len } => &slices[..*len],
      CommentSlices::Heap(slices) => slices,
    }
  }

  fn push(&mut self, comments: &'a [Comment]) {
    match self {
      CommentSlices::Inline { slices, len } => {
        if *len < INLINE_SLICES_LEN {
          slices[*len] = comments;
          *len += 1;
        } else {
          let mut heap = Vec::with_capacity(INLINE_SLICES_LEN * 2);
          heap.extend_from_slice(slices);
          heap.push(comments);
          *self = CommentSlices::Heap(heap);
        }
      }
      CommentSlices::Heap(slices) => slices.push(comments),
    }
  }
}

#[derive(Clone)]
pub struct CommentsIterator<'a> {
  comment_slices: CommentSlices<'a>,
  outer_index: usize,
  inner_index: usize,
  outer_index_back: usize,
  inner_index_back: usize,
}

impl<'a> CommentsIterator<'a> {
  pub fn empty() -> Self {
    CommentsIterator {
      comment_slices: CommentSlices::Inline {
        slices: [&[]; INLINE_SLICES_LEN],
        len: 0,
      },
      outer_index: 0,
      inner_index: 0,
      outer_index_back: 0,
      inner_index_back: 0,
    }
  }

  pub fn new(comment_vecs: Vec<&'a Vec<Comment>>) -> Self {
    let mut iterator = Self::empty();
    for comments in comment_vecs {
      iterator.push_slice(comments);
    }
    iterator
  }

  pub fn reset(&mut self) {
    self.outer_index = 0;
    self.inner_index = 0;
  }

  pub fn extend(&mut self, iterator: CommentsIterator<'a>) {
    for comments in iterator.comment_slices.as_slice() {
      self.comment_slices.push(comments);
    }

    // reset the back iterator
    self.reset_back();
  }

  pub fn is_empty(&self) -> bool {
    // empty slices are never stored
    self.comment_slices.as_slice().is_empty()
  }

  pub fn peek_last_comment(&self) -> Option<&'a Comment> {
    self.comment_slices.as_slice().last().and_then(|comments| comments.last())
  }

  fn push_slice(&mut self, comments: &'a [Comment]) {
    // skip empty slices so the inline storage is only used for comments
    if !comments.is_empty() {
      self.comment_slices.push(comments);
      self.reset_back();
    }
  }

  fn reset_back(&mut self) {
    self.outer_index_back = self.comment_slices.as_slice().len();
    self.inner_index_back = 0;
  }
}

impl<'a> Default for CommentsIterator<'a> {
  fn default() -> Self {
    Self::empty()
  }
}

impl<'a> Iterator for CommentsIterator<'a> {
  type Item = &'a Comment;

  fn next(&mut self) -> Option<&'a Comment> {
    let slices = self.comment_slices.as_slice();
    loop {
      if let Some(comments) = slices.get(self.outer_index) {
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
    for comments in self.comment_slices.as_slice().iter().skip(self.outer_index) {
      count += comments.len() - next_inner_index;
      next_inner_index = 0;
    }
    (count, Some(count))
  }
}

impl<'a> DoubleEndedIterator for CommentsIterator<'a> {
  fn next_back(&mut self) -> Option<&'a Comment> {
    let slices = self.comment_slices.as_slice();
    if self.inner_index_back == 0 {
      if self.outer_index_back == 0 {
        return None;
      }
      self.outer_index_back -= 1;
      // the slices are never empty
      self.inner_index_back = slices[self.outer_index_back].len() - 1;
    } else {
      self.inner_index_back -= 1;
    }

    slices.get(self.outer_index_back).and_then(|inner| inner.get(self.inner_index_back))
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::swc::common::DUMMY_SP;
  use crate::swc::common::comments::CommentKind;

  fn comments(texts: &[&str]) -> Vec<Comment> {
    texts
      .iter()
      .map(|text| Comment {
        kind: CommentKind::Line,
        span: DUMMY_SP,
        text: (*text).into(),
      })
      .collect()
  }

  fn texts(iterator: impl Iterator<Item = &'static Comment>) -> Vec<String> {
    iterator.map(|c| c.text.to_string()).collect()
  }

  #[test]
  fn iterates_forwards_and_backwards_across_slices() {
    let slices: Vec<&'static Vec<Comment>> = vec![
      Box::leak(Box::new(comments(&["a", "b"]))),
      Box::leak(Box::new(comments(&[]))),
      Box::leak(Box::new(comments(&["c"]))),
    ];
    let iterator = CommentsIterator::new(slices);
    assert!(!iterator.is_empty());
    assert_eq!(iterator.size_hint(), (3, Some(3)));
    assert_eq!(iterator.peek_last_comment().unwrap().text.as_str(), "c");
    assert_eq!(texts(iterator.clone()), vec!["a", "b", "c"]);
    assert_eq!(texts(iterator.rev()), vec!["c", "b", "a"]);
  }

  #[test]
  fn extends_past_the_inline_slices() {
    let mut iterator = CommentsIterator::empty();
    assert!(iterator.is_empty());
    assert_eq!(iterator.clone().next_back(), None);
    let mut expected = Vec::new();
    for i in 0..(INLINE_SLICES_LEN * 3) {
      let text = i.to_string();
      expected.push(text.clone());
      let slice: &'static Vec<Comment> = Box::leak(Box::new(comments(&[&text])));
      iterator.extend(CommentsIterator::new(vec![slice]));
      // empty iterators are skipped
      iterator.extend(CommentsIterator::empty());
    }
    assert!(matches!(iterator.comment_slices, CommentSlices::Heap(_)));
    assert_eq!(texts(iterator.clone()), expected);
    expected.reverse();
    assert_eq!(texts(iterator.rev()), expected);
  }
}
