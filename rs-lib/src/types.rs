use crate::generated::*;
use swc_common::Spanned;

pub trait NodeTrait: Spanned {
  fn parent(&self) -> Option<Node>;
  fn children(&self) -> Vec<Node>;

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

  fn prev_sibling(&self) -> Option<Node> {
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

  fn next_sibling(&self) -> Option<Node> {
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

  fn text<'a>(&self, source_file_text: &'a str) -> &'a str {
    let span = self.span();
    &source_file_text[(span.lo.0 as usize)..(span.hi.0 as usize)]
  }
}

pub trait CastableNode {
  fn try_cast(node: &Node) -> Option<&'static Self>;
}
