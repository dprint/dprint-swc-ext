# dprint-swc-ecma-ast-view

Proof of concept.

The library at `./rs-lib` is code generated from [swc_ecma_ast](https://crates.io/crates/swc_ecma_ast) via the code in `./generation` to produce a more easily navigable immutable AST.

## What does this do?

Creates a wrapper AST around [swc](https://github.com/swc-project/swc)'s AST that keeps track of the node parents and adds a `Node` enum type to allow referencing any kind of node.

## Helpers

All:

- `.parent() -> Option<Node<'a>>`
- `.children() -> Vec<Node<'a>>`
- `.child_index() -> usize`
- `.prev_sibling() -> Option<Node<'a>>`
- `.next_sibling() -> Option<Node<'a>>`
- `.text() -> &str`
- `.text_fast(file_text: &str) -> &str` -- Doesn't require going up the tree to the root node

Node/enum node specific helpers:

- `.to::<NodeType>() -> &NodeType`
- `.try_to::<NodeType>() -> Option<&NodeType>` -- Copied design from rslint, but not sure about this naming because "try" usually means it returns a `Result<T>`

## TODO

- `.children_with_tokens() -> Vec<NodeOrToken<'a>>` - Gets the children with the tokens found between the children
- `.tokens() -> Vec<Token<'a>>` - All the descendant tokens within the span of the node.
- Methods for getting comments

## Example

Given the following parsed input code:

<!-- dprint-ignore -->
```ts
class MyClass { prop: string; myMethod() {}}
```

Code can be written like so:

```rust
let file_text: String = ...;
let module: swc_ecma_ast::Module = ...;
let source_file_info = SourceFileInfo {
  module: &module,
  // optionally provide the file text for using the `.text()` method
  file_text: Some(&file_text),
  // optionally provide the tokens for certain methods... not yet implemented
  tokens: None,
}

dprint_swc_ecma_ast_view::with_ast_view(source_file_info, |ast_view| {
  let class = ast_view.body[0].to::<ClassDecl>().class;
  println!("{:?}", class.text(file_text));

  for child in class.children() {
    println!("---------");
    println!("Child: {:?}", child.text(file_text));
    println!("Parent: {:?}", child.parent().unwrap().text(file_text));
    if let Some(prev_sibling) = child.prev_sibling() {
      println!("Previous sibling: {:?}", prev_sibling.text(file_text));
    }
    if let Some(next_sibling) = child.next_sibling() {
      println!("Next sibling: {:?}", next_sibling.text(file_text));
    }
  }
});
```

Outputs:

```
"class MyClass { prop: string; myMethod() {}}"
---------
Child: "prop: string;"
Parent: "class MyClass { prop: string; myMethod() {}}"
Next sibling: "myMethod() {}"
---------
Child: "myMethod() {}"
Parent: "class MyClass { prop: string; myMethod() {}}"
Previous sibling: "prop: string;"
```
