use std::sync::Arc;

use swc_common::input::StringInput;
pub use text_lines::LineAndColumnIndex;
pub use text_lines::TextLines;

use super::pos::*;
use super::text_encoding::strip_bom_mut;
use super::text_encoding::BOM_CHAR;
use super::LineAndColumnDisplay;

/// Stores the source text along with other data such as where all the lines
/// occur in the text.
///
/// Note: This struct is cheap to clone.
#[derive(Clone)]
pub struct SourceTextInfo {
  // keep this struct cheap to clone
  start_pos: StartSourcePos,
  text: Arc<str>,
  text_lines: Arc<TextLines>,
}

impl SourceTextInfo {
  /// Creates a new `SourceTextInfo` from the provided source text.
  pub fn new(text: Arc<str>) -> Self {
    Self::new_with_pos(StartSourcePos::START_SOURCE_POS.as_source_pos(), text)
  }

  /// Creates a new `SourceTextInfo` from the provided source start position
  /// and source text.
  ///
  /// Note: When bundling swc will keep increasing the start position for
  /// each source file.
  pub fn new_with_pos(start_pos: SourcePos, text: Arc<str>) -> Self {
    // The BOM should be stripped before it gets passed here
    // because it's a text encoding concern that should be
    // stripped when the file is read.
    // todo(dsherret): re-enable after removing the below
    // assert!(!text.starts_with(BOM_CHAR), "BOM should be stripped before creating a SourceTextInfo.");

    // todo(dsherret): remove this once handled downstream
    let text = if text.starts_with(BOM_CHAR) {
      let mut text = text.to_string();
      strip_bom_mut(&mut text);
      text.into()
    } else {
      text
    };

    Self::new_with_indent_width(start_pos, text, 2)
  }

  /// Creates a new `SourceTextInfo` from the provided start position,
  /// source text, and indentation width.
  ///
  /// The indentation width determines the number of columns to use
  /// when going over a tab character. For example, an indent width
  /// of 2 will mean each tab character will represent 2 columns.
  /// The default indentation width used in the other methods is `2`
  /// to match the default indentation used by `deno fmt`.
  pub fn new_with_indent_width(start_pos: SourcePos, text: Arc<str>, indent_width: usize) -> Self {
    Self {
      start_pos: StartSourcePos(start_pos),
      text_lines: Arc::new(TextLines::with_indent_width(&text, indent_width)),
      text,
    }
  }

  /// Creates a new `SourceTextInfo` from the provided source text.
  ///
  /// Generally, prefer using `SourceTextInfo::new` to provide a
  /// string already in an `std::sync::Arc`.
  pub fn from_string(text: String) -> Self {
    Self::new(text.into())
  }

  /// Gets an swc `StringInput` for this text information that can be
  /// used with parsing.
  pub fn as_string_input(&self) -> StringInput<'_> {
    let range = self.range();
    StringInput::new(self.text_str(), range.start.as_byte_pos(), range.end.as_byte_pos())
  }

  /// Gets the source text.
  pub fn text(&self) -> Arc<str> {
    self.text.clone()
  }

  /// Gets a reference to the source text.
  pub fn text_str(&self) -> &str {
    &self.text
  }

  /// Gets the range—start and end byte position—of the source text.
  pub fn range(&self) -> SourceRange<StartSourcePos> {
    SourceRange::new(self.start_pos, self.start_pos + self.text.len())
  }

  /// Gets the number of lines in the source text.
  pub fn lines_count(&self) -> usize {
    self.text_lines.lines_count()
  }

  /// Gets the 0-indexed line index at the provided byte position.
  ///
  /// Note that this will panic when providing a byte position outside
  /// the range of the source text.
  pub fn line_index(&self, pos: SourcePos) -> usize {
    self.assert_pos(pos);
    self.text_lines.line_index(self.get_relative_index_from_pos(pos))
  }

  /// Gets the line start byte position of the provided 0-indexed line index.
  ///
  /// Note that this will panic if providing a line index outside the
  /// bounds of the number of lines.
  pub fn line_start(&self, line_index: usize) -> SourcePos {
    self.assert_line_index(line_index);
    self.get_pos_from_relative_index(self.text_lines.line_start(line_index))
  }

  /// Gets the line end byte position of the provided 0-indexed line index.
  ///
  /// Note that this will panic if providing a line index outside the
  /// bounds of the number of lines.
  pub fn line_end(&self, line_index: usize) -> SourcePos {
    self.assert_line_index(line_index);
    self.get_pos_from_relative_index(self.text_lines.line_end(line_index))
  }

  /// Gets the 0-indexed line and column index of the provided byte position.
  ///
  /// Note that this will panic when providing a byte position outside
  /// the range of the source text.
  pub fn line_and_column_index(&self, pos: SourcePos) -> LineAndColumnIndex {
    self.assert_pos(pos);
    self.text_lines.line_and_column_index(self.get_relative_index_from_pos(pos))
  }

  /// Gets the 1-indexed line and column index of the provided byte position
  /// taking into account the default indentation width.
  ///
  /// Note that this will panic when providing a byte position outside
  /// the range of the source text.
  pub fn line_and_column_display(&self, pos: SourcePos) -> LineAndColumnDisplay {
    self.assert_pos(pos);
    self.text_lines.line_and_column_display(self.get_relative_index_from_pos(pos))
  }

  /// Gets the 1-indexed line and column index of the provided byte position
  /// with a custom indentation width.
  ///
  /// Note that this will panic when providing a byte position outside
  /// the range of the source text.
  pub fn line_and_column_display_with_indent_width(&self, pos: SourcePos, indent_width: usize) -> LineAndColumnDisplay {
    self.assert_pos(pos);
    self
      .text_lines
      .line_and_column_display_with_indent_width(self.get_relative_index_from_pos(pos), indent_width)
  }

  /// Gets the source position of the provided line and column index.
  ///
  /// Note that this will panic if providing a line index outside the
  /// bounds of the number of lines, but will clip the the line end byte index
  /// when exceeding the line length.
  pub fn loc_to_source_pos(&self, line_and_column_index: LineAndColumnIndex) -> SourcePos {
    self.assert_line_index(line_and_column_index.line_index);
    self.get_pos_from_relative_index(self.text_lines.byte_index(line_and_column_index))
  }

  /// Gets a reference to the text slice of the line at the provided
  /// 0-based index.
  ///
  /// Note that this will panic if providing a line index outside the
  /// bounds of the number of lines.
  pub fn line_text(&self, line_index: usize) -> &str {
    let range = SourceRange {
      start: self.line_start(line_index),
      end: self.line_end(line_index),
    };
    self.range_text(&range)
  }

  /// Gets the source text located within the provided range.
  pub fn range_text(&self, range: &SourceRange) -> &str {
    let start = self.get_relative_index_from_pos(range.start);
    let end = self.get_relative_index_from_pos(range.end);
    &self.text_str()[start..end]
  }

  fn assert_pos(&self, pos: SourcePos) {
    let range = self.range();
    if pos < range.start {
      panic!("The provided position {} was less than the start position {}.", pos, range.start,);
    } else if pos > range.end {
      panic!("The provided position {} was greater than the end position {}.", pos, range.end,);
    }
  }

  fn assert_line_index(&self, line_index: usize) {
    if line_index >= self.lines_count() {
      panic!(
        "The specified line index {} was greater or equal to the number of lines of {}.",
        line_index,
        self.lines_count()
      );
    }
  }

  fn get_relative_index_from_pos(&self, pos: SourcePos) -> usize {
    pos - self.start_pos
  }

  fn get_pos_from_relative_index(&self, relative_index: usize) -> SourcePos {
    self.start_pos + relative_index
  }
}

impl std::fmt::Debug for SourceTextInfo {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SourceTextInfo")
      .field("start_pos", &self.start_pos)
      .field("text", &self.text)
      .finish()
  }
}

pub trait SourceTextInfoProvider<'a> {
  fn text_info(&self) -> &'a SourceTextInfo;
}

impl<'a> SourceTextInfoProvider<'a> for &'a SourceTextInfo {
  fn text_info(&self) -> &'a SourceTextInfo {
    self
  }
}

pub trait SourceTextProvider<'a> {
  fn text(&self) -> &'a Arc<str>;
  fn start_pos(&self) -> StartSourcePos;
}

impl<'a, T> SourceTextProvider<'a> for T
where
  T: SourceTextInfoProvider<'a>,
{
  fn text(&self) -> &'a Arc<str> {
    &self.text_info().text
  }

  fn start_pos(&self) -> StartSourcePos {
    self.text_info().start_pos
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn line_and_column_index() {
    let text = "12\n3\r\nβ\n5";
    for i in 0..10 {
      run_with_text_info(SourceTextInfo::new_with_pos(SourcePos::new(i), text.to_string().into()), i);
    }

    fn run_with_text_info(text_info: SourceTextInfo, i: usize) {
      assert_pos_line_and_col(&text_info, i, 0, 0); // 1
      assert_pos_line_and_col(&text_info, 1 + i, 0, 1); // 2
      assert_pos_line_and_col(&text_info, 2 + i, 0, 2); // \n
      assert_pos_line_and_col(&text_info, 3 + i, 1, 0); // 3
      assert_pos_line_and_col(&text_info, 4 + i, 1, 1); // \r
      assert_pos_line_and_col(&text_info, 5 + i, 1, 2); // \n
      assert_pos_line_and_col(&text_info, 6 + i, 2, 0); // first β index
      assert_pos_line_and_col(&text_info, 7 + i, 2, 0); // second β index
      assert_pos_line_and_col(&text_info, 8 + i, 2, 1); // \n
      assert_pos_line_and_col(&text_info, 9 + i, 3, 0); // 5
      assert_pos_line_and_col(&text_info, 10 + i, 3, 1); // <EOF>
    }
  }

  fn assert_pos_line_and_col(text_info: &SourceTextInfo, pos: usize, line_index: usize, column_index: usize) {
    assert_eq!(
      text_info.line_and_column_index(SourcePos::new(pos)),
      LineAndColumnIndex { line_index, column_index }
    );
  }

  #[test]
  #[should_panic(expected = "The provided position 0 was less than the start position 1.")]
  fn line_and_column_index_panic_less_than() {
    let info = SourceTextInfo::new_with_pos(SourcePos::new(1), "test".to_string().into());
    info.line_and_column_index(SourcePos::new(0));
  }

  #[test]
  #[should_panic(expected = "The provided position 6 was greater than the end position 5.")]
  fn line_and_column_index_panic_greater_than() {
    let info = SourceTextInfo::new_with_pos(SourcePos::new(1), "test".to_string().into());
    info.line_and_column_index(SourcePos::new(6));
  }

  #[test]
  fn line_start() {
    let text = "12\n3\r\n4\n5";
    for i in 0..10 {
      run_with_text_info(SourceTextInfo::new_with_pos(SourcePos::new(i), text.to_string().into()), i);
    }

    fn run_with_text_info(text_info: SourceTextInfo, i: usize) {
      assert_line_start(&text_info, 0, i);
      assert_line_start(&text_info, 1, 3 + i);
      assert_line_start(&text_info, 2, 6 + i);
      assert_line_start(&text_info, 3, 8 + i);
    }
  }

  fn assert_line_start(text_info: &SourceTextInfo, line_index: usize, line_end: usize) {
    assert_eq!(text_info.line_start(line_index), SourcePos::new(line_end));
  }

  #[test]
  #[should_panic(expected = "The specified line index 1 was greater or equal to the number of lines of 1.")]
  fn line_start_equal_number_lines() {
    let info = SourceTextInfo::new_with_pos(SourcePos::new(1), "test".to_string().into());
    info.line_start(1);
  }

  #[test]
  fn line_end() {
    let text = "12\n3\r\n4\n5";
    for i in 0..10 {
      run_with_text_info(SourceTextInfo::new_with_pos(SourcePos::new(i), text.to_string().into()), i);
    }

    fn run_with_text_info(text_info: SourceTextInfo, i: usize) {
      assert_line_end(&text_info, 0, 2 + i);
      assert_line_end(&text_info, 1, 4 + i);
      assert_line_end(&text_info, 2, 7 + i);
      assert_line_end(&text_info, 3, 9 + i);
    }
  }

  fn assert_line_end(text_info: &SourceTextInfo, line_index: usize, line_end: usize) {
    assert_eq!(text_info.line_end(line_index), SourcePos::new(line_end));
  }

  #[test]
  #[should_panic(expected = "The specified line index 1 was greater or equal to the number of lines of 1.")]
  fn line_end_equal_number_lines() {
    let info = SourceTextInfo::new_with_pos(SourcePos::new(1), "test".to_string().into());
    info.line_end(1);
  }
}
