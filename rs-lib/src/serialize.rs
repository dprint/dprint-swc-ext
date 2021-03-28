use super::serialize_generated::*;
use serde_json::ser::Formatter as JsonFormatter;
use std::io::{Error, Write};
use swc_ecmascript::ast::*;

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
