extern crate dprint_swc_ecma_ast_view;
use dprint_swc_ecma_ast_view::{
  CastableNode, ClassDecl, Decl, ModuleItem, Node, NodeTrait, SourceFileInfo, TokenContainer,
};
use std::path::{Path, PathBuf};
use swc_common::{
  comments::{Comment, SingleThreadedComments, SingleThreadedCommentsMap},
  errors::{DiagnosticBuilder, Emitter, Handler},
  BytePos, FileName, SourceFile, Spanned,
};
use swc_ecmascript::ast::Module;
use swc_ecmascript::parser::{
  lexer::Lexer, token::TokenAndSpan, Capturing, JscTarget, Parser, StringInput, Syntax,
};

#[test]
fn test_creating_reference() {
  let file_text = "class MyClass { prop: string; myMethod() {}}";
  let (module, tokens) = get_swc_ast(&PathBuf::from("file.ts"), file_text);
  let token_container = TokenContainer::new(&tokens);
  let info = SourceFileInfo {
    module: &module,
    file_text: Some(file_text),
    tokens: Some(&token_container),
  };
  dprint_swc_ecma_ast_view::with_ast_view(info, |ast_view| {
    println!("Test {:?}", ast_view.text());
    println!("Test 2 {:?}", ast_view.body[0].text());
    println!("Test 3 {:?}", ast_view.body[0].children()[0].text());
    let class = ast_view.body[0].to::<ClassDecl>().class;
    println!("{:?}", class.text());

    for child in class.children() {
      println!("---------");
      println!("Child: {:?}", child.text());
      println!("Parent: {:?}", child.parent().unwrap().text());
      if let Some(prev_sibling) = child.prev_sibling() {
        println!("Previous sibling: {:?}", prev_sibling.text());
      }
      if let Some(next_sibling) = child.next_sibling() {
        println!("Next sibling: {:?}", next_sibling.text());
      }
      for token in child.tokens() {
        println!("Token: {:?}", token);
      }
    }
  });
  println!("SUCCESS");
}

fn get_swc_ast(file_path: &Path, file_text: &str) -> (Module, Vec<TokenAndSpan>) {
  // lifted from dprint-plugin-typescript
  let handler = Handler::with_emitter(false, false, Box::new(EmptyEmitter {}));
  let file_bytes = file_text.as_bytes();
  let source_file = SourceFile::new(
    FileName::Custom(file_path.to_string_lossy().into()),
    false,
    FileName::Custom(file_path.to_string_lossy().into()),
    file_text.into(),
    BytePos(0),
  );

  let comments: SingleThreadedComments = Default::default();
  return {
    let mut ts_config: swc_ecma_parser::TsConfig = Default::default();
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
      Ok(module) => Ok((module, tokens)),
    }
  }
  .unwrap();

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
}
