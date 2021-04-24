use super::serialize_generated::*;
use serde_json::ser::Formatter as JsonFormatter;
use std::io::{Error, Write};
use swc_common::comments::SingleThreadedCommentsMapInner;
use swc_ecmascript::ast::*;
use swc_ecmascript::parser::token::TokenAndSpan;

pub fn serialize_program(
  w: &mut impl Write,
  f: &mut impl JsonFormatter,
  file_text: &str,
  node: &Program,
) -> Result<(), Error> {
  let mut serializer = FileSerializer::new(w, f, file_text);
  match node {
    Program::Module(module) => serializer.serialize_module(module),
    Program::Script(script) => serializer.serialize_script(script),
  }
}

pub fn serialize_module(
  w: &mut impl Write,
  f: &mut impl JsonFormatter,
  file_text: &str,
  module: &Module,
) -> Result<(), Error> {
  let mut serializer = FileSerializer::new(w, f, file_text);
  serializer.serialize_module(module)
}

pub fn serialize_token_and_spans(
  w: &mut impl Write,
  f: &mut impl JsonFormatter,
  file_text: &str,
  token_and_spans: &Vec<TokenAndSpan>,
) -> Result<(), Error> {
  let mut serializer = FileSerializer::new(w, f, file_text);
  serializer.serialize_token_and_spans(token_and_spans)
}

pub fn serialize_comments(
  w: &mut impl Write,
  f: &mut impl JsonFormatter,
  file_text: &str,
  leading: &SingleThreadedCommentsMapInner,
  trailing: &SingleThreadedCommentsMapInner,
) -> Result<(), Error> {
  let mut serializer = FileSerializer::new(w, f, file_text);
  serializer.serialize_comments(leading, trailing)
}
