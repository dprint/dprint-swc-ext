// This code is code generated.
// Run `./scripts/generate.sh` from the root directory to regenerate it.
use std::io::{Error, Write};
use serde_json::ser::{Formatter as JsonFormatter, to_string as to_json_string};
use swc_common::{Spanned, comments::{Comment, CommentKind, SingleThreadedCommentsMapInner}};
use swc_ecmascript::parser::token::{BinOpToken, Keyword, Token, TokenAndSpan, Word};
use swc_ecmascript::ast::*;

pub fn serialize_array_lit(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ArrayLit) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 0)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "elems")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.elems.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    match item {
      Some(value) => {
        serialize_expr_or_spread(w, f, value)?;
      }
      None => f.write_null(w)?,
    }
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_array_pat(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ArrayPat) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 1)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "elems")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.elems.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    match item {
      Some(value) => {
        serialize_pat(w, f, value)?;
      }
      None => f.write_null(w)?,
    }
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "optional")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.optional)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_ann {
    Some(value) => {
      serialize_ts_type_ann(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_arrow_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ArrowExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 2)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "params")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.params.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_pat(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "body")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_block_stmt_or_expr(w, f, &node.body)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "isAsync")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.is_async)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "isGenerator")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.is_generator)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeParams")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_params {
    Some(value) => {
      serialize_ts_type_param_decl(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "returnType")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.return_type {
    Some(value) => {
      serialize_ts_type_ann(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_assign_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &AssignExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 3)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "op")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.op)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "left")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_pat_or_expr(w, f, &node.left)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "right")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.right)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_assign_pat(w: &mut impl Write, f: &mut impl JsonFormatter, node: &AssignPat) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 4)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "left")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_pat(w, f, &node.left)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "right")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.right)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_ann {
    Some(value) => {
      serialize_ts_type_ann(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_assign_pat_prop(w: &mut impl Write, f: &mut impl JsonFormatter, node: &AssignPatProp) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 5)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "key")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.key)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "value")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.value {
    Some(value) => {
      serialize_expr(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_assign_prop(w: &mut impl Write, f: &mut impl JsonFormatter, node: &AssignProp) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 6)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "key")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.key)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "value")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.value)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_await_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &AwaitExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 7)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "arg")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.arg)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_big_int(w: &mut impl Write, f: &mut impl JsonFormatter, node: &BigInt) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 8)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "value")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.value)?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_bin_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &BinExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 9)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "op")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.op)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "left")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.left)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "right")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.right)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_binding_ident(w: &mut impl Write, f: &mut impl JsonFormatter, node: &BindingIdent) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 10)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "id")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.id)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_ann {
    Some(value) => {
      serialize_ts_type_ann(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_block_stmt(w: &mut impl Write, f: &mut impl JsonFormatter, node: &BlockStmt) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 11)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "stmts")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.stmts.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_stmt(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_bool(w: &mut impl Write, f: &mut impl JsonFormatter, node: &Bool) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 12)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "value")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.value)?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_break_stmt(w: &mut impl Write, f: &mut impl JsonFormatter, node: &BreakStmt) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 13)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "label")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.label {
    Some(value) => {
      serialize_ident(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_call_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &CallExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 14)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "callee")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr_or_super(w, f, &node.callee)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "args")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.args.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_expr_or_spread(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeArgs")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_args {
    Some(value) => {
      serialize_ts_type_param_instantiation(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_catch_clause(w: &mut impl Write, f: &mut impl JsonFormatter, node: &CatchClause) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 15)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "param")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.param {
    Some(value) => {
      serialize_pat(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "body")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_block_stmt(w, f, &node.body)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_class(w: &mut impl Write, f: &mut impl JsonFormatter, node: &Class) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 16)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "decorators")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.decorators.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_decorator(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "body")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.body.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_class_member(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "superClass")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.super_class {
    Some(value) => {
      serialize_expr(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "isAbstract")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.is_abstract)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeParams")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_params {
    Some(value) => {
      serialize_ts_type_param_decl(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "superTypeParams")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.super_type_params {
    Some(value) => {
      serialize_ts_type_param_instantiation(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "implements")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.implements.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_ts_expr_with_type_args(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_class_decl(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ClassDecl) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 17)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "ident")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.ident)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "declare")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.declare)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "class")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_class(w, f, &node.class)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_class_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ClassExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 18)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "ident")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.ident {
    Some(value) => {
      serialize_ident(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "class")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_class(w, f, &node.class)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_class_method(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ClassMethod) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 19)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "key")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_prop_name(w, f, &node.key)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "function")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_function(w, f, &node.function)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "methodKind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.kind)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "isStatic")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.is_static)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "accessibility")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.accessibility {
    Some(value) => {
      write!(w, "{}", to_json_string(value)?)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "isAbstract")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.is_abstract)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "isOptional")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.is_optional)?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_class_prop(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ClassProp) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 20)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "key")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.key)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "value")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.value {
    Some(value) => {
      serialize_expr(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_ann {
    Some(value) => {
      serialize_ts_type_ann(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "isStatic")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.is_static)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "decorators")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.decorators.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_decorator(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "computed")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.computed)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "accessibility")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.accessibility {
    Some(value) => {
      write!(w, "{}", to_json_string(value)?)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "isAbstract")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.is_abstract)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "isOptional")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.is_optional)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "readonly")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.readonly)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "declare")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.declare)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "definite")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.definite)?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_computed_prop_name(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ComputedPropName) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 21)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "expr")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.expr)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_cond_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &CondExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 22)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "test")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.test)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "cons")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.cons)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "alt")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.alt)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_constructor(w: &mut impl Write, f: &mut impl JsonFormatter, node: &Constructor) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 23)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "key")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_prop_name(w, f, &node.key)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "params")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.params.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_param_or_ts_param_prop(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "body")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.body {
    Some(value) => {
      serialize_block_stmt(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "accessibility")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.accessibility {
    Some(value) => {
      write!(w, "{}", to_json_string(value)?)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "isOptional")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.is_optional)?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_continue_stmt(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ContinueStmt) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 24)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "label")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.label {
    Some(value) => {
      serialize_ident(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_debugger_stmt(w: &mut impl Write, f: &mut impl JsonFormatter, node: &DebuggerStmt) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 25)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_decorator(w: &mut impl Write, f: &mut impl JsonFormatter, node: &Decorator) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 26)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "expr")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.expr)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_do_while_stmt(w: &mut impl Write, f: &mut impl JsonFormatter, node: &DoWhileStmt) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 27)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "test")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.test)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "body")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_stmt(w, f, &node.body)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_empty_stmt(w: &mut impl Write, f: &mut impl JsonFormatter, node: &EmptyStmt) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 28)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_export_all(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ExportAll) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 29)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "src")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_str(w, f, &node.src)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "asserts")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.asserts {
    Some(value) => {
      serialize_object_lit(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_export_decl(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ExportDecl) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 30)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "decl")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_decl(w, f, &node.decl)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_export_default_decl(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ExportDefaultDecl) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 31)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "decl")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_default_decl(w, f, &node.decl)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_export_default_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ExportDefaultExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 32)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "expr")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.expr)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_export_default_specifier(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ExportDefaultSpecifier) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 33)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "exported")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.exported)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_export_named_specifier(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ExportNamedSpecifier) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 34)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "orig")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.orig)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "exported")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.exported {
    Some(value) => {
      serialize_ident(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_export_namespace_specifier(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ExportNamespaceSpecifier) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 35)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "name")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.name)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_expr_or_spread(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ExprOrSpread) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 36)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "spread")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.spread {
    Some(value) => {
      write!(w, "{}", to_json_string(value)?)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "expr")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.expr)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_expr_stmt(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ExprStmt) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 37)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "expr")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.expr)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_fn_decl(w: &mut impl Write, f: &mut impl JsonFormatter, node: &FnDecl) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 38)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "ident")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.ident)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "declare")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.declare)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "function")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_function(w, f, &node.function)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_fn_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &FnExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 39)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "ident")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.ident {
    Some(value) => {
      serialize_ident(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "function")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_function(w, f, &node.function)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_for_in_stmt(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ForInStmt) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 40)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "left")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_var_decl_or_pat(w, f, &node.left)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "right")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.right)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "body")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_stmt(w, f, &node.body)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_for_of_stmt(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ForOfStmt) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 41)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "awaitToken")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.await_token {
    Some(value) => {
      write!(w, "{}", to_json_string(value)?)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "left")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_var_decl_or_pat(w, f, &node.left)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "right")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.right)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "body")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_stmt(w, f, &node.body)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_for_stmt(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ForStmt) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 42)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "init")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.init {
    Some(value) => {
      serialize_var_decl_or_expr(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "test")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.test {
    Some(value) => {
      serialize_expr(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "update")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.update {
    Some(value) => {
      serialize_expr(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "body")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_stmt(w, f, &node.body)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_function(w: &mut impl Write, f: &mut impl JsonFormatter, node: &Function) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 43)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "params")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.params.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_param(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "decorators")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.decorators.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_decorator(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "body")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.body {
    Some(value) => {
      serialize_block_stmt(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "isGenerator")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.is_generator)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "isAsync")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.is_async)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeParams")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_params {
    Some(value) => {
      serialize_ts_type_param_decl(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "returnType")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.return_type {
    Some(value) => {
      serialize_ts_type_ann(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_getter_prop(w: &mut impl Write, f: &mut impl JsonFormatter, node: &GetterProp) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 44)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "key")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_prop_name(w, f, &node.key)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_ann {
    Some(value) => {
      serialize_ts_type_ann(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "body")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.body {
    Some(value) => {
      serialize_block_stmt(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ident(w: &mut impl Write, f: &mut impl JsonFormatter, node: &Ident) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 45)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "sym")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.sym)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "optional")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.optional)?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_if_stmt(w: &mut impl Write, f: &mut impl JsonFormatter, node: &IfStmt) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 46)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "test")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.test)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "cons")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_stmt(w, f, &node.cons)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "alt")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.alt {
    Some(value) => {
      serialize_stmt(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_import_decl(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ImportDecl) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 47)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "specifiers")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.specifiers.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_import_specifier(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "src")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_str(w, f, &node.src)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeOnly")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.type_only)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "asserts")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.asserts {
    Some(value) => {
      serialize_object_lit(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_import_default_specifier(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ImportDefaultSpecifier) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 48)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "local")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.local)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_import_named_specifier(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ImportNamedSpecifier) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 49)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "local")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.local)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "imported")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.imported {
    Some(value) => {
      serialize_ident(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_import_star_as_specifier(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ImportStarAsSpecifier) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 50)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "local")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.local)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_invalid(w: &mut impl Write, f: &mut impl JsonFormatter, node: &Invalid) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 51)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_jsxattr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &JSXAttr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 52)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "name")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_jsxattr_name(w, f, &node.name)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "value")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.value {
    Some(value) => {
      serialize_jsxattr_value(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_jsxclosing_element(w: &mut impl Write, f: &mut impl JsonFormatter, node: &JSXClosingElement) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 53)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "name")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_jsxelement_name(w, f, &node.name)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_jsxclosing_fragment(w: &mut impl Write, f: &mut impl JsonFormatter, node: &JSXClosingFragment) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 54)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_jsxelement(w: &mut impl Write, f: &mut impl JsonFormatter, node: &JSXElement) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 55)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "opening")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_jsxopening_element(w, f, &node.opening)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "children")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.children.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_jsxelement_child(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "closing")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.closing {
    Some(value) => {
      serialize_jsxclosing_element(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_jsxempty_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &JSXEmptyExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 56)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_jsxexpr_container(w: &mut impl Write, f: &mut impl JsonFormatter, node: &JSXExprContainer) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 57)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "expr")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_jsxexpr(w, f, &node.expr)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_jsxfragment(w: &mut impl Write, f: &mut impl JsonFormatter, node: &JSXFragment) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 58)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "opening")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_jsxopening_fragment(w, f, &node.opening)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "children")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.children.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_jsxelement_child(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "closing")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_jsxclosing_fragment(w, f, &node.closing)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_jsxmember_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &JSXMemberExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 59)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "obj")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_jsxobject(w, f, &node.obj)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "prop")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.prop)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_jsxnamespaced_name(w: &mut impl Write, f: &mut impl JsonFormatter, node: &JSXNamespacedName) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 60)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "ns")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.ns)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "name")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.name)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_jsxopening_element(w: &mut impl Write, f: &mut impl JsonFormatter, node: &JSXOpeningElement) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 61)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "name")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_jsxelement_name(w, f, &node.name)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "attrs")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.attrs.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_jsxattr_or_spread(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "selfClosing")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.self_closing)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeArgs")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_args {
    Some(value) => {
      serialize_ts_type_param_instantiation(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_jsxopening_fragment(w: &mut impl Write, f: &mut impl JsonFormatter, node: &JSXOpeningFragment) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 62)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_jsxspread_child(w: &mut impl Write, f: &mut impl JsonFormatter, node: &JSXSpreadChild) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 63)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "expr")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.expr)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_jsxtext(w: &mut impl Write, f: &mut impl JsonFormatter, node: &JSXText) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 64)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "value")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.value)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "raw")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.raw)?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_key_value_pat_prop(w: &mut impl Write, f: &mut impl JsonFormatter, node: &KeyValuePatProp) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 65)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "key")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_prop_name(w, f, &node.key)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "value")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_pat(w, f, &node.value)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_key_value_prop(w: &mut impl Write, f: &mut impl JsonFormatter, node: &KeyValueProp) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 66)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "key")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_prop_name(w, f, &node.key)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "value")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.value)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_labeled_stmt(w: &mut impl Write, f: &mut impl JsonFormatter, node: &LabeledStmt) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 67)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "label")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.label)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "body")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_stmt(w, f, &node.body)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_member_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &MemberExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 68)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "obj")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr_or_super(w, f, &node.obj)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "prop")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.prop)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "computed")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.computed)?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_meta_prop_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &MetaPropExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 69)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "meta")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.meta)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "prop")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.prop)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_method_prop(w: &mut impl Write, f: &mut impl JsonFormatter, node: &MethodProp) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 70)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "key")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_prop_name(w, f, &node.key)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "function")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_function(w, f, &node.function)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_module(w: &mut impl Write, f: &mut impl JsonFormatter, node: &Module) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 71)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "body")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.body.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_module_item(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "shebang")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.shebang {
    Some(value) => {
      write!(w, "{}", to_json_string(value)?)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_named_export(w: &mut impl Write, f: &mut impl JsonFormatter, node: &NamedExport) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 72)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "specifiers")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.specifiers.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_export_specifier(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "src")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.src {
    Some(value) => {
      serialize_str(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeOnly")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.type_only)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "asserts")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.asserts {
    Some(value) => {
      serialize_object_lit(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_new_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &NewExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 73)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "callee")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.callee)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "args")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.args {
    Some(value) => {
      f.begin_array(w)?;
      for (i, item) in value.iter().enumerate() {
        f.begin_array_value(w, i == 0)?;
        serialize_expr_or_spread(w, f, item)?;
        f.end_array_value(w)?;
      }
      f.end_array(w)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeArgs")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_args {
    Some(value) => {
      serialize_ts_type_param_instantiation(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_null(w: &mut impl Write, f: &mut impl JsonFormatter, node: &Null) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 74)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_number(w: &mut impl Write, f: &mut impl JsonFormatter, node: &Number) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 75)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "value")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.value)?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_object_lit(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ObjectLit) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 76)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "props")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.props.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_prop_or_spread(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_object_pat(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ObjectPat) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 77)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "props")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.props.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_object_pat_prop(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "optional")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.optional)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_ann {
    Some(value) => {
      serialize_ts_type_ann(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_opt_chain_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &OptChainExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 78)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "questionDotToken")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.question_dot_token)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "expr")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.expr)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_param(w: &mut impl Write, f: &mut impl JsonFormatter, node: &Param) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 79)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "decorators")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.decorators.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_decorator(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "pat")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_pat(w, f, &node.pat)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_paren_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ParenExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 80)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "expr")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.expr)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_private_method(w: &mut impl Write, f: &mut impl JsonFormatter, node: &PrivateMethod) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 81)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "key")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_private_name(w, f, &node.key)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "function")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_function(w, f, &node.function)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "methodKind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.kind)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "isStatic")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.is_static)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "accessibility")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.accessibility {
    Some(value) => {
      write!(w, "{}", to_json_string(value)?)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "isAbstract")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.is_abstract)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "isOptional")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.is_optional)?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_private_name(w: &mut impl Write, f: &mut impl JsonFormatter, node: &PrivateName) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 82)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "id")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.id)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_private_prop(w: &mut impl Write, f: &mut impl JsonFormatter, node: &PrivateProp) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 83)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "key")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_private_name(w, f, &node.key)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "value")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.value {
    Some(value) => {
      serialize_expr(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_ann {
    Some(value) => {
      serialize_ts_type_ann(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "isStatic")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.is_static)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "decorators")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.decorators.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_decorator(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "computed")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.computed)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "accessibility")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.accessibility {
    Some(value) => {
      write!(w, "{}", to_json_string(value)?)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "isAbstract")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.is_abstract)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "isOptional")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.is_optional)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "readonly")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.readonly)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "definite")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.definite)?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_regex(w: &mut impl Write, f: &mut impl JsonFormatter, node: &Regex) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 84)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "exp")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.exp)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "flags")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.flags)?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_rest_pat(w: &mut impl Write, f: &mut impl JsonFormatter, node: &RestPat) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 85)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "dot3Token")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.dot3_token)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "arg")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_pat(w, f, &node.arg)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_ann {
    Some(value) => {
      serialize_ts_type_ann(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_return_stmt(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ReturnStmt) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 86)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "arg")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.arg {
    Some(value) => {
      serialize_expr(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_script(w: &mut impl Write, f: &mut impl JsonFormatter, node: &Script) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 87)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "body")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.body.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_stmt(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "shebang")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.shebang {
    Some(value) => {
      write!(w, "{}", to_json_string(value)?)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_seq_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &SeqExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 88)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "exprs")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.exprs.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_expr(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_setter_prop(w: &mut impl Write, f: &mut impl JsonFormatter, node: &SetterProp) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 89)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "key")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_prop_name(w, f, &node.key)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "param")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_pat(w, f, &node.param)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "body")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.body {
    Some(value) => {
      serialize_block_stmt(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_spread_element(w: &mut impl Write, f: &mut impl JsonFormatter, node: &SpreadElement) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 90)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "dot3Token")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.dot3_token)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "expr")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.expr)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_str(w: &mut impl Write, f: &mut impl JsonFormatter, node: &Str) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 91)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "value")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.value)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "hasEscape")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.has_escape)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "strKind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.kind)?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_super(w: &mut impl Write, f: &mut impl JsonFormatter, node: &Super) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 92)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_switch_case(w: &mut impl Write, f: &mut impl JsonFormatter, node: &SwitchCase) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 93)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "test")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.test {
    Some(value) => {
      serialize_expr(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "cons")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.cons.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_stmt(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_switch_stmt(w: &mut impl Write, f: &mut impl JsonFormatter, node: &SwitchStmt) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 94)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "discriminant")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.discriminant)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "cases")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.cases.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_switch_case(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_tagged_tpl(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TaggedTpl) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 95)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "tag")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.tag)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "exprs")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.exprs.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_expr(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "quasis")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.quasis.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_tpl_element(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeParams")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_params {
    Some(value) => {
      serialize_ts_type_param_instantiation(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_this_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ThisExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 96)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_throw_stmt(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ThrowStmt) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 97)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "arg")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.arg)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_tpl(w: &mut impl Write, f: &mut impl JsonFormatter, node: &Tpl) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 98)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "exprs")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.exprs.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_expr(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "quasis")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.quasis.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_tpl_element(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_tpl_element(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TplElement) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 99)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "tail")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.tail)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "cooked")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.cooked {
    Some(value) => {
      serialize_str(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "raw")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_str(w, f, &node.raw)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_try_stmt(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TryStmt) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 100)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "block")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_block_stmt(w, f, &node.block)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "handler")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.handler {
    Some(value) => {
      serialize_catch_clause(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "finalizer")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.finalizer {
    Some(value) => {
      serialize_block_stmt(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_array_type(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsArrayType) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 101)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "elemType")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_type(w, f, &node.elem_type)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_as_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsAsExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 102)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "expr")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.expr)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_type(w, f, &node.type_ann)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_call_signature_decl(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsCallSignatureDecl) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 103)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "params")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.params.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_ts_fn_param(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_ann {
    Some(value) => {
      serialize_ts_type_ann(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeParams")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_params {
    Some(value) => {
      serialize_ts_type_param_decl(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_conditional_type(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsConditionalType) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 104)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "checkType")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_type(w, f, &node.check_type)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "extendsType")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_type(w, f, &node.extends_type)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "trueType")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_type(w, f, &node.true_type)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "falseType")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_type(w, f, &node.false_type)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_const_assertion(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsConstAssertion) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 105)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "expr")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.expr)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_construct_signature_decl(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsConstructSignatureDecl) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 106)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "params")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.params.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_ts_fn_param(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_ann {
    Some(value) => {
      serialize_ts_type_ann(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeParams")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_params {
    Some(value) => {
      serialize_ts_type_param_decl(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_constructor_type(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsConstructorType) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 107)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "params")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.params.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_ts_fn_param(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeParams")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_params {
    Some(value) => {
      serialize_ts_type_param_decl(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_type_ann(w, f, &node.type_ann)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "isAbstract")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.is_abstract)?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_enum_decl(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsEnumDecl) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 108)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "declare")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.declare)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "isConst")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.is_const)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "id")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.id)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "members")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.members.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_ts_enum_member(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_enum_member(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsEnumMember) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 109)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "id")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_enum_member_id(w, f, &node.id)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "init")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.init {
    Some(value) => {
      serialize_expr(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_export_assignment(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsExportAssignment) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 110)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "expr")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.expr)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_expr_with_type_args(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsExprWithTypeArgs) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 111)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "expr")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_entity_name(w, f, &node.expr)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeArgs")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_args {
    Some(value) => {
      serialize_ts_type_param_instantiation(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_external_module_ref(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsExternalModuleRef) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 112)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "expr")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_str(w, f, &node.expr)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_fn_type(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsFnType) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 113)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "params")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.params.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_ts_fn_param(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeParams")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_params {
    Some(value) => {
      serialize_ts_type_param_decl(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_type_ann(w, f, &node.type_ann)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_import_equals_decl(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsImportEqualsDecl) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 114)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "declare")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.declare)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "isExport")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.is_export)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "id")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.id)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "moduleRef")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_module_ref(w, f, &node.module_ref)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_import_type(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsImportType) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 115)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "arg")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_str(w, f, &node.arg)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "qualifier")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.qualifier {
    Some(value) => {
      serialize_ts_entity_name(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeArgs")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_args {
    Some(value) => {
      serialize_ts_type_param_instantiation(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_index_signature(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsIndexSignature) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 116)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "params")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.params.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_ts_fn_param(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_ann {
    Some(value) => {
      serialize_ts_type_ann(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "readonly")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.readonly)?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_indexed_access_type(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsIndexedAccessType) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 117)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "readonly")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.readonly)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "objType")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_type(w, f, &node.obj_type)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "indexType")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_type(w, f, &node.index_type)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_infer_type(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsInferType) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 118)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeParam")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_type_param(w, f, &node.type_param)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_interface_body(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsInterfaceBody) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 119)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "body")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.body.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_ts_type_element(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_interface_decl(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsInterfaceDecl) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 120)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "id")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.id)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "declare")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.declare)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeParams")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_params {
    Some(value) => {
      serialize_ts_type_param_decl(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "extends")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.extends.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_ts_expr_with_type_args(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "body")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_interface_body(w, f, &node.body)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_intersection_type(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsIntersectionType) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 121)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "types")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.types.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_ts_type(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_keyword_type(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsKeywordType) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 122)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "keywordKind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.kind)?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_lit_type(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsLitType) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 123)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "lit")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_lit(w, f, &node.lit)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_mapped_type(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsMappedType) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 124)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "readonly")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.readonly {
    Some(value) => {
      write!(w, "{}", to_json_string(value)?)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeParam")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_type_param(w, f, &node.type_param)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "nameType")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.name_type {
    Some(value) => {
      serialize_ts_type(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "optional")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.optional {
    Some(value) => {
      write!(w, "{}", to_json_string(value)?)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_ann {
    Some(value) => {
      serialize_ts_type(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_method_signature(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsMethodSignature) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 125)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "readonly")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.readonly)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "key")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.key)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "computed")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.computed)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "optional")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.optional)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "params")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.params.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_ts_fn_param(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_ann {
    Some(value) => {
      serialize_ts_type_ann(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeParams")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_params {
    Some(value) => {
      serialize_ts_type_param_decl(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_module_block(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsModuleBlock) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 126)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "body")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.body.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_module_item(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_module_decl(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsModuleDecl) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 127)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "declare")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.declare)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "global")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.global)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "id")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_module_name(w, f, &node.id)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "body")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.body {
    Some(value) => {
      serialize_ts_namespace_body(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_namespace_decl(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsNamespaceDecl) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 128)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "declare")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.declare)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "global")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.global)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "id")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.id)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "body")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_namespace_body(w, f, &node.body)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_namespace_export_decl(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsNamespaceExportDecl) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 129)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "id")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.id)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_non_null_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsNonNullExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 130)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "expr")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.expr)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_optional_type(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsOptionalType) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 131)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_type(w, f, &node.type_ann)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_param_prop(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsParamProp) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 132)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "decorators")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.decorators.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_decorator(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "accessibility")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.accessibility {
    Some(value) => {
      write!(w, "{}", to_json_string(value)?)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "readonly")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.readonly)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "param")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_param_prop_param(w, f, &node.param)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_parenthesized_type(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsParenthesizedType) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 133)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_type(w, f, &node.type_ann)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_property_signature(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsPropertySignature) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 134)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "readonly")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.readonly)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "key")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.key)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "computed")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.computed)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "optional")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.optional)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "init")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.init {
    Some(value) => {
      serialize_expr(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "params")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.params.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_ts_fn_param(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_ann {
    Some(value) => {
      serialize_ts_type_ann(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeParams")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_params {
    Some(value) => {
      serialize_ts_type_param_decl(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_qualified_name(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsQualifiedName) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 135)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "left")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_entity_name(w, f, &node.left)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "right")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.right)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_rest_type(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsRestType) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 136)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_type(w, f, &node.type_ann)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_this_type(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsThisType) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 137)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_tpl_lit_type(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsTplLitType) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 138)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "types")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.types.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_ts_type(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "quasis")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.quasis.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_tpl_element(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_tuple_element(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsTupleElement) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 139)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "label")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.label {
    Some(value) => {
      serialize_pat(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "ty")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_type(w, f, &node.ty)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_tuple_type(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsTupleType) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 140)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "elemTypes")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.elem_types.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_ts_tuple_element(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_type_alias_decl(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsTypeAliasDecl) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 141)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "declare")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.declare)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "id")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.id)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeParams")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_params {
    Some(value) => {
      serialize_ts_type_param_decl(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_type(w, f, &node.type_ann)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_type_ann(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsTypeAnn) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 142)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_type(w, f, &node.type_ann)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_type_assertion(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsTypeAssertion) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 143)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "expr")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.expr)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_type(w, f, &node.type_ann)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_type_lit(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsTypeLit) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 144)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "members")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.members.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_ts_type_element(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_type_operator(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsTypeOperator) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 145)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "op")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.op)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_type(w, f, &node.type_ann)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_type_param(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsTypeParam) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 146)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "name")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ident(w, f, &node.name)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "constraint")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.constraint {
    Some(value) => {
      serialize_ts_type(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "default")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.default {
    Some(value) => {
      serialize_ts_type(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_type_param_decl(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsTypeParamDecl) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 147)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "params")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.params.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_ts_type_param(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_type_param_instantiation(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsTypeParamInstantiation) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 148)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "params")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.params.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_ts_type(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_type_predicate(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsTypePredicate) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 149)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "asserts")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.asserts)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "paramName")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_this_type_or_ident(w, f, &node.param_name)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeAnn")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_ann {
    Some(value) => {
      serialize_ts_type_ann(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_type_query(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsTypeQuery) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 150)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "exprName")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_type_query_expr(w, f, &node.expr_name)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_type_ref(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsTypeRef) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 151)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeName")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_ts_entity_name(w, f, &node.type_name)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "typeParams")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.type_params {
    Some(value) => {
      serialize_ts_type_param_instantiation(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_ts_union_type(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsUnionType) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 152)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "types")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.types.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_ts_type(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_unary_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &UnaryExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 153)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "op")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.op)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "arg")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.arg)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_update_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &UpdateExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 154)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "op")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.op)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "prefix")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.prefix)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "arg")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.arg)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_var_decl(w: &mut impl Write, f: &mut impl JsonFormatter, node: &VarDecl) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 155)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "declKind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.kind)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "declare")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.declare)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "decls")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_array(w)?;
  for (i, item) in node.decls.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_var_declarator(w, f, item)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_var_declarator(w: &mut impl Write, f: &mut impl JsonFormatter, node: &VarDeclarator) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 156)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "name")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_pat(w, f, &node.name)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "init")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.init {
    Some(value) => {
      serialize_expr(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "definite")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.definite)?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_while_stmt(w: &mut impl Write, f: &mut impl JsonFormatter, node: &WhileStmt) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 157)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "test")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.test)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "body")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_stmt(w, f, &node.body)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_with_stmt(w: &mut impl Write, f: &mut impl JsonFormatter, node: &WithStmt) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 158)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "obj")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_expr(w, f, &node.obj)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "body")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_stmt(w, f, &node.body)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_yield_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &YieldExpr) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, 159)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.span())?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "arg")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  match &node.arg {
    Some(value) => {
      serialize_expr(w, f, value)?;
    }
    None => f.write_null(w)?,
  }
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "delegate")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&node.delegate)?)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_block_stmt_or_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &BlockStmtOrExpr) -> Result<(), Error> {
  match node {
    BlockStmtOrExpr::BlockStmt(node) => serialize_block_stmt(w, f, node)?,
    BlockStmtOrExpr::Expr(node) => serialize_expr(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_class_member(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ClassMember) -> Result<(), Error> {
  match node {
    ClassMember::Constructor(node) => serialize_constructor(w, f, node)?,
    ClassMember::Method(node) => serialize_class_method(w, f, node)?,
    ClassMember::PrivateMethod(node) => serialize_private_method(w, f, node)?,
    ClassMember::ClassProp(node) => serialize_class_prop(w, f, node)?,
    ClassMember::PrivateProp(node) => serialize_private_prop(w, f, node)?,
    ClassMember::TsIndexSignature(node) => serialize_ts_index_signature(w, f, node)?,
    ClassMember::Empty(node) => serialize_empty_stmt(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_decl(w: &mut impl Write, f: &mut impl JsonFormatter, node: &Decl) -> Result<(), Error> {
  match node {
    Decl::Class(node) => serialize_class_decl(w, f, node)?,
    Decl::Fn(node) => serialize_fn_decl(w, f, node)?,
    Decl::Var(node) => serialize_var_decl(w, f, node)?,
    Decl::TsInterface(node) => serialize_ts_interface_decl(w, f, node)?,
    Decl::TsTypeAlias(node) => serialize_ts_type_alias_decl(w, f, node)?,
    Decl::TsEnum(node) => serialize_ts_enum_decl(w, f, node)?,
    Decl::TsModule(node) => serialize_ts_module_decl(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_default_decl(w: &mut impl Write, f: &mut impl JsonFormatter, node: &DefaultDecl) -> Result<(), Error> {
  match node {
    DefaultDecl::Class(node) => serialize_class_expr(w, f, node)?,
    DefaultDecl::Fn(node) => serialize_fn_expr(w, f, node)?,
    DefaultDecl::TsInterfaceDecl(node) => serialize_ts_interface_decl(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_export_specifier(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ExportSpecifier) -> Result<(), Error> {
  match node {
    ExportSpecifier::Namespace(node) => serialize_export_namespace_specifier(w, f, node)?,
    ExportSpecifier::Default(node) => serialize_export_default_specifier(w, f, node)?,
    ExportSpecifier::Named(node) => serialize_export_named_specifier(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &Expr) -> Result<(), Error> {
  match node {
    Expr::This(node) => serialize_this_expr(w, f, node)?,
    Expr::Array(node) => serialize_array_lit(w, f, node)?,
    Expr::Object(node) => serialize_object_lit(w, f, node)?,
    Expr::Fn(node) => serialize_fn_expr(w, f, node)?,
    Expr::Unary(node) => serialize_unary_expr(w, f, node)?,
    Expr::Update(node) => serialize_update_expr(w, f, node)?,
    Expr::Bin(node) => serialize_bin_expr(w, f, node)?,
    Expr::Assign(node) => serialize_assign_expr(w, f, node)?,
    Expr::Member(node) => serialize_member_expr(w, f, node)?,
    Expr::Cond(node) => serialize_cond_expr(w, f, node)?,
    Expr::Call(node) => serialize_call_expr(w, f, node)?,
    Expr::New(node) => serialize_new_expr(w, f, node)?,
    Expr::Seq(node) => serialize_seq_expr(w, f, node)?,
    Expr::Ident(node) => serialize_ident(w, f, node)?,
    Expr::Lit(node) => serialize_lit(w, f, node)?,
    Expr::Tpl(node) => serialize_tpl(w, f, node)?,
    Expr::TaggedTpl(node) => serialize_tagged_tpl(w, f, node)?,
    Expr::Arrow(node) => serialize_arrow_expr(w, f, node)?,
    Expr::Class(node) => serialize_class_expr(w, f, node)?,
    Expr::Yield(node) => serialize_yield_expr(w, f, node)?,
    Expr::MetaProp(node) => serialize_meta_prop_expr(w, f, node)?,
    Expr::Await(node) => serialize_await_expr(w, f, node)?,
    Expr::Paren(node) => serialize_paren_expr(w, f, node)?,
    Expr::JSXMember(node) => serialize_jsxmember_expr(w, f, node)?,
    Expr::JSXNamespacedName(node) => serialize_jsxnamespaced_name(w, f, node)?,
    Expr::JSXEmpty(node) => serialize_jsxempty_expr(w, f, node)?,
    Expr::JSXElement(node) => serialize_jsxelement(w, f, node)?,
    Expr::JSXFragment(node) => serialize_jsxfragment(w, f, node)?,
    Expr::TsTypeAssertion(node) => serialize_ts_type_assertion(w, f, node)?,
    Expr::TsConstAssertion(node) => serialize_ts_const_assertion(w, f, node)?,
    Expr::TsNonNull(node) => serialize_ts_non_null_expr(w, f, node)?,
    Expr::TsAs(node) => serialize_ts_as_expr(w, f, node)?,
    Expr::PrivateName(node) => serialize_private_name(w, f, node)?,
    Expr::OptChain(node) => serialize_opt_chain_expr(w, f, node)?,
    Expr::Invalid(node) => serialize_invalid(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_expr_or_super(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ExprOrSuper) -> Result<(), Error> {
  match node {
    ExprOrSuper::Super(node) => serialize_super(w, f, node)?,
    ExprOrSuper::Expr(node) => serialize_expr(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_import_specifier(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ImportSpecifier) -> Result<(), Error> {
  match node {
    ImportSpecifier::Named(node) => serialize_import_named_specifier(w, f, node)?,
    ImportSpecifier::Default(node) => serialize_import_default_specifier(w, f, node)?,
    ImportSpecifier::Namespace(node) => serialize_import_star_as_specifier(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_jsxattr_name(w: &mut impl Write, f: &mut impl JsonFormatter, node: &JSXAttrName) -> Result<(), Error> {
  match node {
    JSXAttrName::Ident(node) => serialize_ident(w, f, node)?,
    JSXAttrName::JSXNamespacedName(node) => serialize_jsxnamespaced_name(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_jsxattr_or_spread(w: &mut impl Write, f: &mut impl JsonFormatter, node: &JSXAttrOrSpread) -> Result<(), Error> {
  match node {
    JSXAttrOrSpread::JSXAttr(node) => serialize_jsxattr(w, f, node)?,
    JSXAttrOrSpread::SpreadElement(node) => serialize_spread_element(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_jsxattr_value(w: &mut impl Write, f: &mut impl JsonFormatter, node: &JSXAttrValue) -> Result<(), Error> {
  match node {
    JSXAttrValue::Lit(node) => serialize_lit(w, f, node)?,
    JSXAttrValue::JSXExprContainer(node) => serialize_jsxexpr_container(w, f, node)?,
    JSXAttrValue::JSXElement(node) => serialize_jsxelement(w, f, node)?,
    JSXAttrValue::JSXFragment(node) => serialize_jsxfragment(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_jsxelement_child(w: &mut impl Write, f: &mut impl JsonFormatter, node: &JSXElementChild) -> Result<(), Error> {
  match node {
    JSXElementChild::JSXText(node) => serialize_jsxtext(w, f, node)?,
    JSXElementChild::JSXExprContainer(node) => serialize_jsxexpr_container(w, f, node)?,
    JSXElementChild::JSXSpreadChild(node) => serialize_jsxspread_child(w, f, node)?,
    JSXElementChild::JSXElement(node) => serialize_jsxelement(w, f, node)?,
    JSXElementChild::JSXFragment(node) => serialize_jsxfragment(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_jsxelement_name(w: &mut impl Write, f: &mut impl JsonFormatter, node: &JSXElementName) -> Result<(), Error> {
  match node {
    JSXElementName::Ident(node) => serialize_ident(w, f, node)?,
    JSXElementName::JSXMemberExpr(node) => serialize_jsxmember_expr(w, f, node)?,
    JSXElementName::JSXNamespacedName(node) => serialize_jsxnamespaced_name(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_jsxexpr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &JSXExpr) -> Result<(), Error> {
  match node {
    JSXExpr::JSXEmptyExpr(node) => serialize_jsxempty_expr(w, f, node)?,
    JSXExpr::Expr(node) => serialize_expr(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_jsxobject(w: &mut impl Write, f: &mut impl JsonFormatter, node: &JSXObject) -> Result<(), Error> {
  match node {
    JSXObject::JSXMemberExpr(node) => serialize_jsxmember_expr(w, f, node)?,
    JSXObject::Ident(node) => serialize_ident(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_lit(w: &mut impl Write, f: &mut impl JsonFormatter, node: &Lit) -> Result<(), Error> {
  match node {
    Lit::Str(node) => serialize_str(w, f, node)?,
    Lit::Bool(node) => serialize_bool(w, f, node)?,
    Lit::Null(node) => serialize_null(w, f, node)?,
    Lit::Num(node) => serialize_number(w, f, node)?,
    Lit::BigInt(node) => serialize_big_int(w, f, node)?,
    Lit::Regex(node) => serialize_regex(w, f, node)?,
    Lit::JSXText(node) => serialize_jsxtext(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_module_decl(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ModuleDecl) -> Result<(), Error> {
  match node {
    ModuleDecl::Import(node) => serialize_import_decl(w, f, node)?,
    ModuleDecl::ExportDecl(node) => serialize_export_decl(w, f, node)?,
    ModuleDecl::ExportNamed(node) => serialize_named_export(w, f, node)?,
    ModuleDecl::ExportDefaultDecl(node) => serialize_export_default_decl(w, f, node)?,
    ModuleDecl::ExportDefaultExpr(node) => serialize_export_default_expr(w, f, node)?,
    ModuleDecl::ExportAll(node) => serialize_export_all(w, f, node)?,
    ModuleDecl::TsImportEquals(node) => serialize_ts_import_equals_decl(w, f, node)?,
    ModuleDecl::TsExportAssignment(node) => serialize_ts_export_assignment(w, f, node)?,
    ModuleDecl::TsNamespaceExport(node) => serialize_ts_namespace_export_decl(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_module_item(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ModuleItem) -> Result<(), Error> {
  match node {
    ModuleItem::ModuleDecl(node) => serialize_module_decl(w, f, node)?,
    ModuleItem::Stmt(node) => serialize_stmt(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_object_pat_prop(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ObjectPatProp) -> Result<(), Error> {
  match node {
    ObjectPatProp::KeyValue(node) => serialize_key_value_pat_prop(w, f, node)?,
    ObjectPatProp::Assign(node) => serialize_assign_pat_prop(w, f, node)?,
    ObjectPatProp::Rest(node) => serialize_rest_pat(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_param_or_ts_param_prop(w: &mut impl Write, f: &mut impl JsonFormatter, node: &ParamOrTsParamProp) -> Result<(), Error> {
  match node {
    ParamOrTsParamProp::TsParamProp(node) => serialize_ts_param_prop(w, f, node)?,
    ParamOrTsParamProp::Param(node) => serialize_param(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_pat(w: &mut impl Write, f: &mut impl JsonFormatter, node: &Pat) -> Result<(), Error> {
  match node {
    Pat::Ident(node) => serialize_binding_ident(w, f, node)?,
    Pat::Array(node) => serialize_array_pat(w, f, node)?,
    Pat::Rest(node) => serialize_rest_pat(w, f, node)?,
    Pat::Object(node) => serialize_object_pat(w, f, node)?,
    Pat::Assign(node) => serialize_assign_pat(w, f, node)?,
    Pat::Invalid(node) => serialize_invalid(w, f, node)?,
    Pat::Expr(node) => serialize_expr(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_pat_or_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &PatOrExpr) -> Result<(), Error> {
  match node {
    PatOrExpr::Expr(node) => serialize_expr(w, f, node)?,
    PatOrExpr::Pat(node) => serialize_pat(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_prop(w: &mut impl Write, f: &mut impl JsonFormatter, node: &Prop) -> Result<(), Error> {
  match node {
    Prop::Shorthand(node) => serialize_ident(w, f, node)?,
    Prop::KeyValue(node) => serialize_key_value_prop(w, f, node)?,
    Prop::Assign(node) => serialize_assign_prop(w, f, node)?,
    Prop::Getter(node) => serialize_getter_prop(w, f, node)?,
    Prop::Setter(node) => serialize_setter_prop(w, f, node)?,
    Prop::Method(node) => serialize_method_prop(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_prop_name(w: &mut impl Write, f: &mut impl JsonFormatter, node: &PropName) -> Result<(), Error> {
  match node {
    PropName::Ident(node) => serialize_ident(w, f, node)?,
    PropName::Str(node) => serialize_str(w, f, node)?,
    PropName::Num(node) => serialize_number(w, f, node)?,
    PropName::Computed(node) => serialize_computed_prop_name(w, f, node)?,
    PropName::BigInt(node) => serialize_big_int(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_prop_or_spread(w: &mut impl Write, f: &mut impl JsonFormatter, node: &PropOrSpread) -> Result<(), Error> {
  match node {
    PropOrSpread::Spread(node) => serialize_spread_element(w, f, node)?,
    PropOrSpread::Prop(node) => serialize_prop(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_stmt(w: &mut impl Write, f: &mut impl JsonFormatter, node: &Stmt) -> Result<(), Error> {
  match node {
    Stmt::Block(node) => serialize_block_stmt(w, f, node)?,
    Stmt::Empty(node) => serialize_empty_stmt(w, f, node)?,
    Stmt::Debugger(node) => serialize_debugger_stmt(w, f, node)?,
    Stmt::With(node) => serialize_with_stmt(w, f, node)?,
    Stmt::Return(node) => serialize_return_stmt(w, f, node)?,
    Stmt::Labeled(node) => serialize_labeled_stmt(w, f, node)?,
    Stmt::Break(node) => serialize_break_stmt(w, f, node)?,
    Stmt::Continue(node) => serialize_continue_stmt(w, f, node)?,
    Stmt::If(node) => serialize_if_stmt(w, f, node)?,
    Stmt::Switch(node) => serialize_switch_stmt(w, f, node)?,
    Stmt::Throw(node) => serialize_throw_stmt(w, f, node)?,
    Stmt::Try(node) => serialize_try_stmt(w, f, node)?,
    Stmt::While(node) => serialize_while_stmt(w, f, node)?,
    Stmt::DoWhile(node) => serialize_do_while_stmt(w, f, node)?,
    Stmt::For(node) => serialize_for_stmt(w, f, node)?,
    Stmt::ForIn(node) => serialize_for_in_stmt(w, f, node)?,
    Stmt::ForOf(node) => serialize_for_of_stmt(w, f, node)?,
    Stmt::Decl(node) => serialize_decl(w, f, node)?,
    Stmt::Expr(node) => serialize_expr_stmt(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_ts_entity_name(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsEntityName) -> Result<(), Error> {
  match node {
    TsEntityName::TsQualifiedName(node) => serialize_ts_qualified_name(w, f, node)?,
    TsEntityName::Ident(node) => serialize_ident(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_ts_enum_member_id(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsEnumMemberId) -> Result<(), Error> {
  match node {
    TsEnumMemberId::Ident(node) => serialize_ident(w, f, node)?,
    TsEnumMemberId::Str(node) => serialize_str(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_ts_fn_or_constructor_type(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsFnOrConstructorType) -> Result<(), Error> {
  match node {
    TsFnOrConstructorType::TsFnType(node) => serialize_ts_fn_type(w, f, node)?,
    TsFnOrConstructorType::TsConstructorType(node) => serialize_ts_constructor_type(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_ts_fn_param(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsFnParam) -> Result<(), Error> {
  match node {
    TsFnParam::Ident(node) => serialize_binding_ident(w, f, node)?,
    TsFnParam::Array(node) => serialize_array_pat(w, f, node)?,
    TsFnParam::Rest(node) => serialize_rest_pat(w, f, node)?,
    TsFnParam::Object(node) => serialize_object_pat(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_ts_lit(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsLit) -> Result<(), Error> {
  match node {
    TsLit::Number(node) => serialize_number(w, f, node)?,
    TsLit::Str(node) => serialize_str(w, f, node)?,
    TsLit::Bool(node) => serialize_bool(w, f, node)?,
    TsLit::BigInt(node) => serialize_big_int(w, f, node)?,
    TsLit::Tpl(node) => serialize_ts_tpl_lit_type(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_ts_module_name(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsModuleName) -> Result<(), Error> {
  match node {
    TsModuleName::Ident(node) => serialize_ident(w, f, node)?,
    TsModuleName::Str(node) => serialize_str(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_ts_module_ref(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsModuleRef) -> Result<(), Error> {
  match node {
    TsModuleRef::TsEntityName(node) => serialize_ts_entity_name(w, f, node)?,
    TsModuleRef::TsExternalModuleRef(node) => serialize_ts_external_module_ref(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_ts_namespace_body(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsNamespaceBody) -> Result<(), Error> {
  match node {
    TsNamespaceBody::TsModuleBlock(node) => serialize_ts_module_block(w, f, node)?,
    TsNamespaceBody::TsNamespaceDecl(node) => serialize_ts_namespace_decl(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_ts_param_prop_param(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsParamPropParam) -> Result<(), Error> {
  match node {
    TsParamPropParam::Ident(node) => serialize_binding_ident(w, f, node)?,
    TsParamPropParam::Assign(node) => serialize_assign_pat(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_ts_this_type_or_ident(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsThisTypeOrIdent) -> Result<(), Error> {
  match node {
    TsThisTypeOrIdent::TsThisType(node) => serialize_ts_this_type(w, f, node)?,
    TsThisTypeOrIdent::Ident(node) => serialize_ident(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_ts_type(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsType) -> Result<(), Error> {
  match node {
    TsType::TsKeywordType(node) => serialize_ts_keyword_type(w, f, node)?,
    TsType::TsThisType(node) => serialize_ts_this_type(w, f, node)?,
    TsType::TsFnOrConstructorType(node) => serialize_ts_fn_or_constructor_type(w, f, node)?,
    TsType::TsTypeRef(node) => serialize_ts_type_ref(w, f, node)?,
    TsType::TsTypeQuery(node) => serialize_ts_type_query(w, f, node)?,
    TsType::TsTypeLit(node) => serialize_ts_type_lit(w, f, node)?,
    TsType::TsArrayType(node) => serialize_ts_array_type(w, f, node)?,
    TsType::TsTupleType(node) => serialize_ts_tuple_type(w, f, node)?,
    TsType::TsOptionalType(node) => serialize_ts_optional_type(w, f, node)?,
    TsType::TsRestType(node) => serialize_ts_rest_type(w, f, node)?,
    TsType::TsUnionOrIntersectionType(node) => serialize_ts_union_or_intersection_type(w, f, node)?,
    TsType::TsConditionalType(node) => serialize_ts_conditional_type(w, f, node)?,
    TsType::TsInferType(node) => serialize_ts_infer_type(w, f, node)?,
    TsType::TsParenthesizedType(node) => serialize_ts_parenthesized_type(w, f, node)?,
    TsType::TsTypeOperator(node) => serialize_ts_type_operator(w, f, node)?,
    TsType::TsIndexedAccessType(node) => serialize_ts_indexed_access_type(w, f, node)?,
    TsType::TsMappedType(node) => serialize_ts_mapped_type(w, f, node)?,
    TsType::TsLitType(node) => serialize_ts_lit_type(w, f, node)?,
    TsType::TsTypePredicate(node) => serialize_ts_type_predicate(w, f, node)?,
    TsType::TsImportType(node) => serialize_ts_import_type(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_ts_type_element(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsTypeElement) -> Result<(), Error> {
  match node {
    TsTypeElement::TsCallSignatureDecl(node) => serialize_ts_call_signature_decl(w, f, node)?,
    TsTypeElement::TsConstructSignatureDecl(node) => serialize_ts_construct_signature_decl(w, f, node)?,
    TsTypeElement::TsPropertySignature(node) => serialize_ts_property_signature(w, f, node)?,
    TsTypeElement::TsMethodSignature(node) => serialize_ts_method_signature(w, f, node)?,
    TsTypeElement::TsIndexSignature(node) => serialize_ts_index_signature(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_ts_type_query_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsTypeQueryExpr) -> Result<(), Error> {
  match node {
    TsTypeQueryExpr::TsEntityName(node) => serialize_ts_entity_name(w, f, node)?,
    TsTypeQueryExpr::Import(node) => serialize_ts_import_type(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_ts_union_or_intersection_type(w: &mut impl Write, f: &mut impl JsonFormatter, node: &TsUnionOrIntersectionType) -> Result<(), Error> {
  match node {
    TsUnionOrIntersectionType::TsUnionType(node) => serialize_ts_union_type(w, f, node)?,
    TsUnionOrIntersectionType::TsIntersectionType(node) => serialize_ts_intersection_type(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_var_decl_or_expr(w: &mut impl Write, f: &mut impl JsonFormatter, node: &VarDeclOrExpr) -> Result<(), Error> {
  match node {
    VarDeclOrExpr::VarDecl(node) => serialize_var_decl(w, f, node)?,
    VarDeclOrExpr::Expr(node) => serialize_expr(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_var_decl_or_pat(w: &mut impl Write, f: &mut impl JsonFormatter, node: &VarDeclOrPat) -> Result<(), Error> {
  match node {
    VarDeclOrPat::VarDecl(node) => serialize_var_decl(w, f, node)?,
    VarDeclOrPat::Pat(node) => serialize_pat(w, f, node)?,
  }
  Ok(())
}

pub fn serialize_token_and_spans(w: &mut impl Write, f: &mut impl JsonFormatter, tokens: &Vec<TokenAndSpan>) -> Result<(), Error> {
  f.begin_array(w)?;
  for (i, token_and_span) in tokens.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_token_and_span(w, f, &token_and_span)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  Ok(())
}

pub fn serialize_token_and_span(w: &mut impl Write, f: &mut impl JsonFormatter, token_and_span: &TokenAndSpan) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&token_and_span.span)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "hadLineBreak")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_bool(w, token_and_span.had_line_break)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "token")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  serialize_token(w, f, &token_and_span.token)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

fn serialize_bin_op_token(w: &mut impl Write, f: &mut impl JsonFormatter, value: &BinOpToken) -> Result<(), Error> {
  match value {
    BinOpToken::EqEq => f.write_u32(w, 0)?,
    BinOpToken::NotEq => f.write_u32(w, 1)?,
    BinOpToken::EqEqEq => f.write_u32(w, 2)?,
    BinOpToken::NotEqEq => f.write_u32(w, 3)?,
    BinOpToken::Lt => f.write_u32(w, 4)?,
    BinOpToken::LtEq => f.write_u32(w, 5)?,
    BinOpToken::Gt => f.write_u32(w, 6)?,
    BinOpToken::GtEq => f.write_u32(w, 7)?,
    BinOpToken::LShift => f.write_u32(w, 8)?,
    BinOpToken::RShift => f.write_u32(w, 9)?,
    BinOpToken::ZeroFillRShift => f.write_u32(w, 10)?,
    BinOpToken::Add => f.write_u32(w, 11)?,
    BinOpToken::Sub => f.write_u32(w, 12)?,
    BinOpToken::Mul => f.write_u32(w, 13)?,
    BinOpToken::Div => f.write_u32(w, 14)?,
    BinOpToken::Mod => f.write_u32(w, 15)?,
    BinOpToken::BitOr => f.write_u32(w, 16)?,
    BinOpToken::BitXor => f.write_u32(w, 17)?,
    BinOpToken::BitAnd => f.write_u32(w, 18)?,
    BinOpToken::Exp => f.write_u32(w, 19)?,
    BinOpToken::LogicalOr => f.write_u32(w, 20)?,
    BinOpToken::LogicalAnd => f.write_u32(w, 21)?,
    BinOpToken::NullishCoalescing => f.write_u32(w, 22)?,
  }
  Ok(())
}

fn serialize_keyword(w: &mut impl Write, f: &mut impl JsonFormatter, value: &Keyword) -> Result<(), Error> {
  match value {
    Keyword::Await => f.write_u32(w, 0)?,
    Keyword::Break => f.write_u32(w, 1)?,
    Keyword::Case => f.write_u32(w, 2)?,
    Keyword::Catch => f.write_u32(w, 3)?,
    Keyword::Continue => f.write_u32(w, 4)?,
    Keyword::Debugger => f.write_u32(w, 5)?,
    Keyword::Default_ => f.write_u32(w, 6)?,
    Keyword::Do => f.write_u32(w, 7)?,
    Keyword::Else => f.write_u32(w, 8)?,
    Keyword::Finally => f.write_u32(w, 9)?,
    Keyword::For => f.write_u32(w, 10)?,
    Keyword::Function => f.write_u32(w, 11)?,
    Keyword::If => f.write_u32(w, 12)?,
    Keyword::Return => f.write_u32(w, 13)?,
    Keyword::Switch => f.write_u32(w, 14)?,
    Keyword::Throw => f.write_u32(w, 15)?,
    Keyword::Try => f.write_u32(w, 16)?,
    Keyword::Var => f.write_u32(w, 17)?,
    Keyword::Let => f.write_u32(w, 18)?,
    Keyword::Const => f.write_u32(w, 19)?,
    Keyword::While => f.write_u32(w, 20)?,
    Keyword::With => f.write_u32(w, 21)?,
    Keyword::New => f.write_u32(w, 22)?,
    Keyword::This => f.write_u32(w, 23)?,
    Keyword::Super => f.write_u32(w, 24)?,
    Keyword::Class => f.write_u32(w, 25)?,
    Keyword::Extends => f.write_u32(w, 26)?,
    Keyword::Export => f.write_u32(w, 27)?,
    Keyword::Import => f.write_u32(w, 28)?,
    Keyword::Yield => f.write_u32(w, 29)?,
    Keyword::In => f.write_u32(w, 30)?,
    Keyword::InstanceOf => f.write_u32(w, 31)?,
    Keyword::TypeOf => f.write_u32(w, 32)?,
    Keyword::Void => f.write_u32(w, 33)?,
    Keyword::Delete => f.write_u32(w, 34)?,
  }
  Ok(())
}

fn serialize_token(w: &mut impl Write, f: &mut impl JsonFormatter, value: &Token) -> Result<(), Error> {
  match value {
    Token::Word(item0) => {
      f.begin_object(w)?;
      f.begin_object_key(w, true)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "kind")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      f.write_u32(w, 0)?;
      f.end_object_value(w)?;
      f.begin_object_key(w, false)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "inner")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      serialize_word(w, f, &item0)?;
      f.end_object_value(w)?;
      f.end_object(w)?;
    }
    Token::Arrow => f.write_u32(w, 1)?,
    Token::Hash => f.write_u32(w, 2)?,
    Token::At => f.write_u32(w, 3)?,
    Token::Dot => f.write_u32(w, 4)?,
    Token::DotDotDot => f.write_u32(w, 5)?,
    Token::Bang => f.write_u32(w, 6)?,
    Token::LParen => f.write_u32(w, 7)?,
    Token::RParen => f.write_u32(w, 8)?,
    Token::LBracket => f.write_u32(w, 9)?,
    Token::RBracket => f.write_u32(w, 10)?,
    Token::LBrace => f.write_u32(w, 11)?,
    Token::RBrace => f.write_u32(w, 12)?,
    Token::Semi => f.write_u32(w, 13)?,
    Token::Comma => f.write_u32(w, 14)?,
    Token::BackQuote => f.write_u32(w, 15)?,
    Token::Template {
      raw,
      cooked,
      has_escape,
    } => {
      f.begin_object(w)?;
      f.begin_object_key(w, true)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "kind")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      f.write_u32(w, 16)?;
      f.end_object_value(w)?;
      f.begin_object_key(w, false)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "raw")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      write!(w, "{}", to_json_string(&raw)?)?;
      f.end_object_value(w)?;
      f.begin_object_key(w, false)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "cooked")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      match &cooked {
        Some(value) => {
          write!(w, "{}", to_json_string(value)?)?;
        }
        None => f.write_null(w)?,
      }
      f.end_object_value(w)?;
      f.begin_object_key(w, false)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "hasEscape")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      write!(w, "{}", to_json_string(&has_escape)?)?;
      f.end_object_value(w)?;
      f.end_object(w)?;
    }
    Token::Colon => f.write_u32(w, 17)?,
    Token::ColonColon => f.write_u32(w, 18)?,
    Token::BinOp(item0) => {
      f.begin_object(w)?;
      f.begin_object_key(w, true)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "kind")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      f.write_u32(w, 19)?;
      f.end_object_value(w)?;
      f.begin_object_key(w, false)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "inner")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      serialize_bin_op_token(w, f, &item0)?;
      f.end_object_value(w)?;
      f.end_object(w)?;
    }
    Token::AssignOp(item0) => {
      f.begin_object(w)?;
      f.begin_object_key(w, true)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "kind")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      f.write_u32(w, 20)?;
      f.end_object_value(w)?;
      f.begin_object_key(w, false)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "inner")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      write!(w, "{}", to_json_string(&item0)?)?;
      f.end_object_value(w)?;
      f.end_object(w)?;
    }
    Token::DollarLBrace => f.write_u32(w, 21)?,
    Token::QuestionMark => f.write_u32(w, 22)?,
    Token::PlusPlus => f.write_u32(w, 23)?,
    Token::MinusMinus => f.write_u32(w, 24)?,
    Token::Tilde => f.write_u32(w, 25)?,
    Token::Str {
      value,
      has_escape,
    } => {
      f.begin_object(w)?;
      f.begin_object_key(w, true)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "kind")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      f.write_u32(w, 26)?;
      f.end_object_value(w)?;
      f.begin_object_key(w, false)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "value")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      write!(w, "{}", to_json_string(&value)?)?;
      f.end_object_value(w)?;
      f.begin_object_key(w, false)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "hasEscape")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      write!(w, "{}", to_json_string(&has_escape)?)?;
      f.end_object_value(w)?;
      f.end_object(w)?;
    }
    Token::Regex(item0, item1) => {
      f.begin_object(w)?;
      f.begin_object_key(w, true)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "kind")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      f.write_u32(w, 27)?;
      f.end_object_value(w)?;
      f.begin_array(w)?;
      f.begin_array_value(w, true)?;
      write!(w, "{}", to_json_string(&item0)?)?;
      f.end_array_value(w)?;
      f.begin_array_value(w, false)?;
      write!(w, "{}", to_json_string(&item1)?)?;
      f.end_array_value(w)?;
      f.end_array(w)?;
      f.end_object(w)?;
    }
    Token::Num(item0) => {
      f.begin_object(w)?;
      f.begin_object_key(w, true)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "kind")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      f.write_u32(w, 28)?;
      f.end_object_value(w)?;
      f.begin_object_key(w, false)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "inner")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      write!(w, "{}", to_json_string(&item0)?)?;
      f.end_object_value(w)?;
      f.end_object(w)?;
    }
    Token::BigInt(item0) => {
      f.begin_object(w)?;
      f.begin_object_key(w, true)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "kind")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      f.write_u32(w, 29)?;
      f.end_object_value(w)?;
      f.begin_object_key(w, false)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "inner")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      write!(w, "{}", to_json_string(&item0)?)?;
      f.end_object_value(w)?;
      f.end_object(w)?;
    }
    Token::JSXName {
      name,
    } => {
      f.begin_object(w)?;
      f.begin_object_key(w, true)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "kind")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      f.write_u32(w, 30)?;
      f.end_object_value(w)?;
      f.begin_object_key(w, false)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "name")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      write!(w, "{}", to_json_string(&name)?)?;
      f.end_object_value(w)?;
      f.end_object(w)?;
    }
    Token::JSXText {
      raw,
    } => {
      f.begin_object(w)?;
      f.begin_object_key(w, true)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "kind")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      f.write_u32(w, 31)?;
      f.end_object_value(w)?;
      f.begin_object_key(w, false)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "raw")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      write!(w, "{}", to_json_string(&raw)?)?;
      f.end_object_value(w)?;
      f.end_object(w)?;
    }
    Token::JSXTagStart => f.write_u32(w, 32)?,
    Token::JSXTagEnd => f.write_u32(w, 33)?,
    Token::Shebang(item0) => {
      f.begin_object(w)?;
      f.begin_object_key(w, true)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "kind")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      f.write_u32(w, 34)?;
      f.end_object_value(w)?;
      f.begin_object_key(w, false)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "inner")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      write!(w, "{}", to_json_string(&item0)?)?;
      f.end_object_value(w)?;
      f.end_object(w)?;
    }
    Token::Error(_) => panic!("Serializing an AST containing an Error is not currently supported."),
  }
  Ok(())
}

fn serialize_word(w: &mut impl Write, f: &mut impl JsonFormatter, value: &Word) -> Result<(), Error> {
  match value {
    Word::Keyword(item0) => {
      f.begin_object(w)?;
      f.begin_object_key(w, true)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "kind")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      f.write_u32(w, 0)?;
      f.end_object_value(w)?;
      f.begin_object_key(w, false)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "inner")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      serialize_keyword(w, f, &item0)?;
      f.end_object_value(w)?;
      f.end_object(w)?;
    }
    Word::Null => f.write_u32(w, 1)?,
    Word::True => f.write_u32(w, 2)?,
    Word::False => f.write_u32(w, 3)?,
    Word::Ident(item0) => {
      f.begin_object(w)?;
      f.begin_object_key(w, true)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "kind")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      f.write_u32(w, 4)?;
      f.end_object_value(w)?;
      f.begin_object_key(w, false)?;
      f.begin_string(w)?;
      f.write_string_fragment(w, "inner")?;
      f.end_string(w)?;
      f.end_object_key(w)?;
      f.begin_object_value(w)?;
      write!(w, "{}", to_json_string(&item0)?)?;
      f.end_object_value(w)?;
      f.end_object(w)?;
    }
  }
  Ok(())
}


pub fn serialize_comments(w: &mut impl Write, f: &mut impl JsonFormatter, leading: &SingleThreadedCommentsMapInner, trailing: &SingleThreadedCommentsMapInner) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "leading")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_object(w)?;
  for (i, (key, value)) in leading.iter().enumerate() {
    f.begin_object_key(w, i == 0)?;
    f.begin_string(w)?;
    f.write_string_fragment(w, &key.0.to_string())?;
    f.end_string(w)?;
    f.end_object_key(w)?;
    f.begin_object_value(w)?;
    serialize_comment_vec(w, f, value)?;
    f.end_object_value(w)?;
  }
  f.end_object(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "trailing")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_object(w)?;
  for (i, (key, value)) in trailing.iter().enumerate() {
    f.begin_object_key(w, i == 0)?;
    f.begin_string(w)?;
    f.write_string_fragment(w, &key.0.to_string())?;
    f.end_string(w)?;
    f.end_object_key(w)?;
    f.begin_object_value(w)?;
    serialize_comment_vec(w, f, value)?;
    f.end_object_value(w)?;
  }
  f.end_object(w)?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}

pub fn serialize_comment_vec(w: &mut impl Write, f: &mut impl JsonFormatter, comments: &Vec<Comment>) -> Result<(), Error> {
  f.begin_array(w)?;
  for (i, comment) in comments.iter().enumerate() {
    f.begin_array_value(w, i == 0)?;
    serialize_comment(w, f, comment)?;
    f.end_array_value(w)?;
  }
  f.end_array(w)?;
  Ok(())
}

pub fn serialize_comment(w: &mut impl Write, f: &mut impl JsonFormatter, comment: &Comment) -> Result<(), Error> {
  f.begin_object(w)?;
  f.begin_object_key(w, true)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "span")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  write!(w, "{}", to_json_string(&comment.span)?)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "text")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, &comment.text)?;
  f.end_string(w)?;
  f.end_object_value(w)?;
  f.begin_object_key(w, false)?;
  f.begin_string(w)?;
  f.write_string_fragment(w, "kind")?;
  f.end_string(w)?;
  f.end_object_key(w)?;
  f.begin_object_value(w)?;
  f.write_u32(w, match comment.kind {
    CommentKind::Line => 0,
    CommentKind::Block => 1,
  })?;
  f.end_object_value(w)?;
  f.end_object(w)?;
  Ok(())
}
