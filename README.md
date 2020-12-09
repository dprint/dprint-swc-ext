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
- `.lo() -> BytePos`
- `.hi() -> BytePos`
- `.lo_line() -> usize`
- `.lo_line_display() -> usize` -- 1-indexed line number
- `.lo_line_fast(&source_file) -> usize`
- `.hi_line() -> usize`
- `.hi_line_display() -> usize` -- 1-indexed line number
- `.hi_line_fast(&source_file) -> usize`
- `.tokens() -> &[TokenAndSpan]` - All the descendant tokens within the span of the node.
- `.tokens_fast(&token_container) -> &[TokenAndSpan]`
- `.children_with_tokens() -> Vec<NodeOrToken<'a>>` - Gets the children with the tokens found between the children
- `.children_with_tokens_fast(&token_container) -> Vec<NodeOrToken<'a>>`
- `.module() -> &'a Module` - Gets the root node.
- `.source_file() -> &'a swc_common::SourceFile` - Gets the swc source file
- `.token_container() -> &'a TokenContainer` - Gets the token container that was passed into the view.
- `.kind() -> NodeKind` - Gets the "node kind" enum variant associated with the node (ex. `NodeKind::ClassDecl`).

Node/enum node specific helpers:

- `.to::<NodeType>() -> &NodeType`
- `.try_to::<NodeType>() -> Option<&NodeType>` -- Copied design from rslint, but not sure about this naming because "try" usually means it returns a `Result<T>`

## TODO

- `.lo_column()`, `.hi_column()`, `.leading_comments()`, `.trailing_comments()`
- Right now this only works if analyzing one file at a time. It would be good to improve the API to accept a large
  collection of source files (should be easy).

## Example

Given the following parsed input code:

<!-- dprint-ignore -->
```ts
class MyClass { prop: string; myMethod() {}}
```

Code can be written like so:

```rust
// setup swc (parse an AST and optionally get the comments and tokens)
let source_file: swc_common::SourceFile = ...;
let module: swc_ecmascript::ast::Module = ...;
let comments: swc_common::comments::SingleThreadedComments = ...;
let tokens: Vec<swc_ecmascript::parser::token::TokenAndSpan> = ...;

// setup for creating a view
let comment_container = CommentContainer::new(&comments);
let token_container = TokenContainer::new(&tokens);
let source_file_info = SourceFileInfo {
  module: &module,
  // optionally provide the SourceFile for using text related methods
  source_file: Some(&source_file),
  // optionally provide the comments for comment related methods (doesn't do anything yet...)
  comments: Some(&comment_container)
  // optionally provide the tokens for token related methods
  tokens: Some(&token_container),
}

// now create and use the view
dprint_swc_ecma_ast_view::with_ast_view(source_file_info, |ast_view| {
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
