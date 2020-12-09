use std::collections::HashMap;
use swc_common::BytePos;
use swc_ecmascript::parser::token::TokenAndSpan;

pub struct TokenContainer<'a> {
  tokens: &'a Vec<TokenAndSpan>,
  // todo: investigate this
  lo_to_index: HashMap<BytePos, usize>,
  hi_to_index: HashMap<BytePos, usize>,
}

impl<'a> TokenContainer<'a> {
  pub fn new(tokens: &'a Vec<TokenAndSpan>) -> Self {
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

  pub fn get_tokens_in_range(&self, lo: BytePos, hi: BytePos) -> &'a [TokenAndSpan] {
    let start_index = self.get_index_at_lo(lo);
    let end_index = self.get_index_at_hi(hi) + 1;
    return &self.tokens[start_index..end_index];
  }

  pub fn get_previous_token_hi(&self, lo: BytePos) -> Option<BytePos> {
    let index = self.get_index_at_lo(lo);
    if index == 0 {
      None
    } else {
      Some(self.tokens[index - 1].span.hi)
    }
  }

  pub fn get_next_token_lo(&self, hi: BytePos) -> Option<BytePos> {
    let index = self.get_index_at_hi(hi);
    self.tokens.get(index + 1).map(|t| t.span.lo)
  }

  fn get_index_at_lo(&self, lo: BytePos) -> usize {
    if let Some(index) = self.lo_to_index.get(&lo) {
      *index
    } else {
      panic!("Could not find token at lo: ${}", lo.0)
    }
  }

  fn get_index_at_hi(&self, hi: BytePos) -> usize {
    if let Some(index) = self.hi_to_index.get(&hi) {
      *index
    } else {
      panic!("Could not find token at hi: ${}", hi.0)
    }
  }
}
