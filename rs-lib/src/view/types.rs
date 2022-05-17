use super::generated::*;
use crate::common::*;
use crate::swc::ast as swc_ast;
use crate::swc::common::comments::SingleThreadedCommentsMapInner;

pub enum NodeOrToken<'a> {
  Node(Node<'a>),
  Token(&'a TokenAndRange),
}

impl<'a> NodeOrToken<'a> {
  pub fn unwrap_token(&self) -> &'a TokenAndRange {
    match self {
      NodeOrToken::Token(token) => token,
      NodeOrToken::Node(node) => panic!(
        "Expected to unwrap a token, but it was a node of kind {}.",
        node.kind()
      ),
    }
  }

  pub fn unwrap_node(&self) -> &Node<'a> {
    match self {
      NodeOrToken::Node(node) => node,
      NodeOrToken::Token(token) => panic!(
        "Expected to unwrap a node, but it was a token with text '{:?}'.",
        token.token
      ),
    }
  }
}

impl<'a> SourceRanged for NodeOrToken<'a> {
  fn start(&self) -> SourcePos {
    match self {
      NodeOrToken::Node(node) => node.start(),
      NodeOrToken::Token(token) => token.start(),
    }
  }

  fn end(&self) -> SourcePos {
    match self {
      NodeOrToken::Node(node) => node.end(),
      NodeOrToken::Token(token) => token.end(),
    }
  }
}

macro_rules! implement_root_node {
  ($name:ty) => {
    impl<'a> RootNode<'a> for $name {
      fn maybe_text_info(&self) -> Option<&'a SourceTextInfo> {
        self.text_info
      }

      fn maybe_token_container(&self) -> Option<&'a TokenContainer<'a>> {
        self.tokens
      }

      fn maybe_comment_container(&self) -> Option<&'a CommentContainer<'a>> {
        self.comments
      }
    }
  };
}

implement_root_node!(Module<'a>);
implement_root_node!(&Module<'a>);
implement_root_node!(Script<'a>);
implement_root_node!(&Script<'a>);

/// A Module or Script node.
#[derive(Clone, Copy)]
pub enum Program<'a> {
  Module(&'a Module<'a>),
  Script(&'a Script<'a>),
}

impl<'a> SourceRanged for Program<'a> {
  fn start(&self) -> SourcePos {
    match self {
      Program::Module(node) => node.start(),
      Program::Script(node) => node.start(),
    }
  }

  fn end(&self) -> SourcePos {
    match self {
      Program::Module(node) => node.end(),
      Program::Script(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for Program<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    None
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      Program::Module(node) => node.children(),
      Program::Script(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      Program::Module(node) => node.as_node(),
      Program::Script(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      Program::Module(node) => node.kind(),
      Program::Script(node) => node.kind(),
    }
  }
}

impl<'a> From<&Program<'a>> for Node<'a> {
  fn from(node: &Program<'a>) -> Node<'a> {
    match node {
      Program::Module(node) => (*node).into(),
      Program::Script(node) => (*node).into(),
    }
  }
}

impl<'a> From<Program<'a>> for Node<'a> {
  fn from(node: Program<'a>) -> Node<'a> {
    match node {
      Program::Module(node) => node.into(),
      Program::Script(node) => node.into(),
    }
  }
}

impl<'a> RootNode<'a> for Program<'a> {
  fn maybe_text_info(&self) -> Option<&'a SourceTextInfo> {
    match self {
      Program::Module(module) => module.text_info,
      Program::Script(script) => script.text_info,
    }
  }

  fn maybe_token_container(&self) -> Option<&'a TokenContainer<'a>> {
    match self {
      Program::Module(module) => module.tokens,
      Program::Script(script) => script.tokens,
    }
  }

  fn maybe_comment_container(&self) -> Option<&'a CommentContainer<'a>> {
    match self {
      Program::Module(module) => module.comments,
      Program::Script(script) => script.comments,
    }
  }
}

pub trait NodeTrait<'a>: SourceRanged + Sized {
  fn parent(&self) -> Option<Node<'a>>;
  fn children(&self) -> Vec<Node<'a>>;
  fn as_node(&self) -> Node<'a>;
  fn kind(&self) -> NodeKind;

  fn ancestors(&self) -> AncestorIterator<'a> {
    AncestorIterator::new(self.as_node())
  }

  fn start_line(&self) -> usize {
    self.start_line_fast(&self.program())
  }

  fn end_line(&self) -> usize {
    self.end_line_fast(&self.program())
  }

  fn start_column(&self) -> usize {
    self.start_column_fast(&self.program())
  }

  fn end_column(&self) -> usize {
    self.end_column_fast(&self.program())
  }

  fn char_width(&self) -> usize {
    self.char_width_fast(&self.program())
  }

  fn child_index(&self) -> usize {
    if let Some(parent) = self.parent() {
      let start_pos = self.start();
      for (i, child) in parent.children().iter().enumerate() {
        if child.start() == start_pos {
          return i;
        }
      }
      panic!("Could not find the child index for some reason.");
    } else {
      0
    }
  }

  fn previous_sibling(&self) -> Option<Node<'a>> {
    if let Some(parent) = self.parent() {
      let child_index = self.child_index();
      if child_index > 0 {
        Some(parent.children().remove(child_index - 1))
      } else {
        None
      }
    } else {
      None
    }
  }

  /// Gets the previous siblings in the order they appear in the file.
  fn previous_siblings(&self) -> Vec<Node<'a>> {
    if let Some(parent) = self.parent() {
      let child_index = self.child_index();
      if child_index > 0 {
        let mut parent_children = parent.children();
        parent_children.drain(child_index..);
        parent_children
      } else {
        Vec::new()
      }
    } else {
      Vec::new()
    }
  }

  /// Gets the next siblings in the order they appear in the file.
  fn next_sibling(&self) -> Option<Node<'a>> {
    if let Some(parent) = self.parent() {
      let next_index = self.child_index() + 1;
      let mut parent_children = parent.children();
      if next_index < parent_children.len() {
        Some(parent_children.remove(next_index))
      } else {
        None
      }
    } else {
      None
    }
  }

  fn next_siblings(&self) -> Vec<Node<'a>> {
    if let Some(parent) = self.parent() {
      let next_index = self.child_index() + 1;
      let mut parent_children = parent.children();
      if next_index < parent_children.len() {
        parent_children.drain(0..next_index);
        parent_children
      } else {
        Vec::new()
      }
    } else {
      Vec::new()
    }
  }

  fn tokens(&self) -> &'a [TokenAndRange] {
    self.tokens_fast(&self.program())
  }

  fn children_with_tokens(&self) -> Vec<NodeOrToken<'a>> {
    self.children_with_tokens_fast(&self.program())
  }

  fn children_with_tokens_fast(&self, program: &dyn RootNode<'a>) -> Vec<NodeOrToken<'a>> {
    let children = self.children();
    let tokens = self.tokens_fast(program);
    let mut result = Vec::new();
    let mut tokens_index = 0;

    for child in children {
      let child_range = child.range();

      // get the tokens before the current child
      for token in &tokens[tokens_index..] {
        if token.start() < child_range.start {
          result.push(NodeOrToken::Token(token));
          tokens_index += 1;
        } else {
          break;
        }
      }

      // push current child
      result.push(NodeOrToken::Node(child));

      // skip past all the tokens within the token
      for token in &tokens[tokens_index..] {
        if token.range.end <= child_range.end {
          tokens_index += 1;
        } else {
          break;
        }
      }
    }

    // get the tokens after the children
    for token in &tokens[tokens_index..] {
      result.push(NodeOrToken::Token(token));
    }

    result
  }

  fn leading_comments(&self) -> CommentsIterator<'a> {
    self.leading_comments_fast(&self.program())
  }

  fn trailing_comments(&self) -> CommentsIterator<'a> {
    self.trailing_comments_fast(&self.program())
  }

  /// Gets the root node.
  fn program(&self) -> Program<'a> {
    let mut current: Node<'a> = self.as_node();
    while let Some(parent) = current.parent() {
      current = parent;
    }

    // the top-most node will always be a script or module
    match current {
      Node::Module(module) => Program::Module(module),
      Node::Script(script) => Program::Script(script),
      _ => panic!(
        "Expected the root node to be a Module or Script, but it was a {}.",
        current.kind()
      ),
    }
  }

  /// Gets the root node if the view was created from a Module; otherwise panics.
  fn module(&self) -> &Module<'a> {
    match self.program() {
      Program::Module(module) => module,
      Program::Script(_) => {
        panic!("The root node was a Script and not a Module. Use .script() or .program() instead.")
      }
    }
  }

  /// Gets the root node if the view was created from a Script; otherwise panics.
  fn script(&self) -> &Script<'a> {
    match self.program() {
      Program::Script(script) => script,
      Program::Module(_) => {
        panic!("The root node was a Module and not a Script. Use .module() or .program() instead.")
      }
    }
  }

  fn text(&self) -> &'a str {
    self.text_fast(&self.program())
  }

  fn previous_token(&self) -> Option<&'a TokenAndRange> {
    self.previous_token_fast(&self.program())
  }

  fn next_token(&self) -> Option<&'a TokenAndRange> {
    self.next_token_fast(&self.program())
  }

  /// Gets the previous tokens in the order they appear in the file.
  fn previous_tokens(&self) -> &'a [TokenAndRange] {
    self.previous_tokens_fast(&self.program())
  }

  /// Gets the next tokens in the order they appear in the file.
  fn next_tokens(&self) -> &'a [TokenAndRange] {
    self.next_tokens_fast(&self.program())
  }
}

pub trait CastableNode<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self>;
  fn kind() -> NodeKind;
}

#[derive(Clone, Copy)]
pub struct Comments<'a> {
  pub leading: &'a SingleThreadedCommentsMapInner,
  pub trailing: &'a SingleThreadedCommentsMapInner,
}

#[derive(Clone, Copy)]
pub enum ProgramRef<'a> {
  Module(&'a swc_ast::Module),
  Script(&'a swc_ast::Script),
}

impl<'a> From<&'a swc_ast::Program> for ProgramRef<'a> {
  fn from(program: &'a swc_ast::Program) -> Self {
    use swc_ast::Program;

    match program {
      Program::Module(module) => ProgramRef::Module(module),
      Program::Script(script) => ProgramRef::Script(script),
    }
  }
}

impl<'a> From<&'a swc_ast::Module> for ProgramRef<'a> {
  fn from(module: &'a swc_ast::Module) -> Self {
    ProgramRef::Module(module)
  }
}

impl<'a> From<&'a swc_ast::Script> for ProgramRef<'a> {
  fn from(script: &'a swc_ast::Script) -> Self {
    ProgramRef::Script(script)
  }
}

impl<'a> SourceRanged for ProgramRef<'a> {
  fn start(&self) -> SourcePos {
    match self {
      ProgramRef::Module(node) => node.start(),
      ProgramRef::Script(node) => node.start(),
    }
  }

  fn end(&self) -> SourcePos {
    match self {
      ProgramRef::Module(node) => node.end(),
      ProgramRef::Script(node) => node.end(),
    }
  }
}

#[derive(Clone, Copy)]
pub struct ProgramInfo<'a> {
  pub program: ProgramRef<'a>,
  pub text_info: Option<&'a SourceTextInfo>,
  pub tokens: Option<&'a [TokenAndRange]>,
  pub comments: Option<Comments<'a>>,
}

#[derive(Clone, Copy)]
pub struct ModuleInfo<'a> {
  pub module: &'a swc_ast::Module,
  pub text_info: Option<&'a SourceTextInfo>,
  pub tokens: Option<&'a [TokenAndRange]>,
  pub comments: Option<Comments<'a>>,
}

#[derive(Clone, Copy)]
pub struct ScriptInfo<'a> {
  pub script: &'a swc_ast::Script,
  pub text_info: Option<&'a SourceTextInfo>,
  pub tokens: Option<&'a [TokenAndRange]>,
  pub comments: Option<Comments<'a>>,
}

#[derive(Clone)]
pub struct AncestorIterator<'a> {
  current: Node<'a>,
}

impl<'a> AncestorIterator<'a> {
  pub fn new(node: Node<'a>) -> AncestorIterator<'a> {
    AncestorIterator { current: node }
  }
}

impl<'a> Iterator for AncestorIterator<'a> {
  type Item = Node<'a>;

  fn next(&mut self) -> Option<Node<'a>> {
    let parent = self.current.parent();
    if let Some(parent) = parent {
      self.current = parent;
    }
    parent
  }
}

impl TokenAndRange {
  pub fn token_index(&self, program: &dyn RootNode) -> usize {
    program
      .token_container()
      .get_token_index_at_start(self.range.start)
      .unwrap()
  }
}

#[cfg(test)]
mod test {
  use super::super::test_helpers::run_test;
  use crate::common::*;
  use crate::view::*;

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

  #[test]
  fn it_should_get_all_comments() {
    run_test(
      r#"
/// <reference path="foo" />
const a = 42;

/*
 * block comment
 */
let b = true;

// line comment
let c = "";

function foo(name: /* inline comment */ string) {
  console.log(`hello, ${name}`); // greeting!
}

// trailing comment
"#,
      |program| {
        assert_eq!(
          program
            .maybe_comment_container()
            .unwrap()
            .all_comments()
            .count(),
          6
        );
      },
    );
  }
}
