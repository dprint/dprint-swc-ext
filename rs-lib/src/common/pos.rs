use swc_common::BytePos;
use swc_common::Span;

use super::comments::*;
use super::types::*;
use super::text_info::*;

/// Swc unfortunately uses `BytePos(0)` as a magic value. This means
/// that we can't have byte positions of nodes line up with the text.
/// To get around this, we have created our own `SourcePos` wrapper
/// that hides the underlying swc byte position so it can't be used
/// incorrectly.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourcePos(BytePos);

impl SourcePos {
  #[cfg(test)]
  pub fn new(index: usize) -> Self {
    Self(StartSourcePos::START_SOURCE_POS.as_byte_pos() + BytePos(index as u32))
  }

  pub(crate) fn from_byte_pos(byte_pos: BytePos) -> Self {
    #[cfg(debug_assertions)]
    if byte_pos < StartSourcePos::START_SOURCE_POS.as_byte_pos() {
      panic!(concat!(
        "The provided byte position was less than the start byte position. ",
        "Ensure the source file is parsed starting at SourcePos::START_SOURCE_POS."
      ))
    }
    Self(byte_pos)
  }

  pub fn as_byte_pos(&self) -> BytePos {
    self.0
  }

  pub(crate) fn as_usize(&self) -> usize {
    (self.as_byte_pos() - StartSourcePos::START_SOURCE_POS.as_byte_pos()).0 as usize
  }
}

impl std::fmt::Debug for SourcePos {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("SourcePos").field(&self.as_usize()).finish()
  }
}

impl std::fmt::Display for SourcePos {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.as_usize().to_string())
  }
}

impl std::ops::Add<usize> for SourcePos {
  type Output = SourcePos;

  fn add(self, rhs: usize) -> Self::Output {
    SourcePos(BytePos(self.0.0 + rhs as u32))
  }
}

impl std::ops::Sub<usize> for SourcePos {
  type Output = SourcePos;

  fn sub(self, rhs: usize) -> Self::Output {
    SourcePos(BytePos(self.0.0 - rhs as u32))
  }
}

impl std::ops::Sub<SourcePos> for SourcePos {
  type Output = usize;

  fn sub(self, rhs: SourcePos) -> Self::Output {
    (self.0 - rhs.0).0 as usize
  }
}

impl SourceRanged for SourcePos {
  fn start(&self) -> SourcePos {
    *self
  }

  fn end(&self) -> SourcePos {
    *self
  }
}

/// A special source pos that indicates the source start
/// which functions can use as a parameter type in order
/// to ensure someone doesn't provide the wrong position.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StartSourcePos(pub(crate) SourcePos);

impl StartSourcePos {
  /// Use this value as the start byte position when parsing.
  pub const START_SOURCE_POS: StartSourcePos = StartSourcePos(SourcePos(BytePos(1)));

  pub fn as_byte_pos(&self) -> BytePos {
    self.0.as_byte_pos()
  }

  pub fn as_source_pos(&self) -> SourcePos {
    self.0
  }

  pub(crate) fn as_usize(&self) -> usize {
    (self.as_byte_pos() - StartSourcePos::START_SOURCE_POS.as_byte_pos()).0 as usize
  }
}

// Only want Into and not From in order to prevent
// people from creating one of these easily.
#[allow(clippy::from_over_into)]
impl Into<SourcePos> for StartSourcePos {
  fn into(self) -> SourcePos {
    self.as_source_pos()
  }
}

impl std::fmt::Debug for StartSourcePos {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("StartSourcePos").field(&self.as_usize()).finish()
  }
}

impl std::fmt::Display for StartSourcePos {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.as_usize().to_string())
  }
}

impl std::ops::Add<usize> for StartSourcePos {
  type Output = SourcePos;

  fn add(self, rhs: usize) -> Self::Output {
    SourcePos(BytePos(self.0.0.0 + rhs as u32))
  }
}

impl std::ops::Sub<StartSourcePos> for SourcePos {
  type Output = usize;

  fn sub(self, rhs: StartSourcePos) -> Self::Output {
    (self.0 - rhs.0.0).0 as usize
  }
}

impl std::cmp::PartialEq<SourcePos> for StartSourcePos {
  fn eq(&self, other: &SourcePos) -> bool {
    self.0 == *other
  }
}

impl std::cmp::PartialOrd<SourcePos> for StartSourcePos {
  fn partial_cmp(&self, other: &SourcePos) -> Option<std::cmp::Ordering> {
    self.0.partial_cmp(other)
  }
}

impl std::cmp::PartialEq<StartSourcePos> for SourcePos {
  fn eq(&self, other: &StartSourcePos) -> bool {
    *self == other.0
  }
}

impl std::cmp::PartialOrd<StartSourcePos> for SourcePos {
  fn partial_cmp(&self, other: &StartSourcePos) -> Option<std::cmp::Ordering> {
    self.partial_cmp(&other.0)
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRange<T = SourcePos> where T : Into<SourcePos> + Clone + Copy {
  pub start: T,
  pub end: SourcePos,
}

impl<T : Into<SourcePos> + Clone + Copy> SourceRange<T> {
  pub fn new(start: T, end: SourcePos)  -> Self{
    Self { start, end }
  }
}

impl SourceRange<SourcePos> {
  /// Gets the relative byte range based on the source text's start position.
  pub fn as_std_range(&self, source_start: StartSourcePos) -> std::ops::Range<usize> {
    let start = self.start - source_start;
    let end = self.end - source_start;
    start..end
  }
}

impl SourceRange<StartSourcePos> {
  /// Gets the relative byte range based on the source text's start position.
  pub fn as_std_range(&self) -> std::ops::Range<usize> {
    let end = self.end - self.start;
    0..end
  }
}

impl<T: Into<SourcePos> + Clone + Copy> SourceRanged for SourceRange<T> {
  fn start(&self) -> SourcePos {
    self.start.into()
  }

  fn end(&self) -> SourcePos {
    self.end
  }
}

impl std::ops::Sub<StartSourcePos> for SourceRange {
  type Output = std::ops::Range<usize>;

  fn sub(self, rhs: StartSourcePos) -> Self::Output {
    (self.start - rhs)..(self.end - rhs)
  }
}

impl From<Span> for SourceRange {
  fn from(value: Span) -> Self {
    SourceRange {
      start: SourcePos::from_byte_pos(value.lo),
      end: SourcePos::from_byte_pos(value.hi),
    }
  }
}

pub trait SourceRanged {
  fn start(&self) -> SourcePos;
  fn end(&self) -> SourcePos;

  fn range(&self) -> SourceRange {
    SourceRange {
      start: self.start(),
      end: self.end(),
    }
  }

  fn byte_width(&self) -> usize {
    self.end() - self.start()
  }

  fn start_line_fast(&self, source: &dyn SourceTextInfoProvider) -> usize {
    source.text_info().line_index(self.start())
  }

  fn end_line_fast(&self, source: &dyn SourceTextInfoProvider) -> usize {
    source.text_info().line_index(self.end())
  }

  fn start_column_fast(&self, source: &dyn SourceTextInfoProvider) -> usize {
    self.column_at_pos(source, self.start())
  }

  fn end_column_fast(&self, source: &dyn SourceTextInfoProvider) -> usize {
    self.column_at_pos(source, self.end())
  }

  fn column_at_pos(&self, source: &dyn SourceTextInfoProvider, pos: SourcePos) -> usize {
    let text_info = source.text_info();
    let text_bytes = text_info.text_str().as_bytes();
    let pos = pos - self.start();
    let mut line_start = 0;
    for i in (0..pos).rev() {
      if text_bytes[i] == b'\n' {
        line_start = i + 1;
        break;
      }
    }
    let text_slice = &text_info.text_str()[line_start..pos];
    text_slice.chars().count()
  }

  fn char_width_fast(&self, source: &dyn SourceTextInfoProvider) -> usize {
    self.text_fast(source).chars().count()
  }

  fn text_fast<'a>(&self, source: &dyn SourceTextInfoProvider<'a>) -> &'a str {
    let text_info = source.text_info();
    let byte_range = self.range() - text_info.range().start;
    &text_info.text_str()[byte_range]
  }

  fn tokens_fast<'a>(&self, program: &dyn RootNode<'a>) -> &'a [TokenAndRange] {
    let token_container = program.token_container();
    token_container.get_tokens_in_range(self.start(), self.end())
  }

  fn leading_comments_fast<'a>(&self, program: &dyn RootNode<'a>) -> CommentsIterator<'a> {
    program.comment_container().leading_comments(self.start())
  }

  fn trailing_comments_fast<'a>(&self, program: &dyn RootNode<'a>) -> CommentsIterator<'a> {
    program.comment_container().trailing_comments(self.end())
  }

  fn previous_token_fast<'a>(&self, program: &dyn RootNode<'a>) -> Option<&'a TokenAndRange> {
    program.token_container().get_previous_token(self.start())
  }

  fn next_token_fast<'a>(&self, program: &dyn RootNode<'a>) -> Option<&'a TokenAndRange> {
    program.token_container().get_next_token(self.end())
  }

  fn previous_tokens_fast<'a>(&self, program: &dyn RootNode<'a>) -> &'a [TokenAndRange] {
    let token_container = program.token_container();
    let index = token_container
      .get_token_index_at_start(self.start())
      // fallback
      .or_else(|| token_container.get_token_index_at_end(self.start()))
      .unwrap_or_else(|| {
        panic!(
          "The specified start position ({}) did not have a token index.",
          self.start()
        )
      });
    &token_container.tokens[0..index]
  }

  fn next_tokens_fast<'a>(&self, program: &dyn RootNode<'a>) -> &'a [TokenAndRange] {
    let token_container = program.token_container();
    let index = token_container
      .get_token_index_at_end(self.end())
      // fallback
      .or_else(|| token_container.get_token_index_at_start(self.end()))
      .unwrap_or_else(|| {
        panic!(
          "The specified end position ({}) did not have a token index.",
          self.end()
        )
      });
    &token_container.tokens[index + 1..]
  }
}

impl<T> SourceRanged for T
where
  T: swc_common::Spanned,
{
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.span().lo)
  }

  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.span().hi)
  }
}
