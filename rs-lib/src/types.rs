use crate::comments::*;
use crate::generated::*;
use crate::tokens::*;
use swc_common::{comments::SingleThreadedComments, BytePos, Spanned};
use swc_ecmascript::parser::token::TokenAndSpan;

pub enum NodeOrToken<'a> {
  Node(Node<'a>),
  Token(&'a TokenAndSpan),
}

pub trait NodeTrait<'a>: Spanned {
  fn parent(&self) -> Option<Node<'a>>;
  fn children(&self) -> Vec<Node<'a>>;
  fn into_node(&self) -> Node<'a>;
  fn kind(&self) -> NodeKind;

  fn lo(&self) -> BytePos {
    self.span().lo
  }

  fn hi(&self) -> BytePos {
    self.span().hi
  }

  fn lo_line(&self) -> usize {
    self.lo_line_fast(self.module())
  }

  fn lo_line_fast(&self, module: &Module<'a>) -> usize {
    module_to_source_file(module)
      .lookup_line(self.lo())
      .unwrap_or(0)
  }

  fn hi_line(&self) -> usize {
    self.hi_line_fast(self.module())
  }

  fn hi_line_fast(&self, module: &Module<'a>) -> usize {
    module_to_source_file(module)
      .lookup_line(self.hi())
      .unwrap_or(0)
  }

  fn lo_column(&self) -> usize {
    self.lo_column_fast(self.module())
  }

  fn lo_column_fast(&self, module: &Module<'a>) -> usize {
    get_column_at_pos(module, self.lo())
  }

  fn hi_column(&self) -> usize {
    self.hi_column_fast(self.module())
  }

  fn hi_column_fast(&self, module: &Module<'a>) -> usize {
    get_column_at_pos(module, self.hi())
  }

  fn child_index(&self) -> usize {
    if let Some(parent) = self.parent() {
      let lo = self.span().lo;
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

  fn prev_sibling(&self) -> Option<Node<'a>> {
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

  fn next_sibling(&self) -> Option<Node<'a>> {
    if let Some(parent) = self.parent() {
      let next_index = self.child_index() + 1;
      let mut children = parent.children();
      if next_index < children.len() {
        Some(children.remove(next_index))
      } else {
        None
      }
    } else {
      None
    }
  }

  fn text(&self) -> &'a str {
    self.text_fast(&self.module())
  }

  fn text_fast(&self, module: &Module<'a>) -> &'a str {
    let span = self.span();
    let source_file = module_to_source_file(module);
    &source_file.src[(span.lo.0 as usize)..(span.hi.0 as usize)]
  }

  fn tokens(&self) -> &'a [TokenAndSpan] {
    self.tokens_fast(self.module())
  }

  fn tokens_fast(&self, module: &Module<'a>) -> &'a [TokenAndSpan] {
    let span = self.span();
    let token_container = module_to_token_container(module);
    token_container.get_tokens_in_range(span.lo, span.hi)
  }

  fn children_with_tokens(&self) -> Vec<NodeOrToken<'a>> {
    self.children_with_tokens_fast(self.module())
  }

  fn children_with_tokens_fast(&self, module: &Module<'a>) -> Vec<NodeOrToken<'a>> {
    let children = self.children();
    let tokens = self.tokens_fast(module);
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
    self.leading_comments_fast(self.module())
  }

  fn leading_comments_fast(&self, module: &Module<'a>) -> CommentsIterator<'a> {
    module_to_comment_container(module).leading_comments(self.lo())
  }

  fn trailing_comments(&self) -> CommentsIterator<'a> {
    self.trailing_comments_fast(self.module())
  }

  fn trailing_comments_fast(&self, module: &Module<'a>) -> CommentsIterator<'a> {
    module_to_comment_container(module).trailing_comments(self.hi())
  }

  fn module(&self) -> &Module<'a> {
    let mut current: Node<'a> = self.into_node();
    while let Some(parent) = current.parent() {
      current = parent;
    }

    // the top-most node will always be a module
    current.to::<Module<'a>>()
  }
}

fn module_to_source_file<'a>(module: &Module<'a>) -> &'a swc_common::SourceFile {
  module
    .source_file
    .expect("The source file must be provided to `with_view` in order to use this method.")
}

fn module_to_token_container<'a>(module: &Module<'a>) -> &'a TokenContainer<'a> {
  module
    .tokens
    .as_ref()
    .expect("The tokens must be provided to `with_view` in order to use this method.")
}

fn module_to_comment_container<'a>(module: &Module<'a>) -> &'a CommentContainer<'a> {
  module
    .comments
    .as_ref()
    .expect("The comments must be provided to `with_view` in order to use this method.")
}

fn get_column_at_pos(module: &Module, pos: BytePos) -> usize {
  let source_file = module_to_source_file(module);
  let text_bytes = source_file.src.as_bytes();
  let pos = pos.0 as usize;
  let mut line_start = 0;
  for i in (0..pos).rev() {
    if text_bytes[i] == '\n' as u8 {
      line_start = i + 1;
      break;
    }
  }
  let text_slice = std::str::from_utf8(&text_bytes[line_start..pos]).unwrap();
  text_slice.chars().count()
}

pub trait CastableNode<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self>;
  fn kind() -> NodeKind;
}

pub struct SourceFileInfo<'a> {
  pub module: &'a swc_ecmascript::ast::Module,
  pub source_file: Option<&'a swc_common::SourceFile>,
  pub tokens: Option<&'a Vec<TokenAndSpan>>,
  pub comments: Option<&'a SingleThreadedComments>,
}
