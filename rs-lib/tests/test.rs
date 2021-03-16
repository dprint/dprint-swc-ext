extern crate dprint_swc_ecma_ast_view;
use dprint_swc_ecma_ast_view::*;

mod helpers;
use helpers::*;

#[test]
fn it_should_get_children() {
  run_test("class Test { a: string; b: number; }", |program| {
    let class_decl = program.children()[0].expect::<ClassDecl>();
    let children = class_decl.class.children();
    assert_eq!(children.len(), 2);
    assert_eq!(children[0].text(), "a: string;");
    assert_eq!(children[1].text(), "b: number;");
  });
}

#[cfg(feature = "serialize")]
#[test]
fn it_shoule_be_serialized_to_json() {
  let tests = [
    ("let foo = 42;", "./tests/expected/serialize_var_decl.json"),
    (
      "function foo({ a }: { a: number }): boolean { return a % 2 === 0; }",
      "./tests/expected/serialize_ts_function.json",
    ),
  ];

  for (code, expected_path) in tests.iter() {
    run_serialize_test(code, expected_path);
  }
}
