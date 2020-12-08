use std::collections::HashMap;
use swc_common::BytePos;
use swc_ecmascript::parser::token::TokenAndSpan;

pub struct Tokens<'a> {
  tokens: &'a Vec<TokenAndSpan>,
  pos_to_index: HashMap<BytePos, usize>,
}

impl<'a> Tokens<'a> {
  pub fn new(tokens: &'a Vec<TokenAndSpan>) -> Self {
    Tokens {
      tokens,
      pos_to_index: tokens
        .iter()
        .enumerate()
        .map(|(i, token)| (token.span.lo, i))
        .collect(),
    }
  }
}
