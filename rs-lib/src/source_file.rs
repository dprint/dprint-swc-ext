use swc_common::BytePos;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineAndColumnIndex {
  pub line_index: usize,
  pub column_index: usize,
}

pub trait SourceFile {
  fn text(&self) -> &str;
  fn start_pos(&self) -> BytePos;
  fn end_pos(&self) -> BytePos;
  fn lines_count(&self) -> usize;
  fn line_index(&self, pos: BytePos) -> usize;
  fn line_begin_pos(&self, line_index: usize) -> BytePos;

  fn line_and_column_index(&self, pos: BytePos) -> LineAndColumnIndex {
    let line_index = SourceFile::line_index(self, pos);
    let line_begin = SourceFile::line_begin_pos(self, line_index);
    if pos == line_begin {
      return LineAndColumnIndex {
        line_index,
        column_index: 0,
      };
    }
    let line_end = if line_index + 1 >= self.lines_count() {
      self.start_pos() + BytePos(self.text().len() as u32)
    } else {
      // this is fine
      SourceFile::line_begin_pos(self, line_index + 1)
    };
    let relative_line_begin = line_begin - self.start_pos();
    let relative_line_end = line_end - self.start_pos();

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

  fn start_pos(&self) -> BytePos {
    self.start_pos
  }

  fn end_pos(&self) -> BytePos {
    self.end_pos
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

  fn line_begin_pos(&self, line_index: usize) -> BytePos {
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
  pub fn new(start_pos: BytePos, text: &str) -> Self {
    Self {
      line_start_byte_positions: get_line_start_positions(start_pos, text),
      text: text.to_string(),
    }
  }
}

impl SourceFile for SourceFileTextInfo {
  fn text(&self) -> &str {
    &self.text
  }

  fn start_pos(&self) -> BytePos {
    self.line_start_byte_positions[0]
  }

  fn end_pos(&self) -> BytePos {
    self.start_pos() + BytePos(self.text.len() as u32)
  }

  fn lines_count(&self) -> usize {
    self.line_start_byte_positions.len()
  }

  fn line_index(&self, pos: BytePos) -> usize {
    if pos < self.start_pos() {
      panic!(
        "the provided position {} was less than the start position {}",
        pos.0,
        self.start_pos().0
      );
    } else if pos > self.end_pos() {
      panic!(
        "the provided position {} was greater than the end position {}",
        pos.0,
        self.end_pos().0
      );
    }

    match self.line_start_byte_positions.binary_search(&pos) {
      Ok(index) => index,
      Err(insert_index) => insert_index - 1,
    }
  }

  fn line_begin_pos(&self, line_index: usize) -> BytePos {
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
      let info = SourceFileTextInfo::new(BytePos(0 + i), text);
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

  #[test]
  #[should_panic(expected = "the provided position 0 was less than the start position 1")]
  fn line_and_column_index_panic_less_than() {
    let info = SourceFileTextInfo::new(BytePos(1), "test");
    info.line_and_column_index(BytePos(0));
  }

  #[test]
  #[should_panic(expected = "the provided position 6 was greater than the end position 5")]
  fn line_and_column_index_panic_greater_than() {
    let info = SourceFileTextInfo::new(BytePos(1), "test");
    info.line_and_column_index(BytePos(6));
  }

  #[test]
  #[should_panic(
    expected = "the specified line index 1 was greater or equal to the number of lines (1)"
  )]
  fn line_begin_pos_equal_number_lines() {
    let info = SourceFileTextInfo::new(BytePos(1), "test");
    info.line_begin_pos(1);
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
}
