use std::sync::Arc;
use std::sync::OnceLock;

use crate::common::SourceTextInfo;
use crate::common::StartSourcePos;
use crate::swc::ast::EsVersion;
use crate::swc::ast::Program;
use crate::swc::common::comments::SingleThreadedComments;
use crate::swc::common::input::StringInput;
use crate::swc::lexer::common::parser::Parser as _;
use crate::swc::parser::error::Error as SwcError;
use crate::swc::parser::token::TokenAndSpan;
use crate::swc::parser::Syntax;

use super::ParseDiagnostic;
use super::ParsedComments;
use super::ParsedSource;

/// Whether to parse the source as a module, a script, or let swc decide.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseMode {
  Program,
  Module,
  Script,
}

/// Parameters for parsing.
pub struct ParseParams {
  /// Specifier of the source text. This is only used for display
  /// purposes in diagnostics.
  pub specifier: String,
  /// Source text. Any BOM must be stripped before providing it here.
  pub text: Arc<str>,
  /// Syntax to use when parsing.
  pub syntax: Syntax,
  /// Ecmascript version to use when lexing.
  pub es_version: EsVersion,
  /// Whether to capture tokens or not. Tokens must be captured in
  /// order to use the comments of an ast view.
  pub capture_tokens: bool,
  /// Whether to parse as a module, a script, or let swc decide.
  pub mode: ParseMode,
}

/// Parses the provided text, returning an error only when the source
/// could not be parsed at all.
///
/// Non-fatal syntax errors are surfaced via `ParsedSource::diagnostics`.
pub fn parse(params: ParseParams) -> Result<ParsedSource, ParseDiagnostic> {
  let ParseParams {
    specifier,
    text,
    syntax,
    es_version,
    capture_tokens,
    mode,
  } = params;
  debug_assert!(!text.starts_with('\u{FEFF}'), "BOM should be stripped before parsing");
  let input = StringInput::new(
    text.as_ref(),
    StartSourcePos::START_SOURCE_POS.as_byte_pos(),
    (StartSourcePos::START_SOURCE_POS + text.len()).as_byte_pos(),
  );
  let (comments, program, tokens, errors) = parse_string_input(input, syntax, es_version, capture_tokens, mode).map_err(|err| {
    let text_info = SourceTextInfo::new(text.clone());
    ParseDiagnostic::from_swc_error(err, &specifier, text_info)
  })?;
  // pre-populate the text info when it had to be created for diagnostics anyway
  let text_info: OnceLock<SourceTextInfo> = Default::default();
  let diagnostics = if errors.is_empty() {
    Vec::new()
  } else {
    let source_text_info = SourceTextInfo::new(text.clone());
    let diagnostics = errors
      .into_iter()
      .map(|err| ParseDiagnostic::from_swc_error(err, &specifier, source_text_info.clone()))
      .collect();
    text_info.set(source_text_info).unwrap();
    diagnostics
  };

  Ok(ParsedSource {
    specifier,
    text,
    text_info,
    program,
    comments: ParsedComments::from_single_threaded(comments),
    tokens,
    diagnostics,
  })
}

#[allow(clippy::type_complexity)]
fn parse_string_input(
  input: StringInput,
  syntax: Syntax,
  es_version: EsVersion,
  capture_tokens: bool,
  mode: ParseMode,
) -> Result<(SingleThreadedComments, Program, Option<Vec<TokenAndSpan>>, Vec<SwcError>), SwcError> {
  let comments = SingleThreadedComments::default();

  // `parse_program`, `parse_module`, and `parse_script` are inherent methods
  // on the concrete parsers rather than part of a shared trait
  macro_rules! parse_with_mode {
    ($parser:expr) => {
      match mode {
        ParseMode::Program => $parser.parse_program(),
        ParseMode::Module => $parser.parse_module().map(Program::Module),
        ParseMode::Script => $parser.parse_script().map(Program::Script),
      }
    };
  }

  if capture_tokens {
    let lexer = crate::swc::lexer::Lexer::new(syntax, es_version, input, Some(&comments));
    let lexer = crate::swc::lexer::Capturing::new(lexer);
    let mut parser = crate::swc::lexer::Parser::new_from(lexer);
    let program = parse_with_mode!(parser)?;
    let iter = &mut parser.input_mut().iter;
    let tokens = crate::swc::lexer::Capturing::take(iter);
    let errors = parser.take_errors();

    Ok((comments, program, Some(tokens), errors))
  } else {
    let lexer = crate::swc::parser::Lexer::new(syntax, es_version, input, Some(&comments));
    let mut parser = crate::swc::parser::Parser::new_from(lexer);
    let program = parse_with_mode!(parser)?;
    let errors = parser.take_errors();

    Ok((comments, program, None, errors))
  }
}

#[cfg(test)]
mod test {
  use pretty_assertions::assert_eq;

  use super::*;
  use crate::swc::parser::TsSyntax;

  #[test]
  fn parses_with_tokens_and_comments() {
    let parsed_source = parse_ts("// 1\n1 + 1\n// 2").unwrap();
    assert_eq!(parsed_source.specifier(), "file:///my_file.ts");
    assert_eq!(parsed_source.text().as_ref(), "// 1\n1 + 1\n// 2");
    assert_eq!(parsed_source.tokens().len(), 3);
    assert_eq!(parsed_source.comments().leading_map().len(), 1);
    assert_eq!(parsed_source.comments().trailing_map().len(), 1);
    assert!(parsed_source.diagnostics().is_empty());
  }

  #[test]
  fn surfaces_non_fatal_diagnostics() {
    let parsed_source = parse_ts("var foo = 'test").unwrap();
    assert_eq!(parsed_source.diagnostics().len(), 1);
    assert_eq!(
      parsed_source.diagnostics()[0].to_string(),
      concat!(
        "Unterminated string constant at file:///my_file.ts:1:11\n\n",
        "  var foo = 'test\n",
        "            ~~~~~"
      )
    );
  }

  #[test]
  fn errors_for_fatal_diagnostic() {
    let diagnostic = parse_ts("test;\nas#;").err().unwrap();
    assert_eq!(diagnostic.message(), "Expected ';', '}' or <eof>");
    assert_eq!(diagnostic.display_position().line_number, 2);
    assert_eq!(diagnostic.display_position().column_number, 3);
  }

  #[cfg(feature = "view")]
  #[test]
  fn provides_an_ast_view() {
    use crate::common::SourceRanged;
    use crate::view::NodeTrait;

    let parsed_source = parse_ts("// 1\n1 + 1").unwrap();
    parsed_source.with_view(|program| {
      let children = program.children();
      assert_eq!(children.len(), 1);
      assert_eq!(children[0].text_fast(program), "1 + 1");
      assert_eq!(children[0].start().leading_comments_fast(program).count(), 1);
    });
  }

  fn parse_ts(text: &str) -> Result<ParsedSource, ParseDiagnostic> {
    parse(ParseParams {
      specifier: "file:///my_file.ts".to_string(),
      text: text.into(),
      syntax: Syntax::Typescript(TsSyntax::default()),
      es_version: EsVersion::Es2021,
      capture_tokens: true,
      mode: ParseMode::Program,
    })
  }
}
