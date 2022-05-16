use dprint_swc_ext::SourceTextInfo;

use crate::swc::ast::{EsVersion, Module, Script};
use crate::swc::common::{
  comments::SingleThreadedComments,
  errors::{DiagnosticBuilder, Emitter, Handler},
};
use crate::swc::parser::{lexer::Lexer, Capturing, Parser, StringInput, Syntax};
use std::path::Path;

use crate::{SourcePos, TokenAndRange};

#[cfg(feature = "serialize")]
use {super::*, std::path::PathBuf};

pub fn run_test(file_text: &str, run_test: impl Fn(super::Program)) {
  let file_path = Path::new("test.ts");
  run_test_with_module(file_path, file_text, |module| {
    run_test(super::Program::Module(module))
  });
  run_test_with_script(file_path, file_text, |script| {
    run_test(super::Program::Script(script))
  });
}

pub fn run_test_with_module(file_path: &Path, file_text: &str, run_test: impl Fn(&super::Module)) {
  let (module, tokens, text_info, comments) = get_swc_module(file_path, file_text);
  let (leading, trailing) = comments.borrow_all();
  let info = super::ModuleInfo {
    module: &module,
    text_info: Some(&text_info),
    tokens: Some(&tokens),
    comments: Some(super::Comments {
      leading: &leading,
      trailing: &trailing,
    }),
  };
  super::with_ast_view_for_module(info, |module| {
    run_test(module);
  });
}

pub fn run_test_with_script(file_path: &Path, file_text: &str, run_test: impl Fn(&super::Script)) {
  let (script, tokens, text_info, comments) = get_swc_script(file_path, file_text);
  let (leading, trailing) = comments.borrow_all();
  let info = super::ScriptInfo {
    script: &script,
    text_info: Some(&text_info),
    tokens: Some(&tokens),
    comments: Some(super::Comments {
      leading: &leading,
      trailing: &trailing,
    }),
  };
  super::with_ast_view_for_script(info, |script| {
    run_test(script);
  });
}

pub fn get_swc_module(
  file_path: &Path,
  file_text: &str,
) -> (
  Module,
  Vec<TokenAndRange>,
  SourceTextInfo,
  SingleThreadedComments,
) {
  // lifted from dprint-plugin-typescript
  let handler = Handler::with_emitter(false, false, Box::new(EmptyEmitter {}));
  let source_text_info = SourceTextInfo::from_string(file_text.to_string());

  let comments: SingleThreadedComments = Default::default();
  return {
    let ts_config = crate::swc::parser::TsConfig {
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
    let tokens = parser
      .input()
      .take()
      .into_iter()
      .map(|t| t.into())
      .collect();

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

pub fn get_swc_script(
  file_path: &Path,
  file_text: &str,
) -> (
  Script,
  Vec<TokenAndRange>,
  SourceTextInfo,
  SingleThreadedComments,
) {
  // lifted from dprint-plugin-typescript
  let handler = Handler::with_emitter(false, false, Box::new(EmptyEmitter {}));
  let source_text_info = SourceTextInfo::from_string(file_text.to_string());

  let comments: SingleThreadedComments = Default::default();
  return {
    let ts_config = crate::swc::parser::TsConfig {
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
    let tokens = parser
      .input()
      .take()
      .into_iter()
      .map(|t| t.into())
      .collect();

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

#[cfg(feature = "serialize")]
pub fn run_serialize_test(file_text: &str, expected_json_path: impl AsRef<Path>) {
  let should_update = std::env::var("UPDATE").is_ok();
  run_test_with_module(Path::new("test.ts"), file_text, |module| {
    // check AST
    {
      let mut formatter = serde_json::ser::PrettyFormatter::new();
      let mut buffer = Vec::new();
      serialize_module(&mut buffer, &mut formatter, file_text, module.inner).unwrap();
      if should_update {
        std::fs::write(&expected_json_path, &buffer).unwrap();
      }
      let expected = std::fs::read_to_string(expected_json_path.as_ref()).unwrap();
      pretty_assertions::assert_eq!(String::from_utf8(buffer).unwrap(), expected.trim());
    }

    // check tokens
    {
      let mut expected_json_path = PathBuf::from(expected_json_path.as_ref());
      expected_json_path.set_extension("tokens.json");
      let mut formatter = serde_json::ser::PrettyFormatter::new();
      let mut buffer = Vec::new();
      serialize_token_and_spans(
        &mut buffer,
        &mut formatter,
        file_text,
        module.tokens.unwrap().tokens,
      )
      .unwrap();
      if should_update {
        std::fs::write(&expected_json_path, &buffer).unwrap();
      }
      let expected = std::fs::read_to_string(&expected_json_path).unwrap();
      pretty_assertions::assert_eq!(String::from_utf8(buffer).unwrap(), expected.trim());
    }

    // check comments
    {
      let mut expected_json_path = PathBuf::from(expected_json_path.as_ref());
      expected_json_path.set_extension("comments.json");
      let mut formatter = serde_json::ser::PrettyFormatter::new();
      let mut buffer = Vec::new();
      let comments_container = module.comments.unwrap();
      serialize_comments(
        &mut buffer,
        &mut formatter,
        file_text,
        comments_container.leading,
        comments_container.trailing,
      )
      .unwrap();
      if should_update {
        std::fs::write(&expected_json_path, &buffer).unwrap();
      }
      let expected = std::fs::read_to_string(&expected_json_path).unwrap();
      pretty_assertions::assert_eq!(String::from_utf8(buffer).unwrap(), expected.trim());
    }
  });
}

fn should_parse_as_jsx(file_path: &Path) -> bool {
  if let Some(extension) = get_lowercase_extension(file_path) {
    return extension == "tsx" || extension == "jsx" || extension == "js" || extension == "mjs";
  }
  true
}

fn get_lowercase_extension(file_path: &Path) -> Option<String> {
  file_path
    .extension()
    .and_then(|e| e.to_str())
    .map(|f| f.to_lowercase())
}

pub struct EmptyEmitter {}

impl Emitter for EmptyEmitter {
  fn emit(&mut self, _: &DiagnosticBuilder<'_>) {
    // for now, we don't care about diagnostics so do nothing
  }

  fn should_show_explain(&self) -> bool {
    false
  }
}
