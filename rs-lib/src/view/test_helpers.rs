use std::path::Path;

use crate::test_helpers::get_swc_module;
use crate::test_helpers::get_swc_script;

pub fn run_test(file_text: &str, run_test: impl Fn(super::Program<'_>)) {
  let file_path = Path::new("test.ts");
  run_test_with_module(file_path, file_text, |module| run_test(super::Program::Module(module)));
  run_test_with_script(file_path, file_text, |script| run_test(super::Program::Script(script)));
}

pub fn run_test_with_module<'a>(file_path: &Path, file_text: &str, run_test: impl Fn(&super::Module<'_>)) {
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

pub fn run_test_with_script(file_path: &Path, file_text: &str, run_test: impl Fn(&super::Script<'_>)) {
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
