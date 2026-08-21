use std::sync::Arc;
use std::sync::OnceLock;

use crate::common::SourcePos;
use crate::common::SourceRange;
use crate::common::SourceRanged;
use crate::common::SourceTextInfo;
use crate::common::SourceTextProvider;
use crate::common::StartSourcePos;
use crate::swc::ast::Module;
use crate::swc::ast::Program;
use crate::swc::ast::Script;
use crate::swc::common::comments::SingleThreadedComments;
use crate::swc::common::comments::SingleThreadedCommentsMapInner;
use crate::swc::parser::token::TokenAndSpan;

use super::ParseDiagnostic;

/// A parsed source containing an AST, comments, and possibly tokens.
pub struct ParsedSource {
  pub(crate) specifier: String,
  pub(crate) text: Arc<str>,
  pub(crate) text_info: OnceLock<SourceTextInfo>,
  pub(crate) program: Program,
  pub(crate) comments: ParsedComments,
  pub(crate) tokens: Option<Vec<TokenAndSpan>>,
  pub(crate) diagnostics: Vec<ParseDiagnostic>,
}

impl ParsedSource {
  /// Specifier of the source that was parsed.
  pub fn specifier(&self) -> &str {
    &self.specifier
  }

  /// Text of the source that was parsed.
  pub fn text(&self) -> &Arc<str> {
    &self.text
  }

  /// Gets an object with pre-computed positions for lines and indexes of
  /// multi-byte chars.
  ///
  /// Note: Prefer using `.text()` over this if able because this is lazily
  /// created.
  pub fn text_info(&self) -> &SourceTextInfo {
    self.text_info.get_or_init(|| SourceTextInfo::new(self.text.clone()))
  }

  /// Range of the parsed source.
  pub fn range(&self) -> SourceRange<StartSourcePos> {
    SourceRange::new(StartSourcePos::START_SOURCE_POS, StartSourcePos::START_SOURCE_POS + self.text.len())
  }

  /// Gets the parsed program.
  pub fn program(&self) -> &Program {
    &self.program
  }

  /// Gets the parsed program when it was parsed as a module.
  ///
  /// This will panic if the source was parsed as a script.
  pub fn module(&self) -> &Module {
    match &self.program {
      Program::Module(module) => module,
      Program::Script(_) => panic!("Cannot get a module when the source was a script."),
    }
  }

  /// Gets the parsed program when it was parsed as a script.
  ///
  /// This will panic if the source was parsed as a module.
  pub fn script(&self) -> &Script {
    match &self.program {
      Program::Script(script) => script,
      Program::Module(_) => panic!("Cannot get a script when the source was a module."),
    }
  }

  /// Gets the comments found in the source file.
  pub fn comments(&self) -> &ParsedComments {
    &self.comments
  }

  /// Gets the tokens found in the source file.
  ///
  /// This will panic if tokens were not captured during parsing.
  pub fn tokens(&self) -> &[TokenAndSpan] {
    self.tokens.as_deref().expect("Tokens not found because they were not captured during parsing.")
  }

  /// Gets extra non-fatal diagnostics found while parsing.
  pub fn diagnostics(&self) -> &[ParseDiagnostic] {
    &self.diagnostics
  }

  /// Gets a dprint-swc-ext view of the program.
  ///
  /// This provides a closure to examine an "ast view" of the swc AST
  /// which has more helper methods and allows for going up the ancestors
  /// of a node.
  #[cfg(feature = "view")]
  pub fn with_view<'a, T>(&self, with_view: impl FnOnce(crate::view::Program<'a>) -> T) -> T {
    let program_info = crate::view::ProgramInfo {
      program: match &self.program {
        Program::Module(module) => crate::view::ProgramRef::Module(module),
        Program::Script(script) => crate::view::ProgramRef::Script(script),
      },
      text_info: Some(self.text_info()),
      tokens: self.tokens.as_deref(),
      comments: Some(crate::view::Comments {
        leading: &self.comments.leading,
        trailing: &self.comments.trailing,
      }),
    };

    crate::view::with_ast_view(program_info, with_view)
  }
}

impl<'a> SourceTextProvider<'a> for &'a ParsedSource {
  fn text(&self) -> &'a Arc<str> {
    ParsedSource::text(self)
  }

  fn start_pos(&self) -> StartSourcePos {
    StartSourcePos::START_SOURCE_POS
  }
}

impl SourceRanged for ParsedSource {
  fn start(&self) -> SourcePos {
    StartSourcePos::START_SOURCE_POS.as_source_pos()
  }

  fn end(&self) -> SourcePos {
    StartSourcePos::START_SOURCE_POS + self.text.len()
  }
}

impl std::fmt::Debug for ParsedSource {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_struct("ParsedSource")
      .field("specifier", &self.specifier)
      .field("program", &self.program)
      .finish()
  }
}

/// The leading and trailing comments found while parsing.
#[derive(Debug, Default)]
pub struct ParsedComments {
  pub(crate) leading: SingleThreadedCommentsMapInner,
  pub(crate) trailing: SingleThreadedCommentsMapInner,
}

impl ParsedComments {
  pub fn from_single_threaded(comments: SingleThreadedComments) -> Self {
    let (leading, trailing) = comments.take_all();
    Self {
      leading: std::rc::Rc::try_unwrap(leading).unwrap().into_inner(),
      trailing: std::rc::Rc::try_unwrap(trailing).unwrap().into_inner(),
    }
  }

  /// Map of comments that appear before a position.
  pub fn leading_map(&self) -> &SingleThreadedCommentsMapInner {
    &self.leading
  }

  /// Map of comments that appear after a position.
  pub fn trailing_map(&self) -> &SingleThreadedCommentsMapInner {
    &self.trailing
  }
}
