use swc_common::BytePos;
use swc_common::Span;

use crate::SourceTextInfoProvider;

/// Swc unfortunately uses `BytePos(0)` as a magic value. This means
/// that we can't have byte positions of nodes line up with the text.
/// To get around this, we have created our own `SourcePos` wrapper
/// that exposes a 0-indexed value, but hides the underlying swc
/// byte position which is not 0-indexed.
///
/// When using this type, you MUST parse files in swc using
/// `SourcePos::START_BYTE_POS` otherwise bad things will happen.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourcePos(BytePos);

impl SourcePos {
  /// Use this value as the start byte position when parsing.
  pub const START_BYTE_POS: BytePos = BytePos(1_000);

  pub fn new(index: usize) -> Self {
    Self(BytePos(index as u32) + SourcePos::START_BYTE_POS)
  }

  pub fn from_byte_pos(byte_pos: BytePos) -> Self {
    #[cfg(debug_assertions)]
    if byte_pos < SourcePos::START_BYTE_POS {
      panic!(concat!(
        "The provided byte position was less than the start byte position. ",
        "Ensure the source file is parsed starting at SourcePos::START_BYTE_POS."
      ))
    }
    Self(byte_pos)
  }

  pub fn as_byte_pos(&self) -> BytePos {
    self.0
  }

  pub fn as_usize(&self) -> usize {
    (self.0 - SourcePos::START_BYTE_POS).0 as usize
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRange {
  pub start: SourcePos,
  pub end: SourcePos,
}

impl SourceRange {
  pub fn new(start: SourcePos, end: SourcePos)  -> Self{
    Self { start, end }
  }
}

impl std::ops::Sub<SourcePos> for SourceRange {
  type Output = std::ops::Range<usize>;

  fn sub(self, rhs: SourcePos) -> Self::Output {
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
