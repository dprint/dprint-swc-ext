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

pub fn run_test(file_text: &str, run_test: impl Fn(dprint_swc_ecma_ast_view::Program)) {
  let file_path = Path::new("test.ts");
  run_test_with_module(file_path, file_text, |module| {
    run_test(dprint_swc_ecma_ast_view::Program::Module(module))
  });
  run_test_with_script(file_path, file_text, |script| {
    run_test(dprint_swc_ecma_ast_view::Program::Script(script))
  });
}

pub fn run_test_with_module(
  file_path: &Path,
  file_text: &str,
  run_test: impl Fn(&dprint_swc_ecma_ast_view::Module),
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

pub fn run_test_with_script(
  file_path: &Path,
  file_text: &str,
  run_test: impl Fn(&dprint_swc_ecma_ast_view::Script),
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
    let mut ts_config: swc_ecmascript::parser::TsConfig = Default::default();
    ts_config.tsx = should_parse_as_jsx(file_path);
    ts_config.dynamic_import = true;
    ts_config.decorators = true;
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
    let mut ts_config: swc_ecmascript::parser::TsConfig = Default::default();
    ts_config.tsx = should_parse_as_jsx(file_path);
    ts_config.dynamic_import = true;
    ts_config.decorators = true;
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
  let file_path = Path::new("test.ts");
  run_test_with_module(&file_path, file_text, |module| {
    let result = serde_json::to_string_pretty(&module).unwrap();
    let expected = std::fs::read_to_string(expected_json_path.as_ref()).unwrap();
    pretty_assertions::assert_eq!(result, expected.trim());
  });
}

fn should_parse_as_jsx(file_path: &Path) -> bool {
  if let Some(extension) = get_lowercase_extension(file_path) {
    return extension == "tsx" || extension == "jsx" || extension == "js" || extension == "mjs";
  }
  return true;
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
