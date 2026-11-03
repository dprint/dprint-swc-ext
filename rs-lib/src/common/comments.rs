use super::pos::*;
use super::text_info::*;
use super::tokens::*;
use crate::swc::common::comments::Comment;
use crate::swc::common::comments::SingleThreadedCommentsMapInner;
use rustc_hash::FxHashMap;

pub struct CommentContainer<'a> {
  pub leading: &'a SingleThreadedCommentsMapInner,
  pub trailing: &'a SingleThreadedCommentsMapInner,
  /// What `leading_comments` returns at each position with comments, since most
  /// positions don't have any and this only needs to be looked up once.
  leading_by_start: FxHashMap<SourcePos, CommentsPair<'a>>,
  /// What `trailing_comments` returns at each position with comments.
  trailing_by_end: FxHashMap<SourcePos, CommentsPair<'a>>,
}

/// The comments at two positions in the order they're in the text (ex. the trailing comments
/// of a token followed by the leading comments of the next token).
#[derive(Default, Clone, Copy)]
struct CommentsPair<'a> {
  first: &'a [Comment],
  second: &'a [Comment],
}

impl<'a> CommentContainer<'a> {
  pub fn new(
    leading: &'a SingleThreadedCommentsMapInner,
    trailing: &'a SingleThreadedCommentsMapInner,
    tokens: &'a TokenContainer<'a>,
    text_info: &'a SourceTextInfo,
  ) -> Self {
    let text_range = text_info.range();
    let mut leading_by_start = FxHashMap::<SourcePos, CommentsPair<'a>>::default();
    let mut trailing_by_end = FxHashMap::<SourcePos, CommentsPair<'a>>::default();

    for (byte_pos, comments) in leading.iter() {
      let pos = SourcePos::unsafely_from_byte_pos(*byte_pos);
      // the leading comments at a position come after the trailing comments of the token before it
      leading_by_start.entry(pos).or_default().second = comments;
      // and after the trailing comments at the end of the token before it
      let end_before = if pos == text_range.end {
        tokens.tokens.last().map(|token| token.end())
      } else {
        match tokens.get_token_index_at_start(pos) {
          Some(index) if index > 0 => Some(tokens.get_token_at_index(index - 1).unwrap().end()),
          _ => None,
        }
      };
      if let Some(end_before) = end_before {
        trailing_by_end.entry(end_before).or_default().second = comments;
      }
    }

    for (byte_pos, comments) in trailing.iter() {
      let pos = SourcePos::unsafely_from_byte_pos(*byte_pos);
      trailing_by_end.entry(pos).or_default().first = comments;
      // the trailing comments of a token come before the leading comments of the next token
      let next_start = if pos == text_range.start.as_source_pos() {
        tokens.tokens.first().map(|token| token.start())
      } else {
        tokens
          .get_token_index_at_end(pos)
          .and_then(|index| tokens.get_token_at_index(index + 1))
          .map(|token| token.start())
      };
      if let Some(next_start) = next_start {
        leading_by_start.entry(next_start).or_default().first = comments;
      }
    }

    CommentContainer {
      leading,
      trailing,
      leading_by_start,
      trailing_by_end,
    }
  }

  pub fn all_comments(&'a self) -> CommentsIterator<'a> {
    let mut iterator = CommentsIterator::empty();
    for comments in self.leading.values().chain(self.trailing.values()) {
      iterator.push_slice(comments);
    }
    iterator
  }

  /// Gets the trailing comments of the previous token followed by the leading comments
  /// at the position.
  pub fn leading_comments(&'a self, start: SourcePos) -> CommentsIterator<'a> {
    comments_pair_iterator(self.leading_by_start.get(&start))
  }

  /// Gets the trailing comments at the position followed by the leading comments of the
  /// next token.
  pub fn trailing_comments(&'a self, end: SourcePos) -> CommentsIterator<'a> {
    comments_pair_iterator(self.trailing_by_end.get(&end))
  }
}

fn comments_pair_iterator<'a>(pair: Option<&CommentsPair<'a>>) -> CommentsIterator<'a> {
  let mut iterator = CommentsIterator::empty();
  if let Some(pair) = pair {
    iterator.push_slice(pair.first);
    iterator.push_slice(pair.second);
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
  use crate::swc::common::comments::CommentKind;
  use crate::swc::common::DUMMY_SP;

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
