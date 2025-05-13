use crate::swc::ast::EsVersion;
use crate::swc::ast::Module;
use crate::swc::common::comments::SingleThreadedComments;
use crate::swc::common::errors::DiagnosticBuilder;
use crate::swc::common::errors::Emitter;
use crate::swc::common::errors::Handler;
use crate::swc::parser::lexer::Lexer;
use crate::swc::parser::token::TokenAndSpan;
use crate::swc::parser::Capturing;
use crate::swc::parser::Parser;
use crate::swc::parser::Syntax;
use std::path::Path;

use crate::common::SourceTextInfo;

pub fn get_swc_module(file_path: &Path, file_text: &str) -> (Module, Vec<TokenAndSpan>, SourceTextInfo, SingleThreadedComments) {
  // lifted from dprint-plugin-typescript
  let handler = Handler::with_emitter(false, false, Box::new(EmptyEmitter {}));
  let source_text_info = SourceTextInfo::from_string(file_text.to_string());

  let comments: SingleThreadedComments = Default::default();
  return {
    let ts_config = crate::swc::parser::TsSyntax {
      tsx: should_parse_as_jsx(file_path),
      decorators: true,
      ..Default::default()
    };
    let lexer = Lexer::new(
      Syntax::Typescript(ts_config),
      EsVersion::Es2022,
      source_text_info.as_string_input(),
      Some(&comments),
    );
    let lexer = Capturing::new(lexer);
    let mut parser = Parser::new_from(lexer);
    let parse_module_result = parser.parse_module();
    let tokens = parser.input().take();

    match parse_module_result {
      Err(error) => {
        // mark the diagnostic as being handled (otherwise it will panic in its drop)
        let mut diagnostic = error.into_diagnostic(&handler);
        diagnostic.cancel();
        // return the formatted diagnostic string
        Err(diagnostic.message())
      }
      Ok(module) => Ok((module, tokens, source_text_info, comments)),
    }
  }
  .unwrap();
}

#[cfg(feature = "view")]
pub fn get_swc_script(file_path: &Path, file_text: &str) -> (crate::swc::ast::Script, Vec<TokenAndSpan>, SourceTextInfo, SingleThreadedComments) {
  let handler = Handler::with_emitter(false, false, Box::new(EmptyEmitter {}));
  let source_text_info = SourceTextInfo::from_string(file_text.to_string());

  let comments: SingleThreadedComments = Default::default();
  return {
    let ts_config = crate::swc::parser::TsSyntax {
      tsx: should_parse_as_jsx(file_path),
      decorators: true,
      ..Default::default()
    };
    let lexer = Lexer::new(
      Syntax::Typescript(ts_config),
      EsVersion::Es2022,
      source_text_info.as_string_input(),
      Some(&comments),
    );
    let lexer = Capturing::new(lexer);
    let mut parser = Parser::new_from(lexer);
    let parse_script_result = parser.parse_script();
    let tokens = parser.input().take();

    match parse_script_result {
      Err(error) => {
        // mark the diagnostic as being handled (otherwise it will panic in its drop)
        let mut diagnostic = error.into_diagnostic(&handler);
        diagnostic.cancel();
        // return the formatted diagnostic string
        Err(diagnostic.message())
      }
      Ok(script) => Ok((script, tokens, source_text_info, comments)),
    }
  }
  .unwrap();
}

fn should_parse_as_jsx(file_path: &Path) -> bool {
  if let Some(extension) = get_lowercase_extension(file_path) {
    return extension == "tsx" || extension == "jsx" || extension == "js" || extension == "mjs";
  }
  true
}

fn get_lowercase_extension(file_path: &Path) -> Option<String> {
  file_path.extension().and_then(|e| e.to_str()).map(|f| f.to_lowercase())
}

pub struct EmptyEmitter {}

impl Emitter for EmptyEmitter {
  fn emit(&mut self, _: &mut DiagnosticBuilder<'_>) {
    // for now, we don't care about diagnostics so do nothing
  }

  fn should_show_explain(&self) -> bool {
    false
  }
}
