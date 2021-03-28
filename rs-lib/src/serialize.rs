use super::serialize_generated::*;
use serde_json::ser::{to_string as to_json_string, Formatter as JsonFormatter};
use std::io::{Error, Write};
use swc_common::{
  comments::{Comment, CommentKind, SingleThreadedCommentsMapInner},
  Span, Spanned,
};
use swc_ecmascript::ast::*;
use swc_ecmascript::parser::token::{BinOpToken, Keyword, Token, TokenAndSpan, Word};

pub fn serialize_program(
  w: &mut impl Write,
  f: &mut impl JsonFormatter,
  node: &Program,
) -> Result<(), Error> {
  match node {
    Program::Module(module) => serialize_module(w, f, module),
    Program::Script(script) => serialize_script(w, f, script),
  }
}
