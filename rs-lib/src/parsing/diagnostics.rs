use std::borrow::Cow;
use std::fmt;

use crate::common::LineAndColumnDisplay;
use crate::common::SourceRange;
use crate::common::SourceRangedForSpanned;
use crate::common::SourceTextInfo;
use crate::swc::parser::error::Error as SwcError;
use crate::swc::parser::error::SyntaxError;

/// A syntax error that occurred while parsing.
#[derive(Debug, Clone)]
pub struct ParseDiagnostic(Box<ParseDiagnosticInner>);

#[derive(Debug, Clone)]
struct ParseDiagnosticInner {
  specifier: String,
  range: SourceRange,
  kind: SyntaxError,
  source: SourceTextInfo,
}

impl ParseDiagnostic {
  pub fn from_swc_error(err: SwcError, specifier: &str, source: SourceTextInfo) -> ParseDiagnostic {
    ParseDiagnostic(Box::new(ParseDiagnosticInner {
      range: err.range(),
      specifier: specifier.to_string(),
      kind: err.into_kind(),
      source,
    }))
  }

  /// Specifier of the source the diagnostic occurred in.
  pub fn specifier(&self) -> &str {
    &self.0.specifier
  }

  /// Range of the diagnostic.
  pub fn range(&self) -> SourceRange {
    self.0.range
  }

  /// Swc syntax error.
  pub fn kind(&self) -> &SyntaxError {
    &self.0.kind
  }

  /// Text info of the source the diagnostic occurred in.
  pub fn source(&self) -> &SourceTextInfo {
    &self.0.source
  }

  /// Human readable message describing the error.
  pub fn message(&self) -> Cow<'_, str> {
    self.0.kind.msg()
  }

  /// 1-indexed display position the diagnostic occurred at.
  pub fn display_position(&self) -> LineAndColumnDisplay {
    self.0.source.line_and_column_display(self.0.range.start)
  }
}

impl Eq for ParseDiagnostic {}

impl PartialEq for ParseDiagnostic {
  fn eq(&self, other: &Self) -> bool {
    // excludes the source
    self.0.specifier == other.0.specifier && self.0.range == other.0.range && self.0.kind == other.0.kind
  }
}

impl std::error::Error for ParseDiagnostic {}

impl fmt::Display for ParseDiagnostic {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display_position = self.display_position();
    write!(
      f,
      "{} at {}:{}:{}\n\n{}",
      self.message(),
      self.specifier(),
      display_position.line_number,
      display_position.column_number,
      get_range_text_highlight(self.source(), self.range())
        .lines()
        // indent two spaces
        .map(|l| if l.trim().is_empty() { String::new() } else { format!("  {}", l) })
        .collect::<Vec<_>>()
        .join("\n"),
    )
  }
}

/// Multiple syntax errors that occurred while parsing.
#[derive(Debug, Clone)]
pub struct ParseDiagnosticsError(pub Vec<ParseDiagnostic>);

impl std::error::Error for ParseDiagnosticsError {}

impl fmt::Display for ParseDiagnosticsError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for (i, diagnostic) in self.0.iter().enumerate() {
      if i > 0 {
        write!(f, "\n\n")?;
      }

      write!(f, "{}", diagnostic)?
    }

    Ok(())
  }
}

/// Code in this function was adapted from:
/// https://github.com/dprint/dprint/blob/a026a1350d27a61ea18207cb31897b18eaab51a1/crates/core/src/formatting/utils/string_utils.rs#L62
fn get_range_text_highlight(source: &SourceTextInfo, byte_range: SourceRange) -> String {
  let (sub_text, (error_start, error_end)) = get_text_and_error_range(source, byte_range);

  let mut result = String::new();
  // don't use .lines() here because it will trim any empty
  // lines, which might for some reason be part of the range
  let lines = sub_text.split('\n').collect::<Vec<_>>();
  let line_count = lines.len();
  for (i, mut line) in lines.into_iter().enumerate() {
    if line.ends_with('\r') {
      line = &line[..line.len() - 1]; // trim the \r
    }
    let is_last_line = i == line_count - 1;
    // don't show all the lines if there are more than 3 lines
    if i > 2 && !is_last_line {
      continue;
    }
    if i > 0 {
      result.push('\n');
    }
    if i == 2 && !is_last_line {
      result.push_str("...");
      continue;
    }

    let mut error_start_char_index = if i == 0 { get_column_index_of_pos(sub_text, error_start) } else { 0 };
    let mut error_end_char_index = if is_last_line {
      get_column_index_of_pos(sub_text, error_end)
    } else {
      line.chars().count()
    };
    let line_char_count = line.chars().count();
    if line_char_count > 90 {
      let start_char_index = if error_start_char_index > 60 {
        std::cmp::min(error_start_char_index - 20, line_char_count - 80)
      } else {
        0
      };
      error_start_char_index -= start_char_index;
      error_end_char_index -= start_char_index;
      let code_text = line.chars().skip(start_char_index).take(80).collect::<String>();
      let mut line_text = String::new();
      if start_char_index > 0 {
        line_text.push_str("...");
        error_start_char_index += 3;
        error_end_char_index += 3;
      }
      line_text.push_str(&code_text);
      if line_char_count > start_char_index + code_text.chars().count() {
        error_end_char_index = std::cmp::min(error_end_char_index, line_text.chars().count());
        line_text.push_str("...");
      }
      result.push_str(&line_text);
    } else {
      result.push_str(line);
    }
    result.push('\n');

    result.push_str(&" ".repeat(error_start_char_index));
    result.push_str(&"~".repeat(std::cmp::max(
      1, // this means it's the end of the line, so display a single ~
      error_end_char_index - error_start_char_index,
    )));
  }
  result
}

fn get_text_and_error_range(source: &SourceTextInfo, byte_range: SourceRange) -> (&str, (usize, usize)) {
  let mut first_line_index = source.line_index(byte_range.start);
  let mut first_line_start = source.line_start(first_line_index);
  let last_line_end = source.line_end(source.line_index(byte_range.end));
  let mut sub_text = source.range_text(&SourceRange::new(first_line_start, last_line_end));

  // while the text is empty, show the previous line
  while sub_text.trim().is_empty() && first_line_index > 0 {
    first_line_index -= 1;
    first_line_start = source.line_start(first_line_index);
    sub_text = source.range_text(&SourceRange::new(first_line_start, last_line_end));
  }

  let error_start = byte_range.start - first_line_start;
  let error_end = error_start + (byte_range.end - byte_range.start);
  (sub_text, (error_start, error_end))
}

fn get_column_index_of_pos(text: &str, pos: usize) -> usize {
  let line_start_byte_pos = get_line_start_byte_pos(text, pos);
  text[line_start_byte_pos..pos].chars().count()
}

fn get_line_start_byte_pos(text: &str, pos: usize) -> usize {
  let text_bytes = text.as_bytes();
  for i in (0..pos).rev() {
    if text_bytes.get(i) == Some(&b'\n') {
      return i + 1;
    }
  }

  0
}

#[cfg(test)]
mod test {
  use pretty_assertions::assert_eq;

  use super::*;

  #[test]
  fn range_highlight_all_text() {
    let text = SourceTextInfo::from_string(
      concat!(
        "Line 0 - Testing this out with a long line testing0 testing1 testing2 testing3 testing4 testing5 testing6\n",
        "Line 1\n",
        "Line 2\n",
        "Line 3\n",
        "Line 4"
      )
      .to_string(),
    );
    assert_eq!(
      get_range_text_highlight(&text, SourceRange::new(text.line_start(0), text.line_end(4))),
      concat!(
        "Line 0 - Testing this out with a long line testing0 testing1 testing2 testing3 t...\n",
        "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n",
        "Line 1\n",
        "~~~~~~\n",
        "...\n",
        "Line 4\n",
        "~~~~~~"
      )
    );
  }

  #[test]
  fn range_highlight_single_line() {
    let text = SourceTextInfo::from_string("const t = 5;".to_string());
    assert_eq!(
      get_range_text_highlight(&text, SourceRange::new(text.line_start(0) + 6, text.line_start(0) + 7)),
      concat!("const t = 5;\n", "      ~")
    );
  }
}
