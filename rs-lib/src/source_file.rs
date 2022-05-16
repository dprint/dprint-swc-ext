pub use text_lines::LineAndColumnIndex;
pub use text_lines::TextLines;

use crate::SourcePos;
use crate::SourceRange;

pub trait SourceFile {
  fn text(&self) -> &str;
  fn range(&self) -> SourceRange;
  fn lines_count(&self) -> usize;
  fn line_index(&self, pos: SourcePos) -> usize;
  fn line_start(&self, line_index: usize) -> SourcePos;
  fn line_end(&self, line_index: usize) -> SourcePos;
  fn line_and_column_index(&self, pos: SourcePos) -> LineAndColumnIndex;
}

impl SourceFile for swc_common::SourceFile {
  fn text(&self) -> &str {
    &self.src
  }

  fn range(&self) -> SourceRange {
    SourceRange::new(
      SourcePos::from_byte_pos(self.start_pos),
      SourcePos::from_byte_pos(self.end_pos),
    )
  }

  fn lines_count(&self) -> usize {
    swc_common::SourceFile::count_lines(self)
  }

  fn line_index(&self, pos: SourcePos) -> usize {
    if let Some(line_index) = swc_common::SourceFile::lookup_line(self, pos.as_byte_pos()) {
      line_index
    } else {
      panic!("Could not find line index at pos {}.", pos)
    }
  }

  fn line_start(&self, line_index: usize) -> SourcePos {
    if line_index >= self.lines.len() {
      panic!(
        "The specified line index {} was greater or equal to the number of lines of {}.",
        line_index,
        self.lines.len()
      );
    }
    SourcePos::from_byte_pos(self.lines[line_index])
  }

  fn line_end(&self, line_index: usize) -> SourcePos {
    let lines_count = self.lines_count();
    let start_pos = self.range().start;
    if line_index >= lines_count {
      panic!(
        "The specified line index {} was greater or equal to the number of lines of {}.",
        line_index, lines_count,
      );
    } else if line_index + 1 == lines_count {
      start_pos + self.text().len()
    } else {
      let text = self.text().as_bytes();
      let next_line_start_index = self.line_start(line_index + 1) - start_pos;
      if next_line_start_index > 2 && text[next_line_start_index - 2] == b'\r' {
        start_pos + next_line_start_index - 2
      } else {
        start_pos + next_line_start_index - 1
      }
    }
  }

  fn line_and_column_index(&self, pos: SourcePos) -> LineAndColumnIndex {
    let line_index = SourceFile::line_index(self, pos);
    let line_begin = SourceFile::line_start(self, line_index);
    if pos == line_begin {
      return LineAndColumnIndex {
        line_index,
        column_index: 0,
      };
    }
    let start_pos = self.range().start;
    let line_end = if line_index + 1 >= self.lines_count() {
      start_pos + self.text().len()
    } else {
      // better to include the newline portion
      SourceFile::line_start(self, line_index + 1)
    };
    let relative_line_begin = line_begin - start_pos;
    let relative_line_end = line_end - start_pos;

    // ensure no panics will happen here in case someone is specifying a byte position in the middle of a char
    let line_text = &self.text()[relative_line_begin..relative_line_end];
    let column_index = if pos == line_end {
      line_text.chars().count()
    } else {
      line_text
        .char_indices()
        .take_while(|(c_pos, _)| line_begin + *c_pos <= pos)
        .count()
        - 1
    };

    LineAndColumnIndex {
      line_index,
      column_index,
    }
  }
}

pub struct SourceFileTextInfo {
  start_pos: SourcePos,
  text: String,
  text_lines: TextLines,
}

impl SourceFileTextInfo {
  pub fn new(start_pos: SourcePos, text: String) -> Self {
    SourceFileTextInfo::with_indent_width(start_pos, text, 4)
  }

  pub fn with_indent_width(start_pos: SourcePos, text: String, indent_width: usize) -> Self {
    Self {
      start_pos,
      text_lines: TextLines::with_indent_width(&text, indent_width),
      text,
    }
  }

  fn assert_pos(&self, pos: SourcePos) {
    let range = self.range();
    if pos < range.start {
      panic!(
        "The provided position {} was less than the start position {}.",
        pos, range.start,
      );
    } else if pos > range.end {
      panic!(
        "The provided position {} was greater than the end position {}.",
        pos, range.end,
      );
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

impl std::fmt::Debug for SourceFileTextInfo {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SourceFileTextInfo")
      .field("start_pos", &self.start_pos)
      .field("text", &self.text)
      .finish()
  }
}

impl SourceFile for SourceFileTextInfo {
  fn text(&self) -> &str {
    &self.text
  }

  fn range(&self) -> SourceRange {
    SourceRange::new(self.start_pos, self.start_pos + self.text.len())
  }

  fn lines_count(&self) -> usize {
    self.text_lines.lines_count()
  }

  fn line_index(&self, pos: SourcePos) -> usize {
    self.assert_pos(pos);
    self
      .text_lines
      .line_index(self.get_relative_index_from_pos(pos))
  }

  fn line_start(&self, line_index: usize) -> SourcePos {
    self.assert_line_index(line_index);
    self.get_pos_from_relative_index(self.text_lines.line_start(line_index))
  }

  fn line_end(&self, line_index: usize) -> SourcePos {
    self.assert_line_index(line_index);
    self.get_pos_from_relative_index(self.text_lines.line_end(line_index))
  }

  fn line_and_column_index(&self, pos: SourcePos) -> LineAndColumnIndex {
    self.assert_pos(pos);
    self
      .text_lines
      .line_and_column_index(self.get_relative_index_from_pos(pos))
  }
}

#[cfg(test)]
mod test {
  use super::*;

  use swc_common::BytePos;

  #[test]
  fn line_and_column_index() {
    let text = "12\n3\r\nβ\n5";
    for i in 0..10 {
      run_with_source_file(
        SourceFileTextInfo::new(SourcePos::new(i), text.to_string()),
        i,
      );
      run_with_source_file(new_swc_source_file(i, text), i);
    }

    fn run_with_source_file(source_file: impl SourceFile, i: usize) {
      assert_pos_line_and_col(&source_file, i, 0, 0); // 1
      assert_pos_line_and_col(&source_file, 1 + i, 0, 1); // 2
      assert_pos_line_and_col(&source_file, 2 + i, 0, 2); // \n
      assert_pos_line_and_col(&source_file, 3 + i, 1, 0); // 3
      assert_pos_line_and_col(&source_file, 4 + i, 1, 1); // \r
      assert_pos_line_and_col(&source_file, 5 + i, 1, 2); // \n
      assert_pos_line_and_col(&source_file, 6 + i, 2, 0); // first β index
      assert_pos_line_and_col(&source_file, 7 + i, 2, 0); // second β index
      assert_pos_line_and_col(&source_file, 8 + i, 2, 1); // \n
      assert_pos_line_and_col(&source_file, 9 + i, 3, 0); // 5
      assert_pos_line_and_col(&source_file, 10 + i, 3, 1); // <EOF>
    }
  }

  fn assert_pos_line_and_col(
    source_file: &impl SourceFile,
    pos: usize,
    line_index: usize,
    column_index: usize,
  ) {
    assert_eq!(
      source_file.line_and_column_index(SourcePos::new(pos)),
      LineAndColumnIndex {
        line_index,
        column_index,
      }
    );
  }

  #[test]
  #[should_panic(expected = "The provided position 0 was less than the start position 1.")]
  fn line_and_column_index_panic_less_than() {
    let info = SourceFileTextInfo::new(SourcePos::new(1), "test".to_string());
    info.line_and_column_index(SourcePos::new(0));
  }

  #[test]
  #[should_panic(expected = "The provided position 6 was greater than the end position 5.")]
  fn line_and_column_index_panic_greater_than() {
    let info = SourceFileTextInfo::new(SourcePos::new(1), "test".to_string());
    info.line_and_column_index(SourcePos::new(6));
  }

  #[test]
  fn line_start() {
    let text = "12\n3\r\n4\n5";
    for i in 0..10 {
      run_with_source_file(
        SourceFileTextInfo::new(SourcePos::new(i), text.to_string()),
        i,
      );
      run_with_source_file(new_swc_source_file(i, text), i);
    }

    fn run_with_source_file(source_file: impl SourceFile, i: usize) {
      assert_line_start(&source_file, 0, i);
      assert_line_start(&source_file, 1, 3 + i);
      assert_line_start(&source_file, 2, 6 + i);
      assert_line_start(&source_file, 3, 8 + i);
    }
  }

  fn assert_line_start(source_file: &impl SourceFile, line_index: usize, line_end: usize) {
    assert_eq!(source_file.line_start(line_index), SourcePos::new(line_end));
  }

  #[test]
  #[should_panic(
    expected = "The specified line index 1 was greater or equal to the number of lines of 1."
  )]
  fn line_start_equal_number_lines() {
    let info = SourceFileTextInfo::new(SourcePos::new(1), "test".to_string());
    info.line_start(1);
  }

  #[test]
  fn line_end() {
    let text = "12\n3\r\n4\n5";
    for i in 0..10 {
      run_with_source_file(
        SourceFileTextInfo::new(SourcePos::new(i), text.to_string()),
        i,
      );
      run_with_source_file(new_swc_source_file(i, text), i);
    }

    fn run_with_source_file(source_file: impl SourceFile, i: usize) {
      assert_line_end(&source_file, 0, 2 + i);
      assert_line_end(&source_file, 1, 4 + i);
      assert_line_end(&source_file, 2, 7 + i);
      assert_line_end(&source_file, 3, 9 + i);
    }
  }

  fn assert_line_end(source_file: &impl SourceFile, line_index: usize, line_end: usize) {
    assert_eq!(source_file.line_end(line_index), SourcePos::new(line_end));
  }

  #[test]
  #[should_panic(
    expected = "The specified line index 1 was greater or equal to the number of lines of 1."
  )]
  fn line_end_equal_number_lines() {
    let info = SourceFileTextInfo::new(SourcePos::new(1), "test".to_string());
    info.line_end(1);
  }

  fn new_swc_source_file(start_pos: usize, text: &str) -> swc_common::SourceFile {
    swc_common::SourceFile::new(
      swc_common::FileName::Custom("test.ts".to_string()),
      false,
      swc_common::FileName::Custom("test.ts".to_string()),
      text.to_string(),
      SourcePos::START_BYTE_POS + BytePos(start_pos as u32),
    )
  }
}
