use rustc_hash::FxHashMap;
use swc_common::BytePos;
use swc_ecmascript::parser::token::TokenAndSpan;

pub struct TokenContainer<'a> {
  pub tokens: &'a [TokenAndSpan],
  // Uses an FxHashMap because it has faster lookups for u32 keys than the default hasher.
  lo_to_index: FxHashMap<BytePos, usize>,
  hi_to_index: FxHashMap<BytePos, usize>,
}

impl<'a> TokenContainer<'a> {
  pub fn new(tokens: &'a [TokenAndSpan]) -> Self {
    TokenContainer {
      tokens,
      lo_to_index: tokens
        .iter()
        .enumerate()
        .map(|(i, token)| (token.span.lo, i))
        .collect(),
      hi_to_index: tokens
        .iter()
        .enumerate()
        .map(|(i, token)| (token.span.hi, i))
        .collect(),
    }
  }

  pub fn get_token_index_at_lo(&self, lo: BytePos) -> usize {
    if let Some(&index) = self.lo_to_index.get(&lo) {
      index
    // fallback
    } else if let Some(&index) = self.hi_to_index.get(&lo) {
      index
    } else {
      panic!(
        "The specified lo position ({}) did not have a token index.",
        lo.0
      )
    }
  }

  pub fn get_token_index_at_hi(&self, hi: BytePos) -> usize {
    if let Some(&index) = self.hi_to_index.get(&hi) {
      index
    // fallback
    } else if let Some(&index) = self.lo_to_index.get(&hi) {
      index
    } else {
      panic!(
        "The specified hi position ({}) did not have a token index.",
        hi.0
      )
    }
  }

  pub fn get_token_at_index(&self, index: usize) -> Option<&TokenAndSpan> {
    self.tokens.get(index)
  }

  pub fn get_tokens_in_range(&self, lo: BytePos, hi: BytePos) -> &'a [TokenAndSpan] {
    let start_index = self.get_leftmost_token_index(lo);
    let end_index = self.get_rightmost_token_index(hi);

    let start_index = start_index.unwrap_or_else(|| end_index.unwrap_or(0));
    let end_index = end_index.map(|i| i + 1).unwrap_or(start_index);

    &self.tokens[start_index..end_index]
  }

  fn get_leftmost_token_index(&self, lo: BytePos) -> Option<usize> {
    if let Some(&start_index) = self.lo_to_index.get(&lo) {
      Some(start_index)
    // fallback
    } else if let Some(&start_index) = self.hi_to_index.get(&lo) {
      Some(start_index + 1)
    } else {
      // todo: binary search leftmost
      for (i, token) in self.tokens.iter().enumerate() {
        if token.span.lo >= lo {
          return Some(i);
        }
      }

      None
    }
  }

  fn get_rightmost_token_index(&self, hi: BytePos) -> Option<usize> {
    if let Some(&end_index) = self.hi_to_index.get(&hi) {
      Some(end_index)
    // fallback
    } else if let Some(&end_index) = self.lo_to_index.get(&hi) {
      if end_index > 0 {
        Some(end_index - 1)
      } else {
        None
      }
    } else {
      // todo: binary search rightmost
      for (i, token) in self.tokens.iter().enumerate().rev() {
        if token.span.hi <= hi {
          return Some(i);
        }
      }

      None
    }
  }

  pub fn get_previous_token(&self, lo: BytePos) -> Option<&TokenAndSpan> {
    let index = self.lo_to_index.get(&lo);
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
        if token.span.hi > lo {
          return last_token;
        } else {
          last_token = Some(token);
        }
      }

      None
    }
  }

  pub fn get_next_token(&self, hi: BytePos) -> Option<&TokenAndSpan> {
    if let Some(index) = self.hi_to_index.get(&hi) {
      self.tokens.get(index + 1)
    } else {
      // todo: binary search rightmost
      for token in self.tokens {
        if token.span.lo > hi {
          return Some(token);
        }
      }

      None
    }
  }
}

#[cfg(test)]
mod test {
  use swc_common::BytePos;

  use crate::RootNode;
  use crate::test_helpers::*;

  #[test]
  fn get_next_token() {
    run_test(
      r#"let /* a */ a = 5;"#,
      |program| {
        let token_container = program.token_container().unwrap();
        // low token of previous token
        assert_eq!(
          token_container.get_next_token(BytePos(0)).unwrap().span.lo(),
          BytePos(12),
        );
        // hi of previous token
        assert_eq!(
          token_container.get_next_token(BytePos(3)).unwrap().span.lo(),
          BytePos(12),
        );
        // in comment before token
        assert_eq!(
          token_container.get_next_token(BytePos(5)).unwrap().span.lo(),
          BytePos(12),
        );
        // in whitespace before token
        assert_eq!(
          token_container.get_next_token(BytePos(11)).unwrap().span.lo(),
          BytePos(12),
        );
        // at hi of last token
        assert_eq!(
          token_container.get_next_token(BytePos(18)),
          None,
        );
      },
    );
  }
}