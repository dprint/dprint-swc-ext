use swc_common::{BytePos, Span};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineAndColumnIndex {
  pub line_index: usize,
  pub column_index: usize,
}

pub trait SourceFile {
  fn text(&self) -> &str;
  fn span(&self) -> Span;
  fn lines_count(&self) -> usize;
  fn line_index(&self, pos: BytePos) -> usize;
  fn line_start(&self, line_index: usize) -> BytePos;

  fn line_end(&self, line_index: usize) -> BytePos {
    let lines_count = self.lines_count();
    let start_pos = self.span().lo();
    if line_index >= lines_count {
      panic!(
        "the specified line index {} was greater or equal to the number of lines ({})",
        line_index,
        lines_count,
      );
    } else if line_index + 1 == lines_count {
      BytePos(self.text().len() as u32) + start_pos
    } else {
      let text = self.text().as_bytes();
      let next_line_start_index = (self.line_start(line_index + 1) - start_pos).0 as usize;
      if next_line_start_index > 2 && text[next_line_start_index - 2] == b'\r' {
        BytePos(next_line_start_index as u32 - 2) + start_pos
      } else {
        BytePos(next_line_start_index as u32 - 1) + start_pos
      }
    }
  }

  fn line_and_column_index(&self, pos: BytePos) -> LineAndColumnIndex {
    let line_index = SourceFile::line_index(self, pos);
    let line_begin = SourceFile::line_start(self, line_index);
    if pos == line_begin {
      return LineAndColumnIndex {
        line_index,
        column_index: 0,
      };
    }
    let span = self.span();
    let line_end = if line_index + 1 >= self.lines_count() {
      span.lo() + BytePos(self.text().len() as u32)
    } else {
      // better to include the newline portion
      SourceFile::line_start(self, line_index + 1)
    };
    let relative_line_begin = line_begin - span.lo();
    let relative_line_end = line_end - span.lo();

    // ensure no panics will happen here in case someone is specifying a byte position in the middle of a char
    let line_text = &self.text()[relative_line_begin.0 as usize..relative_line_end.0 as usize];
    let column_index = if pos == line_end {
      line_text.chars().count()
    } else {
      let line_begin_pos = line_begin.0 as usize;
      let pos = pos.0 as usize;
      line_text
        .char_indices()
        .position(|(c_pos, _)| line_begin_pos + c_pos >= pos)
        .unwrap()
    };

    LineAndColumnIndex {
      line_index,
      column_index,
    }
  }
}

impl SourceFile for swc_common::SourceFile {
  fn text(&self) -> &str {
    &self.src
  }

  fn span(&self) -> Span {
    Span::new(self.start_pos, self.end_pos, Default::default())
  }

  fn lines_count(&self) -> usize {
    swc_common::SourceFile::count_lines(self)
  }

  fn line_index(&self, pos: BytePos) -> usize {
    if let Some(line_index) = swc_common::SourceFile::lookup_line(self, pos) {
      line_index
    } else {
      panic!("could not find line index at pos {}", pos.0)
    }
  }

  fn line_start(&self, line_index: usize) -> BytePos {
    if line_index >= self.lines.len() {
      panic!(
        "the specified line index {} was greater or equal to the number of lines ({})",
        line_index,
        self.lines.len()
      );
    }
    self.lines[line_index]
  }
}

pub struct SourceFileTextInfo {
  text: String,
  line_start_byte_positions: Vec<BytePos>,
}

impl SourceFileTextInfo {
  pub fn new(start_pos: BytePos, text: String) -> Self {
    Self {
      line_start_byte_positions: get_line_start_positions(start_pos, &text),
      text,
    }
  }
}

impl SourceFile for SourceFileTextInfo {
  fn text(&self) -> &str {
    &self.text
  }

  fn span(&self) -> Span {
    let start = self.line_start_byte_positions[0];
    Span::new(
      start,
      start + BytePos(self.text.len() as u32),
      Default::default(),
    )
  }

  fn lines_count(&self) -> usize {
    self.line_start_byte_positions.len()
  }

  fn line_index(&self, pos: BytePos) -> usize {
    let span = self.span();
    if pos < span.lo() {
      panic!(
        "the provided position {} was less than the start position {}",
        pos.0,
        span.lo().0
      );
    } else if pos > span.hi() {
      panic!(
        "the provided position {} was greater than the end position {}",
        pos.0,
        span.hi().0
      );
    }

    match self.line_start_byte_positions.binary_search(&pos) {
      Ok(index) => index,
      Err(insert_index) => insert_index - 1,
    }
  }

  fn line_start(&self, line_index: usize) -> BytePos {
    if line_index >= self.line_start_byte_positions.len() {
      panic!(
        "the specified line index {} was greater or equal to the number of lines ({})",
        line_index,
        self.line_start_byte_positions.len()
      );
    }
    self.line_start_byte_positions[line_index]
  }
}

fn get_line_start_positions(start_pos: BytePos, text: &str) -> Vec<BytePos> {
  let mut result = vec![start_pos];
  for (index, c) in text.char_indices() {
    if c == '\n' {
      let line_start_pos = start_pos + BytePos((index + 1) as u32);
      result.push(line_start_pos);
    }
  }
  result
}

#[cfg(test)]
mod test {
  use super::*;

  use swc_common::BytePos;

  #[test]
  fn line_and_column_index() {
    let text = "12\n3\r\n4\n5";
    for i in 0..10 {
      let info = SourceFileTextInfo::new(BytePos(0 + i), text.to_string());
      assert_pos_line_and_col(&info, 0 + i, 0, 0); // 1
      assert_pos_line_and_col(&info, 1 + i, 0, 1); // 2
      assert_pos_line_and_col(&info, 2 + i, 0, 2); // \n
      assert_pos_line_and_col(&info, 3 + i, 1, 0); // 3
      assert_pos_line_and_col(&info, 4 + i, 1, 1); // \r
      assert_pos_line_and_col(&info, 5 + i, 1, 2); // \n
      assert_pos_line_and_col(&info, 6 + i, 2, 0); // 4
      assert_pos_line_and_col(&info, 7 + i, 2, 1); // \n
      assert_pos_line_and_col(&info, 8 + i, 3, 0); // 5
      assert_pos_line_and_col(&info, 9 + i, 3, 1); // <EOF>
    }
  }

  fn assert_pos_line_and_col(
    info: &SourceFileTextInfo,
    pos: u32,
    line_index: usize,
    column_index: usize,
  ) {
    assert_eq!(
      info.line_and_column_index(BytePos(pos)),
      LineAndColumnIndex {
        line_index,
        column_index,
      }
    );
  }

  #[test]
  #[should_panic(expected = "the provided position 0 was less than the start position 1")]
  fn line_and_column_index_panic_less_than() {
    let info = SourceFileTextInfo::new(BytePos(1), "test".to_string());
    info.line_and_column_index(BytePos(0));
  }

  #[test]
  #[should_panic(expected = "the provided position 6 was greater than the end position 5")]
  fn line_and_column_index_panic_greater_than() {
    let info = SourceFileTextInfo::new(BytePos(1), "test".to_string());
    info.line_and_column_index(BytePos(6));
  }

  #[test]
  fn line_start() {
    let text = "12\n3\r\n4\n5";
    for i in 0..10 {
      let info = SourceFileTextInfo::new(BytePos(0 + i), text.to_string());
      assert_line_start(&info, 0, BytePos(0 + i));
      assert_line_start(&info, 1, BytePos(3 + i));
      assert_line_start(&info, 2, BytePos(6 + i));
      assert_line_start(&info, 3, BytePos(8 + i));
    }
  }

  fn assert_line_start(
    info: &SourceFileTextInfo,
    line_index: usize,
    line_end: BytePos,
  ) {
    assert_eq!(
      info.line_start(line_index),
      line_end,
    );
  }

  #[test]
  #[should_panic(
    expected = "the specified line index 1 was greater or equal to the number of lines (1)"
  )]
  fn line_start_equal_number_lines() {
    let info = SourceFileTextInfo::new(BytePos(1), "test".to_string());
    info.line_start(1);
  }

  #[test]
  fn line_end() {
    let text = "12\n3\r\n4\n5";
    for i in 0..10 {
      let info = SourceFileTextInfo::new(BytePos(0 + i), text.to_string());
      assert_line_end(&info, 0, BytePos(2 + i));
      assert_line_end(&info, 1, BytePos(4 + i));
      assert_line_end(&info, 2, BytePos(7 + i));
      assert_line_end(&info, 3, BytePos(9 + i));
    }
  }

  fn assert_line_end(
    info: &SourceFileTextInfo,
    line_index: usize,
    line_end: BytePos,
  ) {
    assert_eq!(
      info.line_end(line_index),
      line_end,
    );
  }

  #[test]
  #[should_panic(
    expected = "the specified line index 1 was greater or equal to the number of lines (1)"
  )]
  fn line_end_equal_number_lines() {
    let info = SourceFileTextInfo::new(BytePos(1), "test".to_string());
    info.line_end(1);
  }
}
