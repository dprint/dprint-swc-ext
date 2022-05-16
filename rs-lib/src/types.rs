use std::fmt::Display;

use crate::comments::*;
use crate::generated::*;
use crate::source_file::SourceFile;
use crate::tokens::*;
use swc_common::comments::SingleThreadedCommentsMapInner;
use swc_common::BytePos;
use swc_common::Span;
use swc_common::Spanned;
use swc_ecmascript::ast as swc_ast;
use swc_ecmascript::parser::token::Token;
use swc_ecmascript::parser::token::TokenAndSpan;

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

/// A Module or Script node.
pub trait RootNode<'a> {
  fn source_file(&self) -> Option<&'a dyn SourceFile>;
  fn token_container(&self) -> Option<&'a TokenContainer<'a>>;
  fn comment_container(&self) -> Option<&'a CommentContainer<'a>>;

  fn token_at_index(&self, index: usize) -> Option<&'a TokenAndRange> {
    let token_container = self.token_container();
    let token_container = token_container
      .as_ref()
      .expect("The tokens must be provided to `with_view` in order to use this method.");
    token_container.get_token_at_index(index)
  }
}

macro_rules! implement_root_node {
  ($name:ty) => {
    impl<'a> RootNode<'a> for $name {
      fn source_file(&self) -> Option<&'a dyn SourceFile> {
        self.source_file
      }

      fn token_container(&self) -> Option<&'a TokenContainer<'a>> {
        self.tokens
      }

      fn comment_container(&self) -> Option<&'a CommentContainer<'a>> {
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
  fn source_file(&self) -> Option<&'a dyn SourceFile> {
    match self {
      Program::Module(module) => module.source_file,
      Program::Script(script) => script.source_file,
    }
  }

  fn token_container(&self) -> Option<&'a TokenContainer<'a>> {
    match self {
      Program::Module(module) => module.tokens,
      Program::Script(script) => script.tokens,
    }
  }

  fn comment_container(&self) -> Option<&'a CommentContainer<'a>> {
    match self {
      Program::Module(module) => module.comments,
      Program::Script(script) => script.comments,
    }
  }
}

/// A token with its position.
#[derive(Debug, Clone, PartialEq)]
pub struct TokenAndRange {
  pub token: Token,
  pub range: SourceRange,
}

impl TokenAndRange {
  pub fn token_index(&self, program: &dyn RootNode) -> usize {
    let token_container = root_node_to_token_container(program);
    token_container
      .get_token_index_at_start(self.range.start)
      .unwrap()
  }
}

impl From<TokenAndSpan> for TokenAndRange {
  fn from(value: TokenAndSpan) -> Self {
    TokenAndRange {
      token: value.token,
      range: value.span.into(),
    }
  }
}

impl SourceRanged for TokenAndRange {
  fn start(&self) -> SourcePos {
    self.range.start
  }

  fn end(&self) -> SourcePos {
    self.range.end
  }
}

/// Swc unfortunately uses `BytePos(0)` as a magic value. This means
/// that we can't have byte positions of nodes line up with the text.
/// To get around this, we have created our own `SourcePos` wrapper
/// that exposes a 0-indexed value, but hides the underlying swc
/// byte position which is not 0-indexed.
///
/// When using this type, you MUST parse files in swc using
/// `SourcePos::START_BYTE_POS` otherwise bad things will happen.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourcePos(BytePos);

impl SourcePos {
  /// Use this value as the start byte position when parsing.
  pub const START_BYTE_POS: BytePos = BytePos(1_000);

  pub fn new(index: usize) -> Self {
    Self(BytePos(index as u32) + SourcePos::START_BYTE_POS)
  }

  pub fn from_byte_pos(byte_pos: BytePos) -> Self {
    #[cfg(debug_assertions)]
    if byte_pos < SourcePos::START_BYTE_POS {
      panic!(concat!(
        "The provided byte position was less than the start byte position. ",
        "Ensure the source file is parsed starting at SourcePos::START_BYTE_POS."
      ))
    }
    Self(byte_pos)
  }

  pub fn as_byte_pos(&self) -> BytePos {
    self.0
  }

  pub fn as_usize(&self) -> usize {
    (self.0 - SourcePos::START_BYTE_POS).0 as usize
  }
}

impl std::fmt::Debug for SourcePos {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("SourcePos").field(&self.as_usize()).finish()
  }
}

impl Display for SourcePos {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.as_usize().to_string())
  }
}

impl std::ops::Add<usize> for SourcePos {
  type Output = SourcePos;

  fn add(self, rhs: usize) -> Self::Output {
    SourcePos(BytePos(self.0.0 + rhs as u32))
  }
}

impl std::ops::Sub<usize> for SourcePos {
  type Output = SourcePos;

  fn sub(self, rhs: usize) -> Self::Output {
    SourcePos(BytePos(self.0.0 - rhs as u32))
  }
}

impl std::ops::Sub<SourcePos> for SourcePos {
  type Output = usize;

  fn sub(self, rhs: SourcePos) -> Self::Output {
    self.as_usize() - rhs.as_usize()
  }
}

impl SourceRanged for SourcePos {
  fn start(&self) -> SourcePos {
    *self
  }

  fn end(&self) -> SourcePos {
    *self
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRange {
  pub start: SourcePos,
  pub end: SourcePos,
}

impl SourceRange {
  pub fn new(start: SourcePos, end: SourcePos)  -> Self{
    Self { start, end }
  }
}

impl From<Span> for SourceRange {
  fn from(value: Span) -> Self {
    SourceRange {
      start: SourcePos::from_byte_pos(value.lo),
      end: SourcePos::from_byte_pos(value.hi),
    }
  }
}

pub trait SourceRanged {
  fn start(&self) -> SourcePos;
  fn end(&self) -> SourcePos;

  fn range(&self) -> SourceRange {
    SourceRange {
      start: self.start(),
      end: self.end(),
    }
  }

  fn start_line_fast(&self, program: &dyn RootNode) -> usize {
    root_node_to_source_file(program).line_index(self.start())
  }

  fn end_line_fast(&self, program: &dyn RootNode) -> usize {
    root_node_to_source_file(program).line_index(self.end())
  }

  fn start_column_fast(&self, program: &dyn RootNode) -> usize {
    get_column_at_pos(program, self.start())
  }

  fn end_column_fast(&self, program: &dyn RootNode) -> usize {
    get_column_at_pos(program, self.end())
  }

  fn char_width_fast(&self, program: &dyn RootNode) -> usize {
    self.text_fast(program).chars().count()
  }

  fn tokens_fast<'a>(&self, program: &dyn RootNode<'a>) -> &'a [TokenAndRange] {
    let token_container = root_node_to_token_container(program);
    token_container.get_tokens_in_range(self.start(), self.end())
  }

  fn text_fast<'a>(&self, program: &dyn RootNode<'a>) -> &'a str {
    let source_file = root_node_to_source_file(program);
    &source_file.text()[self.byte_range()]
  }

  fn leading_comments_fast<'a>(&self, program: &dyn RootNode<'a>) -> CommentsIterator<'a> {
    root_node_to_comment_container(program).leading_comments(self.start())
  }

  fn trailing_comments_fast<'a>(&self, program: &dyn RootNode<'a>) -> CommentsIterator<'a> {
    root_node_to_comment_container(program).trailing_comments(self.end())
  }

  fn byte_width(&self) -> usize {
    self.end().as_usize() - self.start().as_usize()
  }

  fn byte_range(&self) -> std::ops::Range<usize> {
    self.start().as_usize()..self.end().as_usize()
  }

  fn previous_token_fast<'a>(&self, program: &dyn RootNode<'a>) -> Option<&'a TokenAndRange> {
    let token_container = root_node_to_token_container(program);
    token_container.get_previous_token(self.start())
  }

  fn next_token_fast<'a>(&self, program: &dyn RootNode<'a>) -> Option<&'a TokenAndRange> {
    let token_container = root_node_to_token_container(program);
    token_container.get_next_token(self.end())
  }

  fn previous_tokens_fast<'a>(&self, program: &dyn RootNode<'a>) -> &'a [TokenAndRange] {
    let token_container = root_node_to_token_container(program);
    let index = token_container
      .get_token_index_at_start(self.start())
      // fallback
      .or_else(|| token_container.get_token_index_at_end(self.start()))
      .unwrap_or_else(|| {
        panic!(
          "The specified start position ({}) did not have a token index.",
          self.start()
        )
      });
    &token_container.tokens[0..index]
  }

  fn next_tokens_fast<'a>(&self, program: &dyn RootNode<'a>) -> &'a [TokenAndRange] {
    let token_container = root_node_to_token_container(program);
    let index = token_container
      .get_token_index_at_end(self.end())
      // fallback
      .or_else(|| token_container.get_token_index_at_start(self.end()))
      .unwrap_or_else(|| {
        panic!(
          "The specified end position ({}) did not have a token index.",
          self.end()
        )
      });
    &token_container.tokens[index + 1..]
  }
}

impl<T> SourceRanged for T
where
  T: Spanned,
{
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.span().lo)
  }

  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.span().hi)
  }
}

pub trait NodeTrait<'a>: SourceRanged {
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

fn root_node_to_source_file<'a>(root_node: &dyn RootNode<'a>) -> &'a dyn SourceFile {
  root_node
    .source_file()
    .expect("The source file must be provided to `with_view` in order to use this method.")
}

fn root_node_to_token_container<'a>(root_node: &dyn RootNode<'a>) -> &'a TokenContainer<'a> {
  root_node
    .token_container()
    .as_ref()
    .expect("The tokens must be provided to `with_view` in order to use this method.")
}

fn root_node_to_comment_container<'a>(root_node: &dyn RootNode<'a>) -> &'a CommentContainer<'a> {
  root_node
    .comment_container()
    .as_ref()
    .expect("The comments must be provided to `with_view` in order to use this method.")
}

fn get_column_at_pos(program: &dyn RootNode, pos: SourcePos) -> usize {
  let source_file = root_node_to_source_file(program);
  let text_bytes = source_file.text().as_bytes();
  let pos = pos.as_usize();
  let mut line_start = 0;
  for i in (0..pos).rev() {
    if text_bytes[i] == b'\n' {
      line_start = i + 1;
      break;
    }
  }
  let text_slice = &source_file.text()[line_start..pos];
  text_slice.chars().count()
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
  pub source_file: Option<&'a dyn SourceFile>,
  pub tokens: Option<&'a [TokenAndRange]>,
  pub comments: Option<Comments<'a>>,
}

#[derive(Clone, Copy)]
pub struct ModuleInfo<'a> {
  pub module: &'a swc_ast::Module,
  pub source_file: Option<&'a dyn SourceFile>,
  pub tokens: Option<&'a [TokenAndRange]>,
  pub comments: Option<Comments<'a>>,
}

#[derive(Clone, Copy)]
pub struct ScriptInfo<'a> {
  pub script: &'a swc_ast::Script,
  pub source_file: Option<&'a dyn SourceFile>,
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

#[cfg(test)]
mod test {
  use crate::test_helpers::run_test;
  use crate::*;

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
          program.comment_container().unwrap().all_comments().count(),
          6
        );
      },
    );
  }
}
