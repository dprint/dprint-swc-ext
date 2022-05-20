use rustc_hash::FxHashMap;

use super::pos::*;
use crate::swc::parser::token::TokenAndSpan;

pub struct TokenContainer<'a> {
  pub tokens: &'a [TokenAndSpan],
  // Uses an FxHashMap because it has faster lookups for u32 keys than the default hasher.
  start_to_index: FxHashMap<SourcePos, usize>,
  end_to_index: FxHashMap<SourcePos, usize>,
}

impl<'a> TokenContainer<'a> {
  pub fn new(tokens: &'a [TokenAndSpan]) -> Self {
    TokenContainer {
      tokens,
      start_to_index: tokens.iter().enumerate().map(|(i, token)| (token.start(), i)).collect(),
      end_to_index: tokens.iter().enumerate().map(|(i, token)| (token.end(), i)).collect(),
    }
  }

  pub fn get_token_index_at_start(&self, start: SourcePos) -> Option<usize> {
    self.start_to_index.get(&start).copied()
  }

  pub fn get_token_index_at_end(&self, end: SourcePos) -> Option<usize> {
    self.end_to_index.get(&end).copied()
  }

  pub fn get_token_at_index(&self, index: usize) -> Option<&TokenAndSpan> {
    self.tokens.get(index)
  }

  pub fn get_tokens_in_range(&self, start: SourcePos, end: SourcePos) -> &'a [TokenAndSpan] {
    let start_index = self.get_leftmost_token_index(start);
    let end_index = self.get_rightmost_token_index(end);

    let start_index = start_index.unwrap_or_else(|| end_index.unwrap_or(0));
    let end_index = end_index.map(|i| i + 1).unwrap_or(start_index);

    &self.tokens[start_index..end_index]
  }

  fn get_leftmost_token_index(&self, start: SourcePos) -> Option<usize> {
    if let Some(&start_index) = self.start_to_index.get(&start) {
      Some(start_index)
    // fallback
    } else if let Some(&start_index) = self.end_to_index.get(&start) {
      Some(start_index + 1)
    } else {
      // todo: binary search leftmost
      for (i, token) in self.tokens.iter().enumerate() {
        if token.start() >= start {
          return Some(i);
        }
      }

      None
    }
  }

  fn get_rightmost_token_index(&self, end: SourcePos) -> Option<usize> {
    if let Some(&end_index) = self.end_to_index.get(&end) {
      Some(end_index)
    // fallback
    } else if let Some(&end_index) = self.start_to_index.get(&end) {
      if end_index > 0 {
        Some(end_index - 1)
      } else {
        None
      }
    } else {
      // todo: binary search rightmost
      for (i, token) in self.tokens.iter().enumerate().rev() {
        if token.end() <= end {
          return Some(i);
        }
      }

      None
    }
  }

  pub fn get_previous_token(&self, start: SourcePos) -> Option<&TokenAndSpan> {
    let index = self.start_to_index.get(&start);
    if let Some(&index) = index {
      if index == 0 {
        None
      } else {
        Some(&self.tokens[index - 1])
      }
    } else {
      // todo: binary search leftmost
      let mut last_token = None;
      for token in self.tokens {
        if token.end() > start {
          return last_token;
        } else {
          last_token = Some(token);
        }
      }

      None
    }
  }

  pub fn get_next_token(&self, end: SourcePos) -> Option<&TokenAndSpan> {
    if let Some(index) = self.end_to_index.get(&end) {
      self.tokens.get(index + 1)
    } else {
      // todo: binary search rightmost
      for token in self.tokens {
        if token.start() > end {
          return Some(token);
        }
      }

      None
    }
  }
}

#[cfg(test)]
mod test {
  use std::path::PathBuf;

  use super::super::pos::SourcePos;
  use super::TokenContainer;
  use crate::common::SourceRangedForSpanned;
  use crate::test_helpers::*;

  #[test]
  fn get_next_token() {
    let (_, tokens, _, _) = get_swc_module(&PathBuf::from("path.js"), r#"let /* a */ a = 5;"#);
    let token_container = TokenContainer::new(&tokens);
    // low token of previous token
    assert_eq!(token_container.get_next_token(SourcePos::new(0)).unwrap().start(), SourcePos::new(12));
    // hi of previous token
    assert_eq!(token_container.get_next_token(SourcePos::new(3)).unwrap().start(), SourcePos::new(12));
    // in comment before token
    assert_eq!(token_container.get_next_token(SourcePos::new(5)).unwrap().start(), SourcePos::new(12));
    // in whitespace before token
    assert_eq!(token_container.get_next_token(SourcePos::new(11)).unwrap().start(), SourcePos::new(12));
    // at hi of last token
    assert_eq!(token_container.get_next_token(SourcePos::new(18)), None);
  }
}
