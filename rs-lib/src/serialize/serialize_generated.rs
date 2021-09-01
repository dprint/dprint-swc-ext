// This code is code generated.
// Run `./scripts/generate.sh` from the root directory to regenerate it.
use std::io::{Error, Write};
use serde_json::ser::{Formatter as JsonFormatter, to_string as to_json_string};
use swc_common::{Span, Spanned, comments::{Comment, CommentKind, SingleThreadedCommentsMapInner}};
use swc_ecmascript::parser::token::{BinOpToken, Keyword, Token, TokenAndSpan, Word};
use swc_ecmascript::ast::*;
use super::*;

pub struct FileSerializer<'a, TWrite: Write, TJsonFormatter: JsonFormatter> {
  w: &'a mut TWrite,
  f: &'a mut TJsonFormatter,
  multi_byte_chars: Vec<MultiByteChar>,
}

impl<'a, TWrite: Write, TJsonFormatter: JsonFormatter> FileSerializer<'a, TWrite, TJsonFormatter> {
  pub fn new(w: &'a mut TWrite, f: &'a mut TJsonFormatter, file_text: &str) -> Self {
    FileSerializer {
      w,
      f,
      multi_byte_chars: get_multi_byte_chars(file_text),
    }
  }

  pub fn serialize_array_lit(&mut self, node: &ArrayLit) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 0)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.elems;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "elems")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      match item {
        Some(value) => {
          self.serialize_expr_or_spread(value)?;
        }
        None => self.f.write_null(self.w)?,
      }
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_array_pat(&mut self, node: &ArrayPat) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 1)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.elems;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "elems")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      match item {
        Some(value) => {
          self.serialize_pat(value)?;
        }
        None => self.f.write_null(self.w)?,
      }
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    let value = &node.optional;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "optional")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    match &node.type_ann {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeAnn")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_ann(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_arrow_expr(&mut self, node: &ArrowExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 2)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.params;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "params")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_pat(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    let value = &node.body;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "body")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_block_stmt_or_expr(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.is_async;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isAsync")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.is_generator;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isGenerator")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    match &node.type_params {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeParams")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_param_decl(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }match &node.return_type {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "returnType")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_ann(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_assign_expr(&mut self, node: &AssignExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 3)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.op;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "op")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.left;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "left")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_pat_or_expr(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.right;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "right")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_assign_pat(&mut self, node: &AssignPat) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 4)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.left;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "left")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_pat(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.right;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "right")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    match &node.type_ann {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeAnn")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_ann(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_assign_pat_prop(&mut self, node: &AssignPatProp) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 5)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.key;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "key")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    match &node.value {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "value")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_expr(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_assign_prop(&mut self, node: &AssignProp) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 6)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.key;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "key")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.value;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "value")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_await_expr(&mut self, node: &AwaitExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 7)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.arg;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "arg")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_big_int(&mut self, node: &BigInt) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 8)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.value;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "value")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_bin_expr(&mut self, node: &BinExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 9)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.op;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "op")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.left;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "left")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.right;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "right")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_binding_ident(&mut self, node: &BindingIdent) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 10)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.id;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "id")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    match &node.type_ann {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeAnn")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_ann(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_block_stmt(&mut self, node: &BlockStmt) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 11)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.stmts;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "stmts")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_stmt(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_bool(&mut self, node: &Bool) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 12)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.value;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "value")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_break_stmt(&mut self, node: &BreakStmt) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 13)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    match &node.label {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "label")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ident(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_call_expr(&mut self, node: &CallExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 14)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.callee;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "callee")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr_or_super(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.args;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "args")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_expr_or_spread(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    match &node.type_args {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeArgs")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_param_instantiation(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_catch_clause(&mut self, node: &CatchClause) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 15)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    match &node.param {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "param")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_pat(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.body;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "body")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_block_stmt(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_class(&mut self, node: &Class) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 16)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.decorators;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "decorators")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_decorator(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    let value = &node.body;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "body")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_class_member(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    match &node.super_class {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "superClass")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_expr(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.is_abstract;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isAbstract")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    match &node.type_params {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeParams")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_param_decl(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }match &node.super_type_params {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "superTypeParams")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_param_instantiation(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.implements;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "implements")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_ts_expr_with_type_args(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_class_decl(&mut self, node: &ClassDecl) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 17)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.ident;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "ident")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.declare;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "declare")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.class;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "class")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_class(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_class_expr(&mut self, node: &ClassExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 18)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    match &node.ident {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "ident")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ident(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.class;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "class")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_class(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_class_method(&mut self, node: &ClassMethod) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 19)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.key;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "key")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_prop_name(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.function;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "function")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_function(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.kind;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "methodKind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.is_static;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isStatic")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    match &node.accessibility {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "accessibility")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        write!(self.w, "{}", to_json_string(value)?)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.is_abstract;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isAbstract")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.is_optional;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isOptional")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.is_override;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isOverride")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_class_prop(&mut self, node: &ClassProp) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 20)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.key;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "key")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    match &node.value {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "value")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_expr(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }match &node.type_ann {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeAnn")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_ann(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.is_static;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isStatic")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.decorators;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "decorators")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_decorator(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    let value = &node.computed;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "computed")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    match &node.accessibility {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "accessibility")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        write!(self.w, "{}", to_json_string(value)?)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.is_abstract;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isAbstract")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.is_optional;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isOptional")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.is_override;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isOverride")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.readonly;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "readonly")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.declare;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "declare")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.definite;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "definite")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_computed_prop_name(&mut self, node: &ComputedPropName) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 21)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.expr;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "expr")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_cond_expr(&mut self, node: &CondExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 22)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.test;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "test")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.cons;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "cons")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.alt;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "alt")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_constructor(&mut self, node: &Constructor) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 23)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.key;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "key")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_prop_name(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.params;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "params")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_param_or_ts_param_prop(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    match &node.body {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "body")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_block_stmt(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }match &node.accessibility {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "accessibility")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        write!(self.w, "{}", to_json_string(value)?)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.is_optional;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isOptional")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_continue_stmt(&mut self, node: &ContinueStmt) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 24)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    match &node.label {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "label")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ident(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_debugger_stmt(&mut self, node: &DebuggerStmt) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 25)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_decorator(&mut self, node: &Decorator) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 26)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.expr;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "expr")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_do_while_stmt(&mut self, node: &DoWhileStmt) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 27)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.test;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "test")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.body;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "body")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_stmt(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_empty_stmt(&mut self, node: &EmptyStmt) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 28)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_export_all(&mut self, node: &ExportAll) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 29)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.src;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "src")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_str(value)?;
    self.f.end_object_value(self.w)?;
    match &node.asserts {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "asserts")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_object_lit(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_export_decl(&mut self, node: &ExportDecl) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 30)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.decl;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "decl")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_decl(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_export_default_decl(&mut self, node: &ExportDefaultDecl) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 31)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.decl;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "decl")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_default_decl(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_export_default_expr(&mut self, node: &ExportDefaultExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 32)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.expr;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "expr")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_export_default_specifier(&mut self, node: &ExportDefaultSpecifier) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 33)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.exported;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "exported")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_export_named_specifier(&mut self, node: &ExportNamedSpecifier) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 34)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.orig;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "orig")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    match &node.exported {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "exported")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ident(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_export_namespace_specifier(&mut self, node: &ExportNamespaceSpecifier) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 35)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.name;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "name")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_expr_or_spread(&mut self, node: &ExprOrSpread) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 36)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    match &node.spread {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "spread")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_span(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.expr;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "expr")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_expr_stmt(&mut self, node: &ExprStmt) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 37)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.expr;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "expr")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_fn_decl(&mut self, node: &FnDecl) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 38)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.ident;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "ident")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.declare;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "declare")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.function;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "function")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_function(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_fn_expr(&mut self, node: &FnExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 39)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    match &node.ident {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "ident")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ident(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.function;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "function")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_function(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_for_in_stmt(&mut self, node: &ForInStmt) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 40)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.left;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "left")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_var_decl_or_pat(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.right;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "right")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.body;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "body")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_stmt(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_for_of_stmt(&mut self, node: &ForOfStmt) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 41)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    match &node.await_token {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "awaitToken")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_span(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.left;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "left")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_var_decl_or_pat(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.right;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "right")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.body;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "body")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_stmt(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_for_stmt(&mut self, node: &ForStmt) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 42)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    match &node.init {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "init")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_var_decl_or_expr(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }match &node.test {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "test")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_expr(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }match &node.update {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "update")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_expr(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.body;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "body")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_stmt(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_function(&mut self, node: &Function) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 43)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.params;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "params")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_param(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    let value = &node.decorators;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "decorators")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_decorator(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    match &node.body {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "body")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_block_stmt(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.is_generator;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isGenerator")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.is_async;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isAsync")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    match &node.type_params {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeParams")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_param_decl(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }match &node.return_type {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "returnType")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_ann(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_getter_prop(&mut self, node: &GetterProp) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 44)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.key;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "key")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_prop_name(value)?;
    self.f.end_object_value(self.w)?;
    match &node.type_ann {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeAnn")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_ann(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }match &node.body {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "body")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_block_stmt(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ident(&mut self, node: &Ident) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 45)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.sym;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "sym")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.optional;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "optional")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_if_stmt(&mut self, node: &IfStmt) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 46)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.test;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "test")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.cons;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "cons")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_stmt(value)?;
    self.f.end_object_value(self.w)?;
    match &node.alt {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "alt")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_stmt(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_import_decl(&mut self, node: &ImportDecl) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 47)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.specifiers;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "specifiers")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_import_specifier(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    let value = &node.src;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "src")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_str(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.type_only;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "typeOnly")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    match &node.asserts {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "asserts")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_object_lit(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_import_default_specifier(&mut self, node: &ImportDefaultSpecifier) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 48)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.local;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "local")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_import_named_specifier(&mut self, node: &ImportNamedSpecifier) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 49)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.local;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "local")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    match &node.imported {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "imported")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ident(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_import_star_as_specifier(&mut self, node: &ImportStarAsSpecifier) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 50)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.local;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "local")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_invalid(&mut self, node: &Invalid) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 51)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_jsxattr(&mut self, node: &JSXAttr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 52)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.name;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "name")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_jsxattr_name(value)?;
    self.f.end_object_value(self.w)?;
    match &node.value {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "value")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_jsxattr_value(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_jsxclosing_element(&mut self, node: &JSXClosingElement) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 53)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.name;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "name")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_jsxelement_name(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_jsxclosing_fragment(&mut self, node: &JSXClosingFragment) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 54)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_jsxelement(&mut self, node: &JSXElement) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 55)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.opening;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "opening")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_jsxopening_element(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.children;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "children")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_jsxelement_child(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    match &node.closing {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "closing")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_jsxclosing_element(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_jsxempty_expr(&mut self, node: &JSXEmptyExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 56)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_jsxexpr_container(&mut self, node: &JSXExprContainer) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 57)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.expr;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "expr")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_jsxexpr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_jsxfragment(&mut self, node: &JSXFragment) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 58)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.opening;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "opening")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_jsxopening_fragment(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.children;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "children")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_jsxelement_child(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    let value = &node.closing;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "closing")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_jsxclosing_fragment(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_jsxmember_expr(&mut self, node: &JSXMemberExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 59)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.obj;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "obj")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_jsxobject(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.prop;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "prop")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_jsxnamespaced_name(&mut self, node: &JSXNamespacedName) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 60)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.ns;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "ns")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.name;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "name")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_jsxopening_element(&mut self, node: &JSXOpeningElement) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 61)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.name;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "name")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_jsxelement_name(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.attrs;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "attrs")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_jsxattr_or_spread(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    let value = &node.self_closing;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "selfClosing")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    match &node.type_args {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeArgs")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_param_instantiation(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_jsxopening_fragment(&mut self, node: &JSXOpeningFragment) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 62)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_jsxspread_child(&mut self, node: &JSXSpreadChild) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 63)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.expr;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "expr")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_jsxtext(&mut self, node: &JSXText) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 64)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.value;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "value")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.raw;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "raw")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_key_value_pat_prop(&mut self, node: &KeyValuePatProp) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 65)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.key;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "key")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_prop_name(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.value;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "value")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_pat(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_key_value_prop(&mut self, node: &KeyValueProp) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 66)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.key;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "key")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_prop_name(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.value;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "value")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_labeled_stmt(&mut self, node: &LabeledStmt) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 67)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.label;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "label")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.body;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "body")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_stmt(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_member_expr(&mut self, node: &MemberExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 68)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.obj;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "obj")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr_or_super(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.prop;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "prop")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.computed;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "computed")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_meta_prop_expr(&mut self, node: &MetaPropExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 69)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.meta;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "meta")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.prop;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "prop")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_method_prop(&mut self, node: &MethodProp) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 70)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.key;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "key")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_prop_name(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.function;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "function")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_function(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_module(&mut self, node: &Module) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 71)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.body;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "body")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_module_item(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    match &node.shebang {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "shebang")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        write!(self.w, "{}", to_json_string(value)?)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_named_export(&mut self, node: &NamedExport) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 72)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.specifiers;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "specifiers")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_export_specifier(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    match &node.src {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "src")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_str(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.type_only;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "typeOnly")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    match &node.asserts {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "asserts")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_object_lit(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_new_expr(&mut self, node: &NewExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 73)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.callee;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "callee")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    match &node.args {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "args")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.f.begin_array(self.w)?;
        for (i, item) in value.iter().enumerate() {
          self.f.begin_array_value(self.w, i == 0)?;
          self.serialize_expr_or_spread(item)?;
          self.f.end_array_value(self.w)?;
        }
        self.f.end_array(self.w)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }match &node.type_args {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeArgs")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_param_instantiation(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_null(&mut self, node: &Null) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 74)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_number(&mut self, node: &Number) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 75)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.value;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "value")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_object_lit(&mut self, node: &ObjectLit) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 76)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.props;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "props")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_prop_or_spread(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_object_pat(&mut self, node: &ObjectPat) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 77)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.props;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "props")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_object_pat_prop(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    let value = &node.optional;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "optional")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    match &node.type_ann {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeAnn")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_ann(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_opt_chain_expr(&mut self, node: &OptChainExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 78)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.question_dot_token;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "questionDotToken")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_span(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.expr;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "expr")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_param(&mut self, node: &Param) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 79)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.decorators;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "decorators")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_decorator(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    let value = &node.pat;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "pat")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_pat(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_paren_expr(&mut self, node: &ParenExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 80)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.expr;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "expr")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_private_method(&mut self, node: &PrivateMethod) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 81)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.key;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "key")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_private_name(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.function;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "function")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_function(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.kind;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "methodKind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.is_static;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isStatic")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    match &node.accessibility {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "accessibility")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        write!(self.w, "{}", to_json_string(value)?)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.is_abstract;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isAbstract")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.is_optional;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isOptional")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.is_override;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isOverride")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_private_name(&mut self, node: &PrivateName) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 82)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.id;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "id")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_private_prop(&mut self, node: &PrivateProp) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 83)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.key;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "key")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_private_name(value)?;
    self.f.end_object_value(self.w)?;
    match &node.value {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "value")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_expr(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }match &node.type_ann {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeAnn")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_ann(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.is_static;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isStatic")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.decorators;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "decorators")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_decorator(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    let value = &node.computed;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "computed")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    match &node.accessibility {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "accessibility")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        write!(self.w, "{}", to_json_string(value)?)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.is_abstract;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isAbstract")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.is_optional;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isOptional")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.is_override;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isOverride")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.readonly;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "readonly")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.definite;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "definite")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_regex(&mut self, node: &Regex) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 84)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.exp;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "exp")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.flags;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "flags")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_rest_pat(&mut self, node: &RestPat) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 85)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.dot3_token;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "dot3Token")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_span(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.arg;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "arg")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_pat(value)?;
    self.f.end_object_value(self.w)?;
    match &node.type_ann {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeAnn")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_ann(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_return_stmt(&mut self, node: &ReturnStmt) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 86)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    match &node.arg {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "arg")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_expr(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_script(&mut self, node: &Script) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 87)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.body;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "body")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_stmt(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    match &node.shebang {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "shebang")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        write!(self.w, "{}", to_json_string(value)?)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_seq_expr(&mut self, node: &SeqExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 88)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.exprs;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "exprs")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_expr(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_setter_prop(&mut self, node: &SetterProp) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 89)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.key;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "key")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_prop_name(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.param;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "param")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_pat(value)?;
    self.f.end_object_value(self.w)?;
    match &node.body {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "body")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_block_stmt(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_spread_element(&mut self, node: &SpreadElement) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 90)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.dot3_token;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "dot3Token")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_span(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.expr;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "expr")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_static_block(&mut self, node: &StaticBlock) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 91)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.body;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "body")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_block_stmt(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_str(&mut self, node: &Str) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 92)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.value;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "value")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.has_escape;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "hasEscape")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.kind;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "strKind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_super(&mut self, node: &Super) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 93)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_switch_case(&mut self, node: &SwitchCase) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 94)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    match &node.test {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "test")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_expr(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.cons;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "cons")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_stmt(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_switch_stmt(&mut self, node: &SwitchStmt) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 95)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.discriminant;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "discriminant")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.cases;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "cases")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_switch_case(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_tagged_tpl(&mut self, node: &TaggedTpl) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 96)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.tag;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "tag")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    match &node.type_params {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeParams")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_param_instantiation(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.tpl;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "tpl")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_tpl(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_this_expr(&mut self, node: &ThisExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 97)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_throw_stmt(&mut self, node: &ThrowStmt) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 98)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.arg;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "arg")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_tpl(&mut self, node: &Tpl) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 99)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.exprs;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "exprs")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_expr(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    let value = &node.quasis;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "quasis")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_tpl_element(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_tpl_element(&mut self, node: &TplElement) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 100)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.tail;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "tail")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    match &node.cooked {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "cooked")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_str(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.raw;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "raw")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_str(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_try_stmt(&mut self, node: &TryStmt) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 101)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.block;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "block")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_block_stmt(value)?;
    self.f.end_object_value(self.w)?;
    match &node.handler {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "handler")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_catch_clause(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }match &node.finalizer {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "finalizer")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_block_stmt(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_array_type(&mut self, node: &TsArrayType) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 102)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.elem_type;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "elemType")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_type(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_as_expr(&mut self, node: &TsAsExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 103)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.expr;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "expr")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.type_ann;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "typeAnn")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_type(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_call_signature_decl(&mut self, node: &TsCallSignatureDecl) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 104)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.params;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "params")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_ts_fn_param(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    match &node.type_ann {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeAnn")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_ann(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }match &node.type_params {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeParams")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_param_decl(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_conditional_type(&mut self, node: &TsConditionalType) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 105)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.check_type;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "checkType")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_type(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.extends_type;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "extendsType")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_type(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.true_type;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "trueType")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_type(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.false_type;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "falseType")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_type(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_const_assertion(&mut self, node: &TsConstAssertion) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 106)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.expr;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "expr")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_construct_signature_decl(&mut self, node: &TsConstructSignatureDecl) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 107)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.params;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "params")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_ts_fn_param(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    match &node.type_ann {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeAnn")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_ann(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }match &node.type_params {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeParams")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_param_decl(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_constructor_type(&mut self, node: &TsConstructorType) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 108)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.params;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "params")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_ts_fn_param(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    match &node.type_params {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeParams")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_param_decl(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.type_ann;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "typeAnn")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_type_ann(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.is_abstract;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isAbstract")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_enum_decl(&mut self, node: &TsEnumDecl) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 109)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.declare;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "declare")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.is_const;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isConst")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.id;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "id")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.members;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "members")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_ts_enum_member(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_enum_member(&mut self, node: &TsEnumMember) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 110)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.id;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "id")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_enum_member_id(value)?;
    self.f.end_object_value(self.w)?;
    match &node.init {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "init")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_expr(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_export_assignment(&mut self, node: &TsExportAssignment) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 111)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.expr;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "expr")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_expr_with_type_args(&mut self, node: &TsExprWithTypeArgs) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 112)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.expr;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "expr")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_entity_name(value)?;
    self.f.end_object_value(self.w)?;
    match &node.type_args {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeArgs")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_param_instantiation(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_external_module_ref(&mut self, node: &TsExternalModuleRef) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 113)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.expr;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "expr")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_str(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_fn_type(&mut self, node: &TsFnType) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 114)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.params;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "params")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_ts_fn_param(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    match &node.type_params {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeParams")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_param_decl(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.type_ann;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "typeAnn")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_type_ann(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_getter_signature(&mut self, node: &TsGetterSignature) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 115)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.readonly;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "readonly")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.key;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "key")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.computed;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "computed")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.optional;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "optional")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    match &node.type_ann {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeAnn")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_ann(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_import_equals_decl(&mut self, node: &TsImportEqualsDecl) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 116)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.declare;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "declare")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.is_export;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isExport")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.is_type_only;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isTypeOnly")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.id;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "id")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.module_ref;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "moduleRef")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_module_ref(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_import_type(&mut self, node: &TsImportType) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 117)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.arg;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "arg")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_str(value)?;
    self.f.end_object_value(self.w)?;
    match &node.qualifier {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "qualifier")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_entity_name(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }match &node.type_args {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeArgs")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_param_instantiation(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_index_signature(&mut self, node: &TsIndexSignature) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 118)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.params;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "params")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_ts_fn_param(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    match &node.type_ann {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeAnn")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_ann(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.readonly;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "readonly")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.is_static;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isStatic")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_indexed_access_type(&mut self, node: &TsIndexedAccessType) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 119)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.readonly;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "readonly")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.obj_type;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "objType")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_type(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.index_type;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "indexType")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_type(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_infer_type(&mut self, node: &TsInferType) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 120)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.type_param;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "typeParam")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_type_param(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_interface_body(&mut self, node: &TsInterfaceBody) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 121)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.body;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "body")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_ts_type_element(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_interface_decl(&mut self, node: &TsInterfaceDecl) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 122)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.id;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "id")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.declare;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "declare")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    match &node.type_params {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeParams")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_param_decl(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.extends;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "extends")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_ts_expr_with_type_args(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    let value = &node.body;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "body")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_interface_body(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_intersection_type(&mut self, node: &TsIntersectionType) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 123)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.types;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "types")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_ts_type(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_keyword_type(&mut self, node: &TsKeywordType) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 124)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.kind;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "keywordKind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_lit_type(&mut self, node: &TsLitType) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 125)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.lit;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "lit")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_lit(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_mapped_type(&mut self, node: &TsMappedType) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 126)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    match &node.readonly {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "readonly")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        write!(self.w, "{}", to_json_string(value)?)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.type_param;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "typeParam")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_type_param(value)?;
    self.f.end_object_value(self.w)?;
    match &node.name_type {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "nameType")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }match &node.optional {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "optional")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        write!(self.w, "{}", to_json_string(value)?)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }match &node.type_ann {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeAnn")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_method_signature(&mut self, node: &TsMethodSignature) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 127)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.readonly;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "readonly")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.key;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "key")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.computed;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "computed")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.optional;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "optional")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.params;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "params")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_ts_fn_param(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    match &node.type_ann {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeAnn")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_ann(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }match &node.type_params {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeParams")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_param_decl(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_module_block(&mut self, node: &TsModuleBlock) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 128)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.body;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "body")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_module_item(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_module_decl(&mut self, node: &TsModuleDecl) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 129)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.declare;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "declare")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.global;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "global")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.id;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "id")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_module_name(value)?;
    self.f.end_object_value(self.w)?;
    match &node.body {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "body")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_namespace_body(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_namespace_decl(&mut self, node: &TsNamespaceDecl) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 130)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.declare;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "declare")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.global;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "global")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.id;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "id")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.body;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "body")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_namespace_body(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_namespace_export_decl(&mut self, node: &TsNamespaceExportDecl) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 131)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.id;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "id")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_non_null_expr(&mut self, node: &TsNonNullExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 132)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.expr;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "expr")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_optional_type(&mut self, node: &TsOptionalType) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 133)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.type_ann;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "typeAnn")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_type(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_param_prop(&mut self, node: &TsParamProp) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 134)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.decorators;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "decorators")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_decorator(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    match &node.accessibility {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "accessibility")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        write!(self.w, "{}", to_json_string(value)?)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.is_override;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "isOverride")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.readonly;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "readonly")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.param;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "param")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_param_prop_param(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_parenthesized_type(&mut self, node: &TsParenthesizedType) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 135)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.type_ann;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "typeAnn")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_type(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_property_signature(&mut self, node: &TsPropertySignature) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 136)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.readonly;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "readonly")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.key;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "key")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.computed;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "computed")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.optional;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "optional")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    match &node.init {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "init")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_expr(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.params;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "params")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_ts_fn_param(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    match &node.type_ann {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeAnn")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_ann(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }match &node.type_params {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeParams")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_param_decl(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_qualified_name(&mut self, node: &TsQualifiedName) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 137)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.left;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "left")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_entity_name(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.right;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "right")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_rest_type(&mut self, node: &TsRestType) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 138)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.type_ann;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "typeAnn")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_type(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_setter_signature(&mut self, node: &TsSetterSignature) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 139)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.readonly;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "readonly")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.key;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "key")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.computed;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "computed")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.optional;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "optional")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.param;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "param")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_fn_param(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_this_type(&mut self, node: &TsThisType) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 140)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_tpl_lit_type(&mut self, node: &TsTplLitType) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 141)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.types;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "types")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_ts_type(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    let value = &node.quasis;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "quasis")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_tpl_element(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_tuple_element(&mut self, node: &TsTupleElement) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 142)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    match &node.label {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "label")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_pat(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.ty;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "ty")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_type(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_tuple_type(&mut self, node: &TsTupleType) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 143)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.elem_types;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "elemTypes")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_ts_tuple_element(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_type_alias_decl(&mut self, node: &TsTypeAliasDecl) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 144)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.declare;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "declare")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.id;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "id")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    match &node.type_params {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeParams")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_param_decl(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.type_ann;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "typeAnn")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_type(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_type_ann(&mut self, node: &TsTypeAnn) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 145)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.type_ann;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "typeAnn")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_type(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_type_assertion(&mut self, node: &TsTypeAssertion) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 146)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.expr;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "expr")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.type_ann;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "typeAnn")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_type(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_type_lit(&mut self, node: &TsTypeLit) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 147)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.members;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "members")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_ts_type_element(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_type_operator(&mut self, node: &TsTypeOperator) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 148)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.op;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "op")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.type_ann;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "typeAnn")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_type(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_type_param(&mut self, node: &TsTypeParam) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 149)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.name;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "name")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ident(value)?;
    self.f.end_object_value(self.w)?;
    match &node.constraint {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "constraint")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }match &node.default {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "default")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_type_param_decl(&mut self, node: &TsTypeParamDecl) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 150)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.params;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "params")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_ts_type_param(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_type_param_instantiation(&mut self, node: &TsTypeParamInstantiation) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 151)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.params;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "params")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_ts_type(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_type_predicate(&mut self, node: &TsTypePredicate) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 152)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.asserts;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "asserts")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.param_name;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "paramName")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_this_type_or_ident(value)?;
    self.f.end_object_value(self.w)?;
    match &node.type_ann {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeAnn")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_ann(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_type_query(&mut self, node: &TsTypeQuery) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 153)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.expr_name;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "exprName")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_type_query_expr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_type_ref(&mut self, node: &TsTypeRef) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 154)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.type_name;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "typeName")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_ts_entity_name(value)?;
    self.f.end_object_value(self.w)?;
    match &node.type_params {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "typeParams")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_ts_type_param_instantiation(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_ts_union_type(&mut self, node: &TsUnionType) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 155)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.types;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "types")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_ts_type(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_unary_expr(&mut self, node: &UnaryExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 156)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.op;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "op")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.arg;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "arg")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_update_expr(&mut self, node: &UpdateExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 157)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.op;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "op")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.prefix;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "prefix")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.arg;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "arg")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_var_decl(&mut self, node: &VarDecl) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 158)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.kind;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "declKind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.declare;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "declare")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    let value = &node.decls;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "decls")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_array(self.w)?;
    for (i, item) in value.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_var_declarator(item)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_var_declarator(&mut self, node: &VarDeclarator) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 159)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.name;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "name")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_pat(value)?;
    self.f.end_object_value(self.w)?;
    match &node.init {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "init")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_expr(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.definite;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "definite")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_while_stmt(&mut self, node: &WhileStmt) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 160)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.test;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "test")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.body;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "body")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_stmt(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_with_stmt(&mut self, node: &WithStmt) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 161)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    let value = &node.obj;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "obj")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_expr(value)?;
    self.f.end_object_value(self.w)?;
    let value = &node.body;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "body")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_stmt(value)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_yield_expr(&mut self, node: &YieldExpr) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, 162)?;
    self.f.end_object_value(self.w)?;
    self.serialize_span_props(&node.span(), false)?;
    match &node.arg {
      Some(value) => {
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "arg")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_expr(value)?;
        self.f.end_object_value(self.w)?;
      }
      None => {}
    }
    let value = &node.delegate;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "delegate")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    write!(self.w, "{}", to_json_string(value)?)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_block_stmt_or_expr(&mut self, node: &BlockStmtOrExpr) -> Result<(), Error> {
    match node {
      BlockStmtOrExpr::BlockStmt(node) => self.serialize_block_stmt(node)?,
      BlockStmtOrExpr::Expr(node) => self.serialize_expr(node)?,
    }
    Ok(())
  }

  pub fn serialize_class_member(&mut self, node: &ClassMember) -> Result<(), Error> {
    match node {
      ClassMember::Constructor(node) => self.serialize_constructor(node)?,
      ClassMember::Method(node) => self.serialize_class_method(node)?,
      ClassMember::PrivateMethod(node) => self.serialize_private_method(node)?,
      ClassMember::ClassProp(node) => self.serialize_class_prop(node)?,
      ClassMember::PrivateProp(node) => self.serialize_private_prop(node)?,
      ClassMember::TsIndexSignature(node) => self.serialize_ts_index_signature(node)?,
      ClassMember::Empty(node) => self.serialize_empty_stmt(node)?,
      ClassMember::StaticBlock(node) => self.serialize_static_block(node)?,
    }
    Ok(())
  }

  pub fn serialize_decl(&mut self, node: &Decl) -> Result<(), Error> {
    match node {
      Decl::Class(node) => self.serialize_class_decl(node)?,
      Decl::Fn(node) => self.serialize_fn_decl(node)?,
      Decl::Var(node) => self.serialize_var_decl(node)?,
      Decl::TsInterface(node) => self.serialize_ts_interface_decl(node)?,
      Decl::TsTypeAlias(node) => self.serialize_ts_type_alias_decl(node)?,
      Decl::TsEnum(node) => self.serialize_ts_enum_decl(node)?,
      Decl::TsModule(node) => self.serialize_ts_module_decl(node)?,
    }
    Ok(())
  }

  pub fn serialize_default_decl(&mut self, node: &DefaultDecl) -> Result<(), Error> {
    match node {
      DefaultDecl::Class(node) => self.serialize_class_expr(node)?,
      DefaultDecl::Fn(node) => self.serialize_fn_expr(node)?,
      DefaultDecl::TsInterfaceDecl(node) => self.serialize_ts_interface_decl(node)?,
    }
    Ok(())
  }

  pub fn serialize_export_specifier(&mut self, node: &ExportSpecifier) -> Result<(), Error> {
    match node {
      ExportSpecifier::Namespace(node) => self.serialize_export_namespace_specifier(node)?,
      ExportSpecifier::Default(node) => self.serialize_export_default_specifier(node)?,
      ExportSpecifier::Named(node) => self.serialize_export_named_specifier(node)?,
    }
    Ok(())
  }

  pub fn serialize_expr(&mut self, node: &Expr) -> Result<(), Error> {
    match node {
      Expr::This(node) => self.serialize_this_expr(node)?,
      Expr::Array(node) => self.serialize_array_lit(node)?,
      Expr::Object(node) => self.serialize_object_lit(node)?,
      Expr::Fn(node) => self.serialize_fn_expr(node)?,
      Expr::Unary(node) => self.serialize_unary_expr(node)?,
      Expr::Update(node) => self.serialize_update_expr(node)?,
      Expr::Bin(node) => self.serialize_bin_expr(node)?,
      Expr::Assign(node) => self.serialize_assign_expr(node)?,
      Expr::Member(node) => self.serialize_member_expr(node)?,
      Expr::Cond(node) => self.serialize_cond_expr(node)?,
      Expr::Call(node) => self.serialize_call_expr(node)?,
      Expr::New(node) => self.serialize_new_expr(node)?,
      Expr::Seq(node) => self.serialize_seq_expr(node)?,
      Expr::Ident(node) => self.serialize_ident(node)?,
      Expr::Lit(node) => self.serialize_lit(node)?,
      Expr::Tpl(node) => self.serialize_tpl(node)?,
      Expr::TaggedTpl(node) => self.serialize_tagged_tpl(node)?,
      Expr::Arrow(node) => self.serialize_arrow_expr(node)?,
      Expr::Class(node) => self.serialize_class_expr(node)?,
      Expr::Yield(node) => self.serialize_yield_expr(node)?,
      Expr::MetaProp(node) => self.serialize_meta_prop_expr(node)?,
      Expr::Await(node) => self.serialize_await_expr(node)?,
      Expr::Paren(node) => self.serialize_paren_expr(node)?,
      Expr::JSXMember(node) => self.serialize_jsxmember_expr(node)?,
      Expr::JSXNamespacedName(node) => self.serialize_jsxnamespaced_name(node)?,
      Expr::JSXEmpty(node) => self.serialize_jsxempty_expr(node)?,
      Expr::JSXElement(node) => self.serialize_jsxelement(node)?,
      Expr::JSXFragment(node) => self.serialize_jsxfragment(node)?,
      Expr::TsTypeAssertion(node) => self.serialize_ts_type_assertion(node)?,
      Expr::TsConstAssertion(node) => self.serialize_ts_const_assertion(node)?,
      Expr::TsNonNull(node) => self.serialize_ts_non_null_expr(node)?,
      Expr::TsAs(node) => self.serialize_ts_as_expr(node)?,
      Expr::PrivateName(node) => self.serialize_private_name(node)?,
      Expr::OptChain(node) => self.serialize_opt_chain_expr(node)?,
      Expr::Invalid(node) => self.serialize_invalid(node)?,
    }
    Ok(())
  }

  pub fn serialize_expr_or_super(&mut self, node: &ExprOrSuper) -> Result<(), Error> {
    match node {
      ExprOrSuper::Super(node) => self.serialize_super(node)?,
      ExprOrSuper::Expr(node) => self.serialize_expr(node)?,
    }
    Ok(())
  }

  pub fn serialize_import_specifier(&mut self, node: &ImportSpecifier) -> Result<(), Error> {
    match node {
      ImportSpecifier::Named(node) => self.serialize_import_named_specifier(node)?,
      ImportSpecifier::Default(node) => self.serialize_import_default_specifier(node)?,
      ImportSpecifier::Namespace(node) => self.serialize_import_star_as_specifier(node)?,
    }
    Ok(())
  }

  pub fn serialize_jsxattr_name(&mut self, node: &JSXAttrName) -> Result<(), Error> {
    match node {
      JSXAttrName::Ident(node) => self.serialize_ident(node)?,
      JSXAttrName::JSXNamespacedName(node) => self.serialize_jsxnamespaced_name(node)?,
    }
    Ok(())
  }

  pub fn serialize_jsxattr_or_spread(&mut self, node: &JSXAttrOrSpread) -> Result<(), Error> {
    match node {
      JSXAttrOrSpread::JSXAttr(node) => self.serialize_jsxattr(node)?,
      JSXAttrOrSpread::SpreadElement(node) => self.serialize_spread_element(node)?,
    }
    Ok(())
  }

  pub fn serialize_jsxattr_value(&mut self, node: &JSXAttrValue) -> Result<(), Error> {
    match node {
      JSXAttrValue::Lit(node) => self.serialize_lit(node)?,
      JSXAttrValue::JSXExprContainer(node) => self.serialize_jsxexpr_container(node)?,
      JSXAttrValue::JSXElement(node) => self.serialize_jsxelement(node)?,
      JSXAttrValue::JSXFragment(node) => self.serialize_jsxfragment(node)?,
    }
    Ok(())
  }

  pub fn serialize_jsxelement_child(&mut self, node: &JSXElementChild) -> Result<(), Error> {
    match node {
      JSXElementChild::JSXText(node) => self.serialize_jsxtext(node)?,
      JSXElementChild::JSXExprContainer(node) => self.serialize_jsxexpr_container(node)?,
      JSXElementChild::JSXSpreadChild(node) => self.serialize_jsxspread_child(node)?,
      JSXElementChild::JSXElement(node) => self.serialize_jsxelement(node)?,
      JSXElementChild::JSXFragment(node) => self.serialize_jsxfragment(node)?,
    }
    Ok(())
  }

  pub fn serialize_jsxelement_name(&mut self, node: &JSXElementName) -> Result<(), Error> {
    match node {
      JSXElementName::Ident(node) => self.serialize_ident(node)?,
      JSXElementName::JSXMemberExpr(node) => self.serialize_jsxmember_expr(node)?,
      JSXElementName::JSXNamespacedName(node) => self.serialize_jsxnamespaced_name(node)?,
    }
    Ok(())
  }

  pub fn serialize_jsxexpr(&mut self, node: &JSXExpr) -> Result<(), Error> {
    match node {
      JSXExpr::JSXEmptyExpr(node) => self.serialize_jsxempty_expr(node)?,
      JSXExpr::Expr(node) => self.serialize_expr(node)?,
    }
    Ok(())
  }

  pub fn serialize_jsxobject(&mut self, node: &JSXObject) -> Result<(), Error> {
    match node {
      JSXObject::JSXMemberExpr(node) => self.serialize_jsxmember_expr(node)?,
      JSXObject::Ident(node) => self.serialize_ident(node)?,
    }
    Ok(())
  }

  pub fn serialize_lit(&mut self, node: &Lit) -> Result<(), Error> {
    match node {
      Lit::Str(node) => self.serialize_str(node)?,
      Lit::Bool(node) => self.serialize_bool(node)?,
      Lit::Null(node) => self.serialize_null(node)?,
      Lit::Num(node) => self.serialize_number(node)?,
      Lit::BigInt(node) => self.serialize_big_int(node)?,
      Lit::Regex(node) => self.serialize_regex(node)?,
      Lit::JSXText(node) => self.serialize_jsxtext(node)?,
    }
    Ok(())
  }

  pub fn serialize_module_decl(&mut self, node: &ModuleDecl) -> Result<(), Error> {
    match node {
      ModuleDecl::Import(node) => self.serialize_import_decl(node)?,
      ModuleDecl::ExportDecl(node) => self.serialize_export_decl(node)?,
      ModuleDecl::ExportNamed(node) => self.serialize_named_export(node)?,
      ModuleDecl::ExportDefaultDecl(node) => self.serialize_export_default_decl(node)?,
      ModuleDecl::ExportDefaultExpr(node) => self.serialize_export_default_expr(node)?,
      ModuleDecl::ExportAll(node) => self.serialize_export_all(node)?,
      ModuleDecl::TsImportEquals(node) => self.serialize_ts_import_equals_decl(node)?,
      ModuleDecl::TsExportAssignment(node) => self.serialize_ts_export_assignment(node)?,
      ModuleDecl::TsNamespaceExport(node) => self.serialize_ts_namespace_export_decl(node)?,
    }
    Ok(())
  }

  pub fn serialize_module_item(&mut self, node: &ModuleItem) -> Result<(), Error> {
    match node {
      ModuleItem::ModuleDecl(node) => self.serialize_module_decl(node)?,
      ModuleItem::Stmt(node) => self.serialize_stmt(node)?,
    }
    Ok(())
  }

  pub fn serialize_object_pat_prop(&mut self, node: &ObjectPatProp) -> Result<(), Error> {
    match node {
      ObjectPatProp::KeyValue(node) => self.serialize_key_value_pat_prop(node)?,
      ObjectPatProp::Assign(node) => self.serialize_assign_pat_prop(node)?,
      ObjectPatProp::Rest(node) => self.serialize_rest_pat(node)?,
    }
    Ok(())
  }

  pub fn serialize_param_or_ts_param_prop(&mut self, node: &ParamOrTsParamProp) -> Result<(), Error> {
    match node {
      ParamOrTsParamProp::TsParamProp(node) => self.serialize_ts_param_prop(node)?,
      ParamOrTsParamProp::Param(node) => self.serialize_param(node)?,
    }
    Ok(())
  }

  pub fn serialize_pat(&mut self, node: &Pat) -> Result<(), Error> {
    match node {
      Pat::Ident(node) => self.serialize_binding_ident(node)?,
      Pat::Array(node) => self.serialize_array_pat(node)?,
      Pat::Rest(node) => self.serialize_rest_pat(node)?,
      Pat::Object(node) => self.serialize_object_pat(node)?,
      Pat::Assign(node) => self.serialize_assign_pat(node)?,
      Pat::Invalid(node) => self.serialize_invalid(node)?,
      Pat::Expr(node) => self.serialize_expr(node)?,
    }
    Ok(())
  }

  pub fn serialize_pat_or_expr(&mut self, node: &PatOrExpr) -> Result<(), Error> {
    match node {
      PatOrExpr::Expr(node) => self.serialize_expr(node)?,
      PatOrExpr::Pat(node) => self.serialize_pat(node)?,
    }
    Ok(())
  }

  pub fn serialize_prop(&mut self, node: &Prop) -> Result<(), Error> {
    match node {
      Prop::Shorthand(node) => self.serialize_ident(node)?,
      Prop::KeyValue(node) => self.serialize_key_value_prop(node)?,
      Prop::Assign(node) => self.serialize_assign_prop(node)?,
      Prop::Getter(node) => self.serialize_getter_prop(node)?,
      Prop::Setter(node) => self.serialize_setter_prop(node)?,
      Prop::Method(node) => self.serialize_method_prop(node)?,
    }
    Ok(())
  }

  pub fn serialize_prop_name(&mut self, node: &PropName) -> Result<(), Error> {
    match node {
      PropName::Ident(node) => self.serialize_ident(node)?,
      PropName::Str(node) => self.serialize_str(node)?,
      PropName::Num(node) => self.serialize_number(node)?,
      PropName::Computed(node) => self.serialize_computed_prop_name(node)?,
      PropName::BigInt(node) => self.serialize_big_int(node)?,
    }
    Ok(())
  }

  pub fn serialize_prop_or_spread(&mut self, node: &PropOrSpread) -> Result<(), Error> {
    match node {
      PropOrSpread::Spread(node) => self.serialize_spread_element(node)?,
      PropOrSpread::Prop(node) => self.serialize_prop(node)?,
    }
    Ok(())
  }

  pub fn serialize_stmt(&mut self, node: &Stmt) -> Result<(), Error> {
    match node {
      Stmt::Block(node) => self.serialize_block_stmt(node)?,
      Stmt::Empty(node) => self.serialize_empty_stmt(node)?,
      Stmt::Debugger(node) => self.serialize_debugger_stmt(node)?,
      Stmt::With(node) => self.serialize_with_stmt(node)?,
      Stmt::Return(node) => self.serialize_return_stmt(node)?,
      Stmt::Labeled(node) => self.serialize_labeled_stmt(node)?,
      Stmt::Break(node) => self.serialize_break_stmt(node)?,
      Stmt::Continue(node) => self.serialize_continue_stmt(node)?,
      Stmt::If(node) => self.serialize_if_stmt(node)?,
      Stmt::Switch(node) => self.serialize_switch_stmt(node)?,
      Stmt::Throw(node) => self.serialize_throw_stmt(node)?,
      Stmt::Try(node) => self.serialize_try_stmt(node)?,
      Stmt::While(node) => self.serialize_while_stmt(node)?,
      Stmt::DoWhile(node) => self.serialize_do_while_stmt(node)?,
      Stmt::For(node) => self.serialize_for_stmt(node)?,
      Stmt::ForIn(node) => self.serialize_for_in_stmt(node)?,
      Stmt::ForOf(node) => self.serialize_for_of_stmt(node)?,
      Stmt::Decl(node) => self.serialize_decl(node)?,
      Stmt::Expr(node) => self.serialize_expr_stmt(node)?,
    }
    Ok(())
  }

  pub fn serialize_ts_entity_name(&mut self, node: &TsEntityName) -> Result<(), Error> {
    match node {
      TsEntityName::TsQualifiedName(node) => self.serialize_ts_qualified_name(node)?,
      TsEntityName::Ident(node) => self.serialize_ident(node)?,
    }
    Ok(())
  }

  pub fn serialize_ts_enum_member_id(&mut self, node: &TsEnumMemberId) -> Result<(), Error> {
    match node {
      TsEnumMemberId::Ident(node) => self.serialize_ident(node)?,
      TsEnumMemberId::Str(node) => self.serialize_str(node)?,
    }
    Ok(())
  }

  pub fn serialize_ts_fn_or_constructor_type(&mut self, node: &TsFnOrConstructorType) -> Result<(), Error> {
    match node {
      TsFnOrConstructorType::TsFnType(node) => self.serialize_ts_fn_type(node)?,
      TsFnOrConstructorType::TsConstructorType(node) => self.serialize_ts_constructor_type(node)?,
    }
    Ok(())
  }

  pub fn serialize_ts_fn_param(&mut self, node: &TsFnParam) -> Result<(), Error> {
    match node {
      TsFnParam::Ident(node) => self.serialize_binding_ident(node)?,
      TsFnParam::Array(node) => self.serialize_array_pat(node)?,
      TsFnParam::Rest(node) => self.serialize_rest_pat(node)?,
      TsFnParam::Object(node) => self.serialize_object_pat(node)?,
    }
    Ok(())
  }

  pub fn serialize_ts_lit(&mut self, node: &TsLit) -> Result<(), Error> {
    match node {
      TsLit::Number(node) => self.serialize_number(node)?,
      TsLit::Str(node) => self.serialize_str(node)?,
      TsLit::Bool(node) => self.serialize_bool(node)?,
      TsLit::BigInt(node) => self.serialize_big_int(node)?,
      TsLit::Tpl(node) => self.serialize_ts_tpl_lit_type(node)?,
    }
    Ok(())
  }

  pub fn serialize_ts_module_name(&mut self, node: &TsModuleName) -> Result<(), Error> {
    match node {
      TsModuleName::Ident(node) => self.serialize_ident(node)?,
      TsModuleName::Str(node) => self.serialize_str(node)?,
    }
    Ok(())
  }

  pub fn serialize_ts_module_ref(&mut self, node: &TsModuleRef) -> Result<(), Error> {
    match node {
      TsModuleRef::TsEntityName(node) => self.serialize_ts_entity_name(node)?,
      TsModuleRef::TsExternalModuleRef(node) => self.serialize_ts_external_module_ref(node)?,
    }
    Ok(())
  }

  pub fn serialize_ts_namespace_body(&mut self, node: &TsNamespaceBody) -> Result<(), Error> {
    match node {
      TsNamespaceBody::TsModuleBlock(node) => self.serialize_ts_module_block(node)?,
      TsNamespaceBody::TsNamespaceDecl(node) => self.serialize_ts_namespace_decl(node)?,
    }
    Ok(())
  }

  pub fn serialize_ts_param_prop_param(&mut self, node: &TsParamPropParam) -> Result<(), Error> {
    match node {
      TsParamPropParam::Ident(node) => self.serialize_binding_ident(node)?,
      TsParamPropParam::Assign(node) => self.serialize_assign_pat(node)?,
    }
    Ok(())
  }

  pub fn serialize_ts_this_type_or_ident(&mut self, node: &TsThisTypeOrIdent) -> Result<(), Error> {
    match node {
      TsThisTypeOrIdent::TsThisType(node) => self.serialize_ts_this_type(node)?,
      TsThisTypeOrIdent::Ident(node) => self.serialize_ident(node)?,
    }
    Ok(())
  }

  pub fn serialize_ts_type(&mut self, node: &TsType) -> Result<(), Error> {
    match node {
      TsType::TsKeywordType(node) => self.serialize_ts_keyword_type(node)?,
      TsType::TsThisType(node) => self.serialize_ts_this_type(node)?,
      TsType::TsFnOrConstructorType(node) => self.serialize_ts_fn_or_constructor_type(node)?,
      TsType::TsTypeRef(node) => self.serialize_ts_type_ref(node)?,
      TsType::TsTypeQuery(node) => self.serialize_ts_type_query(node)?,
      TsType::TsTypeLit(node) => self.serialize_ts_type_lit(node)?,
      TsType::TsArrayType(node) => self.serialize_ts_array_type(node)?,
      TsType::TsTupleType(node) => self.serialize_ts_tuple_type(node)?,
      TsType::TsOptionalType(node) => self.serialize_ts_optional_type(node)?,
      TsType::TsRestType(node) => self.serialize_ts_rest_type(node)?,
      TsType::TsUnionOrIntersectionType(node) => self.serialize_ts_union_or_intersection_type(node)?,
      TsType::TsConditionalType(node) => self.serialize_ts_conditional_type(node)?,
      TsType::TsInferType(node) => self.serialize_ts_infer_type(node)?,
      TsType::TsParenthesizedType(node) => self.serialize_ts_parenthesized_type(node)?,
      TsType::TsTypeOperator(node) => self.serialize_ts_type_operator(node)?,
      TsType::TsIndexedAccessType(node) => self.serialize_ts_indexed_access_type(node)?,
      TsType::TsMappedType(node) => self.serialize_ts_mapped_type(node)?,
      TsType::TsLitType(node) => self.serialize_ts_lit_type(node)?,
      TsType::TsTypePredicate(node) => self.serialize_ts_type_predicate(node)?,
      TsType::TsImportType(node) => self.serialize_ts_import_type(node)?,
    }
    Ok(())
  }

  pub fn serialize_ts_type_element(&mut self, node: &TsTypeElement) -> Result<(), Error> {
    match node {
      TsTypeElement::TsCallSignatureDecl(node) => self.serialize_ts_call_signature_decl(node)?,
      TsTypeElement::TsConstructSignatureDecl(node) => self.serialize_ts_construct_signature_decl(node)?,
      TsTypeElement::TsPropertySignature(node) => self.serialize_ts_property_signature(node)?,
      TsTypeElement::TsGetterSignature(node) => self.serialize_ts_getter_signature(node)?,
      TsTypeElement::TsSetterSignature(node) => self.serialize_ts_setter_signature(node)?,
      TsTypeElement::TsMethodSignature(node) => self.serialize_ts_method_signature(node)?,
      TsTypeElement::TsIndexSignature(node) => self.serialize_ts_index_signature(node)?,
    }
    Ok(())
  }

  pub fn serialize_ts_type_query_expr(&mut self, node: &TsTypeQueryExpr) -> Result<(), Error> {
    match node {
      TsTypeQueryExpr::TsEntityName(node) => self.serialize_ts_entity_name(node)?,
      TsTypeQueryExpr::Import(node) => self.serialize_ts_import_type(node)?,
    }
    Ok(())
  }

  pub fn serialize_ts_union_or_intersection_type(&mut self, node: &TsUnionOrIntersectionType) -> Result<(), Error> {
    match node {
      TsUnionOrIntersectionType::TsUnionType(node) => self.serialize_ts_union_type(node)?,
      TsUnionOrIntersectionType::TsIntersectionType(node) => self.serialize_ts_intersection_type(node)?,
    }
    Ok(())
  }

  pub fn serialize_var_decl_or_expr(&mut self, node: &VarDeclOrExpr) -> Result<(), Error> {
    match node {
      VarDeclOrExpr::VarDecl(node) => self.serialize_var_decl(node)?,
      VarDeclOrExpr::Expr(node) => self.serialize_expr(node)?,
    }
    Ok(())
  }

  pub fn serialize_var_decl_or_pat(&mut self, node: &VarDeclOrPat) -> Result<(), Error> {
    match node {
      VarDeclOrPat::VarDecl(node) => self.serialize_var_decl(node)?,
      VarDeclOrPat::Pat(node) => self.serialize_pat(node)?,
    }
    Ok(())
  }

  pub fn serialize_token_and_spans(&mut self, tokens: &[TokenAndSpan]) -> Result<(), Error> {
    self.f.begin_array(self.w)?;
    for (i, token_and_span) in tokens.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_token_and_span(&token_and_span)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    Ok(())
  }

  pub fn serialize_token_and_span(&mut self, token_and_span: &TokenAndSpan) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.serialize_span_props(&token_and_span.span, true)?;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "hadLineBreak")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_bool(self.w, token_and_span.had_line_break)?;
    self.f.end_object_value(self.w)?;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "token")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.serialize_token(&token_and_span.token)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  fn serialize_bin_op_token(&mut self, value: &BinOpToken) -> Result<(), Error> {
    match value {
      BinOpToken::EqEq => self.f.write_u32(self.w, 0)?,
      BinOpToken::NotEq => self.f.write_u32(self.w, 1)?,
      BinOpToken::EqEqEq => self.f.write_u32(self.w, 2)?,
      BinOpToken::NotEqEq => self.f.write_u32(self.w, 3)?,
      BinOpToken::Lt => self.f.write_u32(self.w, 4)?,
      BinOpToken::LtEq => self.f.write_u32(self.w, 5)?,
      BinOpToken::Gt => self.f.write_u32(self.w, 6)?,
      BinOpToken::GtEq => self.f.write_u32(self.w, 7)?,
      BinOpToken::LShift => self.f.write_u32(self.w, 8)?,
      BinOpToken::RShift => self.f.write_u32(self.w, 9)?,
      BinOpToken::ZeroFillRShift => self.f.write_u32(self.w, 10)?,
      BinOpToken::Add => self.f.write_u32(self.w, 11)?,
      BinOpToken::Sub => self.f.write_u32(self.w, 12)?,
      BinOpToken::Mul => self.f.write_u32(self.w, 13)?,
      BinOpToken::Div => self.f.write_u32(self.w, 14)?,
      BinOpToken::Mod => self.f.write_u32(self.w, 15)?,
      BinOpToken::BitOr => self.f.write_u32(self.w, 16)?,
      BinOpToken::BitXor => self.f.write_u32(self.w, 17)?,
      BinOpToken::BitAnd => self.f.write_u32(self.w, 18)?,
      BinOpToken::Exp => self.f.write_u32(self.w, 19)?,
      BinOpToken::LogicalOr => self.f.write_u32(self.w, 20)?,
      BinOpToken::LogicalAnd => self.f.write_u32(self.w, 21)?,
      BinOpToken::NullishCoalescing => self.f.write_u32(self.w, 22)?,
    }
    Ok(())
  }

  fn serialize_keyword(&mut self, value: &Keyword) -> Result<(), Error> {
    match value {
      Keyword::Await => self.f.write_u32(self.w, 0)?,
      Keyword::Break => self.f.write_u32(self.w, 1)?,
      Keyword::Case => self.f.write_u32(self.w, 2)?,
      Keyword::Catch => self.f.write_u32(self.w, 3)?,
      Keyword::Continue => self.f.write_u32(self.w, 4)?,
      Keyword::Debugger => self.f.write_u32(self.w, 5)?,
      Keyword::Default_ => self.f.write_u32(self.w, 6)?,
      Keyword::Do => self.f.write_u32(self.w, 7)?,
      Keyword::Else => self.f.write_u32(self.w, 8)?,
      Keyword::Finally => self.f.write_u32(self.w, 9)?,
      Keyword::For => self.f.write_u32(self.w, 10)?,
      Keyword::Function => self.f.write_u32(self.w, 11)?,
      Keyword::If => self.f.write_u32(self.w, 12)?,
      Keyword::Return => self.f.write_u32(self.w, 13)?,
      Keyword::Switch => self.f.write_u32(self.w, 14)?,
      Keyword::Throw => self.f.write_u32(self.w, 15)?,
      Keyword::Try => self.f.write_u32(self.w, 16)?,
      Keyword::Var => self.f.write_u32(self.w, 17)?,
      Keyword::Let => self.f.write_u32(self.w, 18)?,
      Keyword::Const => self.f.write_u32(self.w, 19)?,
      Keyword::While => self.f.write_u32(self.w, 20)?,
      Keyword::With => self.f.write_u32(self.w, 21)?,
      Keyword::New => self.f.write_u32(self.w, 22)?,
      Keyword::This => self.f.write_u32(self.w, 23)?,
      Keyword::Super => self.f.write_u32(self.w, 24)?,
      Keyword::Class => self.f.write_u32(self.w, 25)?,
      Keyword::Extends => self.f.write_u32(self.w, 26)?,
      Keyword::Export => self.f.write_u32(self.w, 27)?,
      Keyword::Import => self.f.write_u32(self.w, 28)?,
      Keyword::Yield => self.f.write_u32(self.w, 29)?,
      Keyword::In => self.f.write_u32(self.w, 30)?,
      Keyword::InstanceOf => self.f.write_u32(self.w, 31)?,
      Keyword::TypeOf => self.f.write_u32(self.w, 32)?,
      Keyword::Void => self.f.write_u32(self.w, 33)?,
      Keyword::Delete => self.f.write_u32(self.w, 34)?,
    }
    Ok(())
  }

  fn serialize_token(&mut self, value: &Token) -> Result<(), Error> {
    match value {
      Token::Word(item0) => {
        self.f.begin_object(self.w)?;
        self.f.begin_object_key(self.w, true)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "kind")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.f.write_u32(self.w, 0)?;
        self.f.end_object_value(self.w)?;
        let value = &item0;
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "inner")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_word(value)?;
        self.f.end_object_value(self.w)?;
        self.f.end_object(self.w)?;
      }
      Token::Arrow => self.f.write_u32(self.w, 1)?,
      Token::Hash => self.f.write_u32(self.w, 2)?,
      Token::At => self.f.write_u32(self.w, 3)?,
      Token::Dot => self.f.write_u32(self.w, 4)?,
      Token::DotDotDot => self.f.write_u32(self.w, 5)?,
      Token::Bang => self.f.write_u32(self.w, 6)?,
      Token::LParen => self.f.write_u32(self.w, 7)?,
      Token::RParen => self.f.write_u32(self.w, 8)?,
      Token::LBracket => self.f.write_u32(self.w, 9)?,
      Token::RBracket => self.f.write_u32(self.w, 10)?,
      Token::LBrace => self.f.write_u32(self.w, 11)?,
      Token::RBrace => self.f.write_u32(self.w, 12)?,
      Token::Semi => self.f.write_u32(self.w, 13)?,
      Token::Comma => self.f.write_u32(self.w, 14)?,
      Token::BackQuote => self.f.write_u32(self.w, 15)?,
      Token::Template {
        raw,
        cooked,
        has_escape,
      } => {
        self.f.begin_object(self.w)?;
        self.f.begin_object_key(self.w, true)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "kind")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.f.write_u32(self.w, 16)?;
        self.f.end_object_value(self.w)?;
        let value = &raw;
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "raw")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        write!(self.w, "{}", to_json_string(value)?)?;
        self.f.end_object_value(self.w)?;
        match &cooked {
          Some(value) => {
            self.f.begin_object_key(self.w, false)?;
            self.f.begin_string(self.w)?;
            self.f.write_string_fragment(self.w, "cooked")?;
            self.f.end_string(self.w)?;
            self.f.end_object_key(self.w)?;
            self.f.begin_object_value(self.w)?;
            write!(self.w, "{}", to_json_string(value)?)?;
            self.f.end_object_value(self.w)?;
          }
          None => {}
        }
        let value = &has_escape;
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "hasEscape")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        write!(self.w, "{}", to_json_string(value)?)?;
        self.f.end_object_value(self.w)?;
        self.f.end_object(self.w)?;
      }
      Token::Colon => self.f.write_u32(self.w, 17)?,
      Token::ColonColon => self.f.write_u32(self.w, 18)?,
      Token::BinOp(item0) => {
        self.f.begin_object(self.w)?;
        self.f.begin_object_key(self.w, true)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "kind")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.f.write_u32(self.w, 19)?;
        self.f.end_object_value(self.w)?;
        let value = &item0;
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "inner")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_bin_op_token(value)?;
        self.f.end_object_value(self.w)?;
        self.f.end_object(self.w)?;
      }
      Token::AssignOp(item0) => {
        self.f.begin_object(self.w)?;
        self.f.begin_object_key(self.w, true)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "kind")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.f.write_u32(self.w, 20)?;
        self.f.end_object_value(self.w)?;
        let value = &item0;
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "inner")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        write!(self.w, "{}", to_json_string(value)?)?;
        self.f.end_object_value(self.w)?;
        self.f.end_object(self.w)?;
      }
      Token::DollarLBrace => self.f.write_u32(self.w, 21)?,
      Token::QuestionMark => self.f.write_u32(self.w, 22)?,
      Token::PlusPlus => self.f.write_u32(self.w, 23)?,
      Token::MinusMinus => self.f.write_u32(self.w, 24)?,
      Token::Tilde => self.f.write_u32(self.w, 25)?,
      Token::Str {
        value,
        has_escape,
      } => {
        self.f.begin_object(self.w)?;
        self.f.begin_object_key(self.w, true)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "kind")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.f.write_u32(self.w, 26)?;
        self.f.end_object_value(self.w)?;
        let value = &value;
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "value")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        write!(self.w, "{}", to_json_string(value)?)?;
        self.f.end_object_value(self.w)?;
        let value = &has_escape;
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "hasEscape")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        write!(self.w, "{}", to_json_string(value)?)?;
        self.f.end_object_value(self.w)?;
        self.f.end_object(self.w)?;
      }
      Token::Regex(item0, item1) => {
        self.f.begin_object(self.w)?;
        self.f.begin_object_key(self.w, true)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "kind")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.f.write_u32(self.w, 27)?;
        self.f.end_object_value(self.w)?;
        self.f.begin_array(self.w)?;
        self.f.begin_array_value(self.w, true)?;
        write!(self.w, "{}", to_json_string(&item0)?)?;
        self.f.end_array_value(self.w)?;
        self.f.begin_array_value(self.w, false)?;
        write!(self.w, "{}", to_json_string(&item1)?)?;
        self.f.end_array_value(self.w)?;
        self.f.end_array(self.w)?;
        self.f.end_object(self.w)?;
      }
      Token::Num(item0) => {
        self.f.begin_object(self.w)?;
        self.f.begin_object_key(self.w, true)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "kind")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.f.write_u32(self.w, 28)?;
        self.f.end_object_value(self.w)?;
        let value = &item0;
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "inner")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        write!(self.w, "{}", to_json_string(value)?)?;
        self.f.end_object_value(self.w)?;
        self.f.end_object(self.w)?;
      }
      Token::BigInt(item0) => {
        self.f.begin_object(self.w)?;
        self.f.begin_object_key(self.w, true)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "kind")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.f.write_u32(self.w, 29)?;
        self.f.end_object_value(self.w)?;
        let value = &item0;
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "inner")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        write!(self.w, "{}", to_json_string(value)?)?;
        self.f.end_object_value(self.w)?;
        self.f.end_object(self.w)?;
      }
      Token::JSXName {
        name,
      } => {
        self.f.begin_object(self.w)?;
        self.f.begin_object_key(self.w, true)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "kind")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.f.write_u32(self.w, 30)?;
        self.f.end_object_value(self.w)?;
        let value = &name;
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "name")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        write!(self.w, "{}", to_json_string(value)?)?;
        self.f.end_object_value(self.w)?;
        self.f.end_object(self.w)?;
      }
      Token::JSXText {
        raw,
      } => {
        self.f.begin_object(self.w)?;
        self.f.begin_object_key(self.w, true)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "kind")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.f.write_u32(self.w, 31)?;
        self.f.end_object_value(self.w)?;
        let value = &raw;
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "raw")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        write!(self.w, "{}", to_json_string(value)?)?;
        self.f.end_object_value(self.w)?;
        self.f.end_object(self.w)?;
      }
      Token::JSXTagStart => self.f.write_u32(self.w, 32)?,
      Token::JSXTagEnd => self.f.write_u32(self.w, 33)?,
      Token::Shebang(item0) => {
        self.f.begin_object(self.w)?;
        self.f.begin_object_key(self.w, true)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "kind")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.f.write_u32(self.w, 34)?;
        self.f.end_object_value(self.w)?;
        let value = &item0;
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "inner")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        write!(self.w, "{}", to_json_string(value)?)?;
        self.f.end_object_value(self.w)?;
        self.f.end_object(self.w)?;
      }
      Token::Error(_) => panic!("Serializing an AST containing an Error is not currently supported."),
    }
    Ok(())
  }

  fn serialize_word(&mut self, value: &Word) -> Result<(), Error> {
    match value {
      Word::Keyword(item0) => {
        self.f.begin_object(self.w)?;
        self.f.begin_object_key(self.w, true)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "kind")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.f.write_u32(self.w, 0)?;
        self.f.end_object_value(self.w)?;
        let value = &item0;
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "inner")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.serialize_keyword(value)?;
        self.f.end_object_value(self.w)?;
        self.f.end_object(self.w)?;
      }
      Word::Null => self.f.write_u32(self.w, 1)?,
      Word::True => self.f.write_u32(self.w, 2)?,
      Word::False => self.f.write_u32(self.w, 3)?,
      Word::Ident(item0) => {
        self.f.begin_object(self.w)?;
        self.f.begin_object_key(self.w, true)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "kind")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        self.f.write_u32(self.w, 4)?;
        self.f.end_object_value(self.w)?;
        let value = &item0;
        self.f.begin_object_key(self.w, false)?;
        self.f.begin_string(self.w)?;
        self.f.write_string_fragment(self.w, "inner")?;
        self.f.end_string(self.w)?;
        self.f.end_object_key(self.w)?;
        self.f.begin_object_value(self.w)?;
        write!(self.w, "{}", to_json_string(value)?)?;
        self.f.end_object_value(self.w)?;
        self.f.end_object(self.w)?;
      }
    }
    Ok(())
  }


  pub fn serialize_comments(&mut self, leading: &SingleThreadedCommentsMapInner, trailing: &SingleThreadedCommentsMapInner) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.f.begin_object_key(self.w, true)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "leading")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_object(self.w)?;
    for (i, (key, value)) in leading.iter().enumerate() {
      self.f.begin_object_key(self.w, i == 0)?;
      self.f.begin_string(self.w)?;
      self.f.write_string_fragment(self.w, &key.0.to_string())?;
      self.f.end_string(self.w)?;
      self.f.end_object_key(self.w)?;
      self.f.begin_object_value(self.w)?;
      self.serialize_comment_vec(value)?;
      self.f.end_object_value(self.w)?;
    }
    self.f.end_object(self.w)?;
    self.f.end_object_value(self.w)?;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "trailing")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_object(self.w)?;
    for (i, (key, value)) in trailing.iter().enumerate() {
      self.f.begin_object_key(self.w, i == 0)?;
      self.f.begin_string(self.w)?;
      self.f.write_string_fragment(self.w, &key.0.to_string())?;
      self.f.end_string(self.w)?;
      self.f.end_object_key(self.w)?;
      self.f.begin_object_value(self.w)?;
      self.serialize_comment_vec(value)?;
      self.f.end_object_value(self.w)?;
    }
    self.f.end_object(self.w)?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  pub fn serialize_comment_vec(&mut self, comments: &Vec<Comment>) -> Result<(), Error> {
    self.f.begin_array(self.w)?;
    for (i, comment) in comments.iter().enumerate() {
      self.f.begin_array_value(self.w, i == 0)?;
      self.serialize_comment(comment)?;
      self.f.end_array_value(self.w)?;
    }
    self.f.end_array(self.w)?;
    Ok(())
  }

  pub fn serialize_comment(&mut self, comment: &Comment) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.serialize_span_props(&comment.span, true)?;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "text")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, &comment.text)?;
    self.f.end_string(self.w)?;
    self.f.end_object_value(self.w)?;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "kind")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, match comment.kind {
      CommentKind::Line => 0,
      CommentKind::Block => 1,
    })?;
    self.f.end_object_value(self.w)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  fn serialize_span(&mut self, span: &Span) -> Result<(), Error> {
    self.f.begin_object(self.w)?;
    self.serialize_span_props(span, true)?;
    self.f.end_object(self.w)?;
    Ok(())
  }

  fn serialize_span_props(&mut self, span: &Span, is_first_prop: bool) -> Result<(), Error> {
    self.f.begin_object_key(self.w, is_first_prop)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "start")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, byte_pos_to_char_pos(&self.multi_byte_chars, span.lo()))?;
    self.f.end_object_value(self.w)?;
    self.f.begin_object_key(self.w, false)?;
    self.f.begin_string(self.w)?;
    self.f.write_string_fragment(self.w, "end")?;
    self.f.end_string(self.w)?;
    self.f.end_object_key(self.w)?;
    self.f.begin_object_value(self.w)?;
    self.f.write_u32(self.w, byte_pos_to_char_pos(&self.multi_byte_chars, span.hi()))?;
    self.f.end_object_value(self.w)?;
    Ok(())
  }
}
