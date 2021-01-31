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
