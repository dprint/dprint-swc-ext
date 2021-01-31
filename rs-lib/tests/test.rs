extern crate dprint_swc_ecma_ast_view;
use dprint_swc_ecma_ast_view::{
  CastableNode, ClassDecl, Decl, ModuleInfo, ModuleItem, Node, NodeOrToken, NodeTrait, VarDecl,
};
use std::collections::HashMap;
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
  let file_text = "// 1\n// 2\nclass MyClass { prop: string; myMethod() {}}";
  let (module, tokens, source_file, comments) = get_swc_ast(&PathBuf::from("file.ts"), file_text);
  let info = ModuleInfo {
    module: &module,
    source_file: Some(&source_file),
    tokens: Some(&tokens),
    comments: Some(&comments),
  };
  dprint_swc_ecma_ast_view::with_ast_view_for_module(info, |module| {
    println!("Test {:?}", module.text());
    println!("Test 2 {:?}", module.body[0].text());
    let leading_comments = module.body[0].leading_comments();
    for comment in leading_comments.clone().rev() {
      println!("Comment {:?}", comment);
    }
    println!(
      "Leading comments {:?}",
      leading_comments.collect::<Vec<_>>()
    );
    println!("Test 3 {:?}", module.body[0].children()[0].text());
    let class = module.body[0].expect::<ClassDecl>().class;
    println!("{:?}", class.text());

    for child in class.children() {
      println!("---------");
      println!("Child: {:?}", child.text());
      println!("Lo column: {:?}", child.start_column());
      println!("Hi column: {:?}", child.end_column());
      println!("Parent: {:?}", child.parent().unwrap().text());
      if let Some(previous_sibling) = child.previous_sibling() {
        println!("Previous sibling: {:?}", previous_sibling.text());
      }
      if let Some(next_sibling) = child.next_sibling() {
        println!("Next sibling: {:?}", next_sibling.text());
      }
      for token in child.tokens() {
        println!("Token: {:?}", token);
      }
      for token_or_child in child.children_with_tokens() {
        match token_or_child {
          NodeOrToken::Token(token) => println!("TokenOrChild: {:?}", token),
          NodeOrToken::Node(node) => println!("TokenOrChild: {:?}", node.text()),
        }
      }
    }
  });
  println!("SUCCESS");
}

fn get_swc_ast(
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
