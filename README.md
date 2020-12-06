# dprint-swc-ecma-ast-view

Proof of concept.

The library at `./rs-lib` is code generated from [swc_ecma_ast](https://crates.io/crates/swc_ecma_ast) via the code in `./generation` to produce a more easily navigable immutable AST.

**Currently very unsafe and some craziness going on.**

## What does this do?

Creates a wrapper AST around [swc](https://github.com/swc-project/swc)'s AST that keeps track of the node parents and adds a `Node` enum type to allow referencing any kind of node.

## Helpers

All:

- `.parent() -> Option<Node>`
- `.children() -> Vec<Node>`
- `.child_index() -> usize`
- `.prev_sibling() -> Option<Node>`
- `.next_sibling() -> Option<Node>`
- `.text(file_text: &str) -> &str` -- Might consider removing the need for an argument here.

Node/enum node specific helpers:

- `.to::<NodeType>() -> &NodeType`
- `.try_to::<NodeType>() -> Option<&NodeType>` -- Copied design from rslint, but not sure about this naming because "try" usually means it returns a `Result<T>`

## TODO

- `.children_with_tokens() -> Vec<NodeOrToken>` - Gets the children with the tokens found between the children
- `.tokens() -> Vec<Token>` - All the descendant tokens within the span of the node.
- I think maybe change the code to use an "arena" (ex. https://crates.io/crates/bumpalo)

## Example

Given the following parsed input code:

<!-- dprint-ignore -->
```ts
class MyClass { prop: string; myMethod() {}}
```

Code can be written like so:

```rs
let file_text: String = ...;
let module: swc_ecma_ast::Module = ...;
let ast_view = dprint_swc_ecma_ast_view::get_ast_view(&module);

// VERY IMPORTANT: The `module` and `ast_view` objects must not be dropped
// for the duration of the scope below and the objects in the scope below
// must not be used outside the scope... otherwise bad things will happen :D
{
  let class = &ast_view.body[0].to::<ClassDecl>().class;
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
}
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
