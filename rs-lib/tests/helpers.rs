use std::path::Path;
use swc_common::{
  comments::SingleThreadedComments,
  errors::{DiagnosticBuilder, Emitter, Handler},
  BytePos, FileName, SourceFile,
};
use swc_ecmascript::ast::{Module, Script};
use swc_ecmascript::parser::{
  lexer::Lexer, token::TokenAndSpan, Capturing, JscTarget, Parser, StringInput, Syntax,
};

#[cfg(feature = "serialize")]
use {dprint_swc_ecma_ast_view::*, std::path::PathBuf};

pub fn run_test(file_text: &str, run_test: impl Fn(dprint_swc_ecma_ast_view::Program)) {
  let file_path = Path::new("test.ts");
  run_test_with_module(file_path, file_text, |module| {
    run_test(dprint_swc_ecma_ast_view::Program::Module(module))
  });
  run_test_with_script(file_path, file_text, |script| {
    run_test(dprint_swc_ecma_ast_view::Program::Script(script))
  });
}

pub fn run_test_with_module<'a>(
  file_path: &Path,
  file_text: &str,
  run_test: impl Fn(&'a dprint_swc_ecma_ast_view::Module<'a>),
) {
  let (module, tokens, source_file, comments) = get_swc_module(file_path, file_text);
  let info = dprint_swc_ecma_ast_view::ModuleInfo {
    module: &module,
    source_file: Some(&source_file),
    tokens: Some(&tokens),
    comments: Some(&comments),
  };
  dprint_swc_ecma_ast_view::with_ast_view_for_module(info, |module| {
    run_test(module);
  });
}

pub fn run_test_with_script<'a>(
  file_path: &Path,
  file_text: &str,
  run_test: impl Fn(&'a dprint_swc_ecma_ast_view::Script<'a>),
) {
  let (script, tokens, source_file, comments) = get_swc_script(file_path, file_text);
  let info = dprint_swc_ecma_ast_view::ScriptInfo {
    script: &script,
    source_file: Some(&source_file),
    tokens: Some(&tokens),
    comments: Some(&comments),
  };
  dprint_swc_ecma_ast_view::with_ast_view_for_script(info, |script| {
    run_test(script);
  });
}

pub fn get_swc_module(
  file_path: &Path,
  file_text: &str,
) -> (
  Module,
  Vec<TokenAndSpan>,
  SourceFile,
  SingleThreadedComments,
) {
  // lifted from dprint-plugin-typescript
  let handler = Handler::with_emitter(false, false, Box::new(EmptyEmitter {}));
  let source_file = SourceFile::new(
    FileName::Custom(file_path.to_string_lossy().into()),
    false,
    FileName::Custom(file_path.to_string_lossy().into()),
    file_text.into(),
    BytePos(0),
  );

  let comments: SingleThreadedComments = Default::default();
  return {
    let ts_config = swc_ecmascript::parser::TsConfig {
      tsx: should_parse_as_jsx(file_path),
      dynamic_import: true,
      decorators: true,
      ..Default::default()
    };
    let lexer = Lexer::new(
      Syntax::Typescript(ts_config),
      JscTarget::Es2019,
      StringInput::from(&source_file),
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
      Ok(module) => Ok((module, tokens, source_file, comments)),
    }
  }
  .unwrap();
}

pub fn get_swc_script(
  file_path: &Path,
  file_text: &str,
) -> (
  Script,
  Vec<TokenAndSpan>,
  SourceFile,
  SingleThreadedComments,
) {
  // lifted from dprint-plugin-typescript
  let handler = Handler::with_emitter(false, false, Box::new(EmptyEmitter {}));
  let source_file = SourceFile::new(
    FileName::Custom(file_path.to_string_lossy().into()),
    false,
    FileName::Custom(file_path.to_string_lossy().into()),
    file_text.into(),
    BytePos(0),
  );

  let comments: SingleThreadedComments = Default::default();
  return {
    let ts_config = swc_ecmascript::parser::TsConfig {
      tsx: should_parse_as_jsx(file_path),
      dynamic_import: true,
      decorators: true,
      ..Default::default()
    };
    let lexer = Lexer::new(
      Syntax::Typescript(ts_config),
      JscTarget::Es2019,
      StringInput::from(&source_file),
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
      Ok(script) => Ok((script, tokens, source_file, comments)),
    }
  }
  .unwrap();
}

#[cfg(feature = "serialize")]
pub fn run_serialize_test(file_text: &str, expected_json_path: impl AsRef<Path>) {
  run_test_with_module(&Path::new("test.ts"), file_text, |module| {
    // check AST
    {
      let mut formatter = serde_json::ser::PrettyFormatter::new();
      let mut buffer = Vec::new();
      serialize_module(&mut buffer, &mut formatter, file_text, &module.inner).unwrap();
      // std::fs::write(&expected_json_path, &buffer).unwrap();
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
      // std::fs::write(&expected_json_path, &buffer).unwrap();
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
        &comments_container.leading,
        &comments_container.trailing,
      )
      .unwrap();
      // std::fs::write(&expected_json_path, &buffer).unwrap();
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
