extern crate dprint_swc_ast_ref;
use std::path::{Path, PathBuf};
use swc_common::{
    errors::{Handler, Emitter, DiagnosticBuilder},
    FileName, comments::{Comment, SingleThreadedComments, SingleThreadedCommentsMap}, SourceFile, BytePos, Spanned
};
use swc_ecma_ast::{Module};
use swc_ecma_parser::{Parser, StringInput, Syntax, lexer::Lexer, Capturing, JscTarget, token::{TokenAndSpan}};
use dprint_swc_ast_ref::{NodeTrait, ModuleItem, Stmt, Decl};

#[test]
fn test_creating_reference() {
    let module = get_swc_ast(&PathBuf::from("file.ts"), "class Test { test: string; other() {}}");
    let ref_view = dprint_swc_ast_ref::get_reference_view(&module);
    match &ref_view.body[0] {
        ModuleItem::Stmt(Stmt::Decl(Decl::Class(decl))) => {
            println!("{:?}", decl.span().lo);
            println!("{:?}", decl.class.body[0].parent().span().lo);

            for child in decl.class.children() {
                println!("{:?}", child.span().lo);
            }
        }
        _ => {},
    }
    println!("SUCCESS");
}

fn get_swc_ast(file_path: &Path, file_text: &str) -> Module {
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
    let (module, tokens) = {
        let mut ts_config: swc_ecma_parser::TsConfig = Default::default();
        ts_config.tsx = should_parse_as_jsx(file_path);
        ts_config.dynamic_import = true;
        ts_config.decorators = true;
        let lexer = Lexer::new(
            Syntax::Typescript(ts_config),
            JscTarget::Es2019,
            StringInput::from(&source_file),
            Some(&comments)
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
            },
            Ok(module) => Ok((module, tokens))
        }
    }.unwrap();

    return module;

    fn should_parse_as_jsx(file_path: &Path) -> bool {
        if let Some(extension) = get_lowercase_extension(file_path) {
            return extension == "tsx" || extension == "jsx" || extension == "js" || extension == "mjs";
        }
        return true;
    }

    fn get_lowercase_extension(file_path: &Path) -> Option<String> {
        file_path.extension().and_then(|e| e.to_str()).map(|f| f.to_lowercase())
    }

    pub struct EmptyEmitter {
    }

    impl Emitter for EmptyEmitter {
        fn emit(&mut self, _: &DiagnosticBuilder<'_>) {
            // for now, we don't care about diagnostics so do nothing
        }

        fn should_show_explain(&self) -> bool {
            false
        }
    }
}