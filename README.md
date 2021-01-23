# dprint-swc-ecma-ast-view

[![](https://img.shields.io/crates/v/dprint-swc-ecma-ast-view.svg)](https://crates.io/crates/dprint-swc-ecma-ast-view)

The library at `./rs-lib` is code generated from [swc_ecma_ast](https://crates.io/crates/swc_ecma_ast) via the code in `./generation` to produce a more easily navigable immutable AST.

## What does this do?

1. Creates a wrapper AST around [swc](https://github.com/swc-project/swc)'s AST that stores the node parents.
2. Adds a `Node` enum type to allow referencing any kind of node.
3. Adds many helper methods.

## Helpers

Spanned (All):

- `.lo() -> BytePos`
- `.hi() -> BytePos`
- `.text_fast(module: &Module<'a>) -> &'a str` -- Doesn't require going up the tree to the root node
- `.start_line_fast(module: &Module) -> usize`
- `.end_line_fast(module: &Module) -> usize`
- `.start_column_fast(module: &Module) -> usize`
- `.end_column_fast(module: &Module) -> usize`
- `.width_fast(module: &Module) -> usize`
- `.tokens_fast(module: &Module<'a>) -> &'a [TokenAndSpan]`
- `.leading_comments_fast(module: &Module<'a>) -> CommentsIterator<'a>`
- `.trailing_comments_fast(module: &Module<'a>) -> CommentsIterator<'a>`
- `.previous_token_fast(module: &Module) -> Option<&TokenAndSpan>`
- `.next_token_fast(module: &Module) -> Option<&TokenAndSpan>`
- `.previous_tokens_fast(module: &Module) -> &'a [TokenAndSpan]`
- `.next_tokens_fast(module: &Module) -> &'a [TokenAndSpan]`

Node/Enum Node/Nodes:

- `.module() -> &'a Module` - Gets the root node.
- `.parent() -> Option<Node<'a>>`
- `.children() -> Vec<Node<'a>>`
- `.child_index() -> usize`
- `.ancestors() -> AncestorsIterator<'a>`
- `.previous_sibling() -> Option<Node<'a>>`
- `.next_sibling() -> Option<Node<'a>>`
- `.previous_siblings() -> Vec<Node<'a>>`
- `.next_siblings() -> Vec<Node<'a>>`
- `.text() -> &str` - Slightly slower than `.text_fast(module)` because it requires going up the tree to get the root node
- `.start_line() -> usize`
- `.end_line() -> usize`
- `.start_column() -> usize`
- `.end_column() -> usize`
- `.width() -> usize`
- `.tokens() -> &[TokenAndSpan]` - All the descendant tokens within the span of the node.
- `.children_with_tokens() -> Vec<NodeOrToken<'a>>` - Gets the children with the tokens found between the children
- `.children_with_tokens_fast(module: &Module<'a>) -> Vec<NodeOrToken<'a>>`
- `.leading_comments() -> CommentsIterator<'a>`
- `.trailing_comments() -> CommentsIterator<'a>`
- `.kind() -> NodeKind` - Gets the "node kind" enum variant associated with the node (ex. `NodeKind::ClassDecl`).
- `.previous_token() -> Option<&TokenAndSpan>`
- `.next_token() -> Option<&TokenAndSpan>`
- `.previous_tokens() -> &'a [TokenAndSpan]`
- `.next_tokens() -> &'a [TokenAndSpan]`

Node/Enum Node:

- `.to::<NodeType>() -> Option<&NodeType>`
- `.expect::<NodeType>() -> &NodeType`
- `.is::<NodeType>() -> bool`

TokenAndSpan extensions:

- `.token_index(module: &Module) -> usize` - Gets the token index of the specified module.

Module:

- `token_at_index(index: &usize) - Option<&TokenAndSpan>`

## TODO

- Right now this only works if analyzing one file at a time. It would be good to improve the API to accept a large
  collection of source files (should be easy).
- Unit tests

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
let source_file_info = SourceFileInfo {
  module: &module,
  // optionally provide the swc_common::SourceFile for using text related methods
  source_file: Some(&source_file),
  // optionally provide the comments for comment related methods
  comments: Some(&comments)
  // optionally provide the tokens for token related methods
  tokens: Some(&tokens),
}

// now create and use the view
dprint_swc_ecma_ast_view::with_ast_view(source_file_info, |module| {
  let class = module.body[0].expect::<ClassDecl>().class;
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
