use crate::generated::*;
use crate::tokens::*;
use swc_common::{BytePos, Spanned};
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
    let module = self.module();
    let source_file_text: &str = module
      .text
      .as_ref()
      .expect("The source file text must be provided to `with_view` in order to use this method.");
    self.text_fast(source_file_text)
  }

  fn text_fast<'b>(&self, source_file_text: &'b str) -> &'b str {
    let span = self.span();
    &source_file_text[(span.lo.0 as usize)..(span.hi.0 as usize)]
  }

  fn tokens(&self) -> &'a [TokenAndSpan] {
    let token_container = self.token_container();
    let span = self.span();
    token_container.get_tokens_in_range(span.lo, span.hi)
  }

  fn tokens_fast(&self, token_container: &TokenContainer<'a>) -> &'a [TokenAndSpan] {
    let span = self.span();
    token_container.get_tokens_in_range(span.lo, span.hi)
  }

  fn children_with_tokens(&self) -> Vec<NodeOrToken<'a>> {
    self.children_with_tokens_fast(self.token_container())
  }

  fn children_with_tokens_fast(
    &self,
    token_container: &TokenContainer<'a>,
  ) -> Vec<NodeOrToken<'a>> {
    let children = self.children();
    let tokens = self.tokens_fast(token_container);
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

  fn token_container(&self) -> &'a TokenContainer<'a> {
    let module = self.module();
    module
      .tokens
      .as_ref()
      .expect("The tokens must be provided to `with_view` in order to use this method.")
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

pub trait CastableNode<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self>;
}

pub struct SourceFileInfo<'a> {
  pub module: &'a swc_ecmascript::ast::Module,
  pub file_text: Option<&'a str>,
  pub tokens: Option<&'a TokenContainer<'a>>,
}
