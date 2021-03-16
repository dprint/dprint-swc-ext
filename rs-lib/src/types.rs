use crate::comments::*;
use crate::generated::*;
use crate::tokens::*;
use swc_common::{comments::SingleThreadedComments, BytePos, Span, Spanned};
use swc_ecmascript::parser::token::TokenAndSpan;

#[cfg(feature = "serialize")]
use serde::Serialize;

pub enum NodeOrToken<'a> {
  Node(Node<'a>),
  Token(&'a TokenAndSpan),
}

impl<'a> NodeOrToken<'a> {
  pub fn unwrap_token(&self) -> &'a TokenAndSpan {
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

impl<'a> Spanned for NodeOrToken<'a> {
  fn span(&self) -> Span {
    match self {
      NodeOrToken::Node(node) => node.span(),
      NodeOrToken::Token(token) => token.span(),
    }
  }
}

/// A Module or Script node.
pub trait RootNode<'a> {
  fn source_file(&self) -> Option<&'a swc_common::SourceFile>;
  fn tokens(&self) -> Option<&'a TokenContainer<'a>>;
  fn comments(&self) -> Option<&'a CommentContainer<'a>>;

  fn token_at_index(&self, index: usize) -> Option<&'a TokenAndSpan> {
    let tokens = self.tokens();
    let token_container = tokens
      .as_ref()
      .expect("The tokens must be provided to `with_view` in order to use this method.");
    token_container.get_token_at_index(index)
  }
}

macro_rules! implement_root_node {
  ($name:ty) => {
    impl<'a> RootNode<'a> for $name {
      fn source_file(&self) -> Option<&'a swc_common::SourceFile> {
        self.source_file
      }

      fn tokens(&self) -> Option<&'a TokenContainer<'a>> {
        self.tokens
      }

      fn comments(&self) -> Option<&'a CommentContainer<'a>> {
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
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum Program<'a> {
  Module(&'a Module<'a>),
  Script(&'a Script<'a>),
}

impl<'a> Spanned for Program<'a> {
  fn span(&self) -> Span {
    match self {
      Program::Module(node) => node.span(),
      Program::Script(node) => node.span(),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      Program::Module(node) => node.into_node(),
      Program::Script(node) => node.into_node(),
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
  fn source_file(&self) -> Option<&'a swc_common::SourceFile> {
    match self {
      Program::Module(module) => module.source_file,
      Program::Script(script) => script.source_file,
    }
  }

  fn tokens(&self) -> Option<&'a TokenContainer<'a>> {
    match self {
      Program::Module(module) => module.tokens,
      Program::Script(script) => script.tokens,
    }
  }

  fn comments(&self) -> Option<&'a CommentContainer<'a>> {
    match self {
      Program::Module(module) => module.comments,
      Program::Script(script) => script.comments,
    }
  }
}

pub trait SpannedExt {
  fn lo(&self) -> BytePos;
  fn hi(&self) -> BytePos;
  fn start_line_fast(&self, program: &dyn RootNode) -> usize;
  fn end_line_fast(&self, program: &dyn RootNode) -> usize;
  fn start_column_fast(&self, program: &dyn RootNode) -> usize;
  fn end_column_fast(&self, program: &dyn RootNode) -> usize;
  fn width_fast(&self, program: &dyn RootNode) -> usize;
  fn tokens_fast<'a>(&self, program: &dyn RootNode<'a>) -> &'a [TokenAndSpan];
  fn text_fast<'a>(&self, program: &dyn RootNode<'a>) -> &'a str;
  fn leading_comments_fast<'a>(&self, program: &dyn RootNode<'a>) -> CommentsIterator<'a>;
  fn trailing_comments_fast<'a>(&self, program: &dyn RootNode<'a>) -> CommentsIterator<'a>;

  fn previous_token_fast<'a>(&self, program: &dyn RootNode<'a>) -> Option<&'a TokenAndSpan> {
    let token_container = root_node_to_token_container(program);
    let index = token_container.get_token_index_at_lo(self.lo());
    if index == 0 {
      None
    } else {
      token_container.get_token_at_index(index - 1)
    }
  }

  fn next_token_fast<'a>(&self, program: &dyn RootNode<'a>) -> Option<&'a TokenAndSpan> {
    let token_container = root_node_to_token_container(program);
    let index = token_container.get_token_index_at_hi(self.hi());
    token_container.get_token_at_index(index + 1)
  }

  fn previous_tokens_fast<'a>(&self, program: &dyn RootNode<'a>) -> &'a [TokenAndSpan] {
    let token_container = root_node_to_token_container(program);
    let index = token_container.get_token_index_at_lo(self.lo());
    &token_container.tokens[0..index]
  }

  fn next_tokens_fast<'a>(&self, program: &dyn RootNode<'a>) -> &'a [TokenAndSpan] {
    let token_container = root_node_to_token_container(program);
    let index = token_container.get_token_index_at_hi(self.hi());
    &token_container.tokens[index + 1..]
  }
}

impl<T> SpannedExt for T
where
  T: Spanned,
{
  fn lo(&self) -> BytePos {
    self.span().lo
  }

  fn hi(&self) -> BytePos {
    self.span().hi
  }

  fn start_line_fast(&self, program: &dyn RootNode) -> usize {
    root_node_to_source_file(program)
      .lookup_line(self.lo())
      .unwrap_or(0)
  }

  fn end_line_fast(&self, program: &dyn RootNode) -> usize {
    root_node_to_source_file(program)
      .lookup_line(self.hi())
      .unwrap_or(0)
  }

  fn start_column_fast(&self, program: &dyn RootNode) -> usize {
    get_column_at_pos(program, self.lo())
  }

  fn end_column_fast(&self, program: &dyn RootNode) -> usize {
    get_column_at_pos(program, self.hi())
  }

  fn width_fast(&self, program: &dyn RootNode) -> usize {
    self.text_fast(program).chars().count()
  }

  fn tokens_fast<'a>(&self, program: &dyn RootNode<'a>) -> &'a [TokenAndSpan] {
    let span = self.span();
    let token_container = root_node_to_token_container(program);
    token_container.get_tokens_in_range(span.lo, span.hi)
  }

  fn text_fast<'a>(&self, program: &dyn RootNode<'a>) -> &'a str {
    let span = self.span();
    let source_file = root_node_to_source_file(program);
    &source_file.src[(span.lo.0 as usize)..(span.hi.0 as usize)]
  }

  fn leading_comments_fast<'a>(&self, program: &dyn RootNode<'a>) -> CommentsIterator<'a> {
    root_node_to_comment_container(program).leading_comments(self.lo())
  }

  fn trailing_comments_fast<'a>(&self, program: &dyn RootNode<'a>) -> CommentsIterator<'a> {
    root_node_to_comment_container(program).trailing_comments(self.hi())
  }
}

pub trait NodeTrait<'a>: SpannedExt {
  fn parent(&self) -> Option<Node<'a>>;
  fn children(&self) -> Vec<Node<'a>>;
  fn into_node(&self) -> Node<'a>;
  fn kind(&self) -> NodeKind;

  fn ancestors(&self) -> AncestorIterator<'a> {
    AncestorIterator::new(self.into_node())
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

  fn width(&self) -> usize {
    self.width_fast(&self.program())
  }

  fn child_index(&self) -> usize {
    if let Some(parent) = self.parent() {
      let lo = self.lo();
      for (i, child) in parent.children().iter().enumerate() {
        if child.span().lo == lo {
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

  fn tokens(&self) -> &'a [TokenAndSpan] {
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
      let child_span = child.span();

      // get the tokens before the current child
      for token in &tokens[tokens_index..] {
        if token.span.lo() < child_span.lo {
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
        if token.span.hi() <= child_span.hi {
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
    let mut current: Node<'a> = self.into_node();
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

  fn previous_token(&self) -> Option<&'a TokenAndSpan> {
    self.previous_token_fast(&self.program())
  }

  fn next_token(&self) -> Option<&'a TokenAndSpan> {
    self.next_token_fast(&self.program())
  }

  /// Gets the previous tokens in the order they appear in the file.
  fn previous_tokens(&self) -> &'a [TokenAndSpan] {
    self.previous_tokens_fast(&self.program())
  }

  /// Gets the next tokens in the order they appear in the file.
  fn next_tokens(&self) -> &'a [TokenAndSpan] {
    self.next_tokens_fast(&self.program())
  }
}

pub trait TokenExt {
  fn token_index(&self, program: &dyn RootNode) -> usize;
}

impl TokenExt for TokenAndSpan {
  fn token_index(&self, program: &dyn RootNode) -> usize {
    let token_container = root_node_to_token_container(program);
    token_container.get_token_index_at_lo(self.span.lo)
  }
}

fn root_node_to_source_file<'a>(root_node: &dyn RootNode<'a>) -> &'a swc_common::SourceFile {
  root_node
    .source_file()
    .expect("The source file must be provided to `with_view` in order to use this method.")
}

fn root_node_to_token_container<'a>(root_node: &dyn RootNode<'a>) -> &'a TokenContainer<'a> {
  root_node
    .tokens()
    .as_ref()
    .expect("The tokens must be provided to `with_view` in order to use this method.")
}

fn root_node_to_comment_container<'a>(root_node: &dyn RootNode<'a>) -> &'a CommentContainer<'a> {
  root_node
    .comments()
    .as_ref()
    .expect("The comments must be provided to `with_view` in order to use this method.")
}

fn get_column_at_pos(program: &dyn RootNode, pos: BytePos) -> usize {
  let source_file = root_node_to_source_file(program);
  let text_bytes = source_file.src.as_bytes();
  let pos = pos.0 as usize;
  let mut line_start = 0;
  for i in (0..pos).rev() {
    if text_bytes[i] == '\n' as u8 {
      line_start = i + 1;
      break;
    }
  }
  let text_slice = &source_file.src[line_start..pos];
  text_slice.chars().count()
}

pub trait CastableNode<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self>;
  fn kind() -> NodeKind;
}

#[derive(Clone, Copy)]
pub struct ProgramInfo<'a> {
  pub program: &'a swc_ecmascript::ast::Program,
  pub source_file: Option<&'a swc_common::SourceFile>,
  pub tokens: Option<&'a Vec<TokenAndSpan>>,
  pub comments: Option<&'a SingleThreadedComments>,
}

#[derive(Clone, Copy)]
pub struct ModuleInfo<'a> {
  pub module: &'a swc_ecmascript::ast::Module,
  pub source_file: Option<&'a swc_common::SourceFile>,
  pub tokens: Option<&'a Vec<TokenAndSpan>>,
  pub comments: Option<&'a SingleThreadedComments>,
}

#[derive(Clone, Copy)]
pub struct ScriptInfo<'a> {
  pub script: &'a swc_ecmascript::ast::Script,
  pub source_file: Option<&'a swc_common::SourceFile>,
  pub tokens: Option<&'a Vec<TokenAndSpan>>,
  pub comments: Option<&'a SingleThreadedComments>,
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
