// This code is code generated.
// Run `./scripts/generate.sh` from the root directory to regenerate it.
use std::marker::PhantomData;
use serde::Serialize;
use swc_common::{Span, Spanned};
use crate::generated::*;

#[derive(Serialize)]
#[serde(rename = "ArrayLit", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableArrayLit<'a> {
  span: Span,
  elems: Vec<Option<&'a ExprOrSpread<'a>>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ArrayLit<'a>> for SerializableArrayLit<'a> {
  fn from(orig: ArrayLit<'a>) -> Self {
    Self {
      span: orig.span(),
      elems: orig.elems,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ArrayPat", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableArrayPat<'a> {
  span: Span,
  optional: bool,
  elems: Vec<Option<Pat<'a>>>,
  type_ann: Option<&'a TsTypeAnn<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ArrayPat<'a>> for SerializableArrayPat<'a> {
  fn from(orig: ArrayPat<'a>) -> Self {
    Self {
      span: orig.span(),
      optional: orig.optional().clone(),
      elems: orig.elems,
      type_ann: orig.type_ann,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ArrowExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableArrowExpr<'a> {
  span: Span,
  is_async: bool,
  is_generator: bool,
  params: Vec<Pat<'a>>,
  body: BlockStmtOrExpr<'a>,
  type_params: Option<&'a TsTypeParamDecl<'a>>,
  return_type: Option<&'a TsTypeAnn<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ArrowExpr<'a>> for SerializableArrowExpr<'a> {
  fn from(orig: ArrowExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      is_async: orig.is_async().clone(),
      is_generator: orig.is_generator().clone(),
      params: orig.params,
      body: orig.body,
      type_params: orig.type_params,
      return_type: orig.return_type,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "AssignExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableAssignExpr<'a> {
  span: Span,
  op: AssignOp,
  left: PatOrExpr<'a>,
  right: Expr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<AssignExpr<'a>> for SerializableAssignExpr<'a> {
  fn from(orig: AssignExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      op: orig.op().clone(),
      left: orig.left,
      right: orig.right,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "AssignPat", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableAssignPat<'a> {
  span: Span,
  left: Pat<'a>,
  right: Expr<'a>,
  type_ann: Option<&'a TsTypeAnn<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<AssignPat<'a>> for SerializableAssignPat<'a> {
  fn from(orig: AssignPat<'a>) -> Self {
    Self {
      span: orig.span(),
      left: orig.left,
      right: orig.right,
      type_ann: orig.type_ann,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "AssignPatProp", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableAssignPatProp<'a> {
  span: Span,
  key: &'a Ident<'a>,
  value: Option<Expr<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<AssignPatProp<'a>> for SerializableAssignPatProp<'a> {
  fn from(orig: AssignPatProp<'a>) -> Self {
    Self {
      span: orig.span(),
      key: orig.key,
      value: orig.value,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "AssignProp", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableAssignProp<'a> {
  span: Span,
  key: &'a Ident<'a>,
  value: Expr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<AssignProp<'a>> for SerializableAssignProp<'a> {
  fn from(orig: AssignProp<'a>) -> Self {
    Self {
      span: orig.span(),
      key: orig.key,
      value: orig.value,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "AwaitExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableAwaitExpr<'a> {
  span: Span,
  arg: Expr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<AwaitExpr<'a>> for SerializableAwaitExpr<'a> {
  fn from(orig: AwaitExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      arg: orig.arg,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "BigInt", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableBigInt<'a> {
  span: Span,
  value: num_bigint::BigInt,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<BigInt<'a>> for SerializableBigInt<'a> {
  fn from(orig: BigInt<'a>) -> Self {
    Self {
      span: orig.span(),
      value: orig.value().clone(),
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "BinExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableBinExpr<'a> {
  span: Span,
  op: BinaryOp,
  left: Expr<'a>,
  right: Expr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<BinExpr<'a>> for SerializableBinExpr<'a> {
  fn from(orig: BinExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      op: orig.op().clone(),
      left: orig.left,
      right: orig.right,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "BindingIdent", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableBindingIdent<'a> {
  span: Span,
  id: &'a Ident<'a>,
  type_ann: Option<&'a TsTypeAnn<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<BindingIdent<'a>> for SerializableBindingIdent<'a> {
  fn from(orig: BindingIdent<'a>) -> Self {
    Self {
      span: orig.span(),
      id: orig.id,
      type_ann: orig.type_ann,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "BlockStmt", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableBlockStmt<'a> {
  span: Span,
  stmts: Vec<Stmt<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<BlockStmt<'a>> for SerializableBlockStmt<'a> {
  fn from(orig: BlockStmt<'a>) -> Self {
    Self {
      span: orig.span(),
      stmts: orig.stmts,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "Bool", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableBool<'a> {
  span: Span,
  value: bool,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<Bool<'a>> for SerializableBool<'a> {
  fn from(orig: Bool<'a>) -> Self {
    Self {
      span: orig.span(),
      value: orig.value().clone(),
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "BreakStmt", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableBreakStmt<'a> {
  span: Span,
  label: Option<&'a Ident<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<BreakStmt<'a>> for SerializableBreakStmt<'a> {
  fn from(orig: BreakStmt<'a>) -> Self {
    Self {
      span: orig.span(),
      label: orig.label,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "CallExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableCallExpr<'a> {
  span: Span,
  callee: ExprOrSuper<'a>,
  args: Vec<&'a ExprOrSpread<'a>>,
  type_args: Option<&'a TsTypeParamInstantiation<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<CallExpr<'a>> for SerializableCallExpr<'a> {
  fn from(orig: CallExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      callee: orig.callee,
      args: orig.args,
      type_args: orig.type_args,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "CatchClause", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableCatchClause<'a> {
  span: Span,
  param: Option<Pat<'a>>,
  body: &'a BlockStmt<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<CatchClause<'a>> for SerializableCatchClause<'a> {
  fn from(orig: CatchClause<'a>) -> Self {
    Self {
      span: orig.span(),
      param: orig.param,
      body: orig.body,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "Class", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableClass<'a> {
  span: Span,
  is_abstract: bool,
  decorators: Vec<&'a Decorator<'a>>,
  body: Vec<ClassMember<'a>>,
  super_class: Option<Expr<'a>>,
  type_params: Option<&'a TsTypeParamDecl<'a>>,
  super_type_params: Option<&'a TsTypeParamInstantiation<'a>>,
  implements: Vec<&'a TsExprWithTypeArgs<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<Class<'a>> for SerializableClass<'a> {
  fn from(orig: Class<'a>) -> Self {
    Self {
      span: orig.span(),
      is_abstract: orig.is_abstract().clone(),
      decorators: orig.decorators,
      body: orig.body,
      super_class: orig.super_class,
      type_params: orig.type_params,
      super_type_params: orig.super_type_params,
      implements: orig.implements,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ClassDecl", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableClassDecl<'a> {
  span: Span,
  declare: bool,
  ident: &'a Ident<'a>,
  class: &'a Class<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ClassDecl<'a>> for SerializableClassDecl<'a> {
  fn from(orig: ClassDecl<'a>) -> Self {
    Self {
      span: orig.span(),
      declare: orig.declare().clone(),
      ident: orig.ident,
      class: orig.class,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ClassExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableClassExpr<'a> {
  span: Span,
  ident: Option<&'a Ident<'a>>,
  class: &'a Class<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ClassExpr<'a>> for SerializableClassExpr<'a> {
  fn from(orig: ClassExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      ident: orig.ident,
      class: orig.class,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ClassMethod", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableClassMethod<'a> {
  span: Span,
  kind: MethodKind,
  is_static: bool,
  accessibility: Option<Accessibility>,
  is_abstract: bool,
  is_optional: bool,
  key: PropName<'a>,
  function: &'a Function<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ClassMethod<'a>> for SerializableClassMethod<'a> {
  fn from(orig: ClassMethod<'a>) -> Self {
    Self {
      span: orig.span(),
      kind: orig.kind().clone(),
      is_static: orig.is_static().clone(),
      accessibility: orig.accessibility().clone(),
      is_abstract: orig.is_abstract().clone(),
      is_optional: orig.is_optional().clone(),
      key: orig.key,
      function: orig.function,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ClassProp", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableClassProp<'a> {
  span: Span,
  is_static: bool,
  computed: bool,
  accessibility: Option<Accessibility>,
  is_abstract: bool,
  is_optional: bool,
  readonly: bool,
  declare: bool,
  definite: bool,
  key: Expr<'a>,
  value: Option<Expr<'a>>,
  type_ann: Option<&'a TsTypeAnn<'a>>,
  decorators: Vec<&'a Decorator<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ClassProp<'a>> for SerializableClassProp<'a> {
  fn from(orig: ClassProp<'a>) -> Self {
    Self {
      span: orig.span(),
      is_static: orig.is_static().clone(),
      computed: orig.computed().clone(),
      accessibility: orig.accessibility().clone(),
      is_abstract: orig.is_abstract().clone(),
      is_optional: orig.is_optional().clone(),
      readonly: orig.readonly().clone(),
      declare: orig.declare().clone(),
      definite: orig.definite().clone(),
      key: orig.key,
      value: orig.value,
      type_ann: orig.type_ann,
      decorators: orig.decorators,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ComputedPropName", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableComputedPropName<'a> {
  span: Span,
  expr: Expr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ComputedPropName<'a>> for SerializableComputedPropName<'a> {
  fn from(orig: ComputedPropName<'a>) -> Self {
    Self {
      span: orig.span(),
      expr: orig.expr,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "CondExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableCondExpr<'a> {
  span: Span,
  test: Expr<'a>,
  cons: Expr<'a>,
  alt: Expr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<CondExpr<'a>> for SerializableCondExpr<'a> {
  fn from(orig: CondExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      test: orig.test,
      cons: orig.cons,
      alt: orig.alt,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "Constructor", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableConstructor<'a> {
  span: Span,
  accessibility: Option<Accessibility>,
  is_optional: bool,
  key: PropName<'a>,
  params: Vec<ParamOrTsParamProp<'a>>,
  body: Option<&'a BlockStmt<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<Constructor<'a>> for SerializableConstructor<'a> {
  fn from(orig: Constructor<'a>) -> Self {
    Self {
      span: orig.span(),
      accessibility: orig.accessibility().clone(),
      is_optional: orig.is_optional().clone(),
      key: orig.key,
      params: orig.params,
      body: orig.body,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ContinueStmt", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableContinueStmt<'a> {
  span: Span,
  label: Option<&'a Ident<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ContinueStmt<'a>> for SerializableContinueStmt<'a> {
  fn from(orig: ContinueStmt<'a>) -> Self {
    Self {
      span: orig.span(),
      label: orig.label,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "DebuggerStmt", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableDebuggerStmt<'a> {
  span: Span,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<DebuggerStmt<'a>> for SerializableDebuggerStmt<'a> {
  fn from(orig: DebuggerStmt<'a>) -> Self {
    Self {
      span: orig.span(),
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "Decorator", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableDecorator<'a> {
  span: Span,
  expr: Expr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<Decorator<'a>> for SerializableDecorator<'a> {
  fn from(orig: Decorator<'a>) -> Self {
    Self {
      span: orig.span(),
      expr: orig.expr,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "DoWhileStmt", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableDoWhileStmt<'a> {
  span: Span,
  test: Expr<'a>,
  body: Stmt<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<DoWhileStmt<'a>> for SerializableDoWhileStmt<'a> {
  fn from(orig: DoWhileStmt<'a>) -> Self {
    Self {
      span: orig.span(),
      test: orig.test,
      body: orig.body,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "EmptyStmt", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableEmptyStmt<'a> {
  span: Span,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<EmptyStmt<'a>> for SerializableEmptyStmt<'a> {
  fn from(orig: EmptyStmt<'a>) -> Self {
    Self {
      span: orig.span(),
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ExportAll", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableExportAll<'a> {
  span: Span,
  src: &'a Str<'a>,
  asserts: Option<&'a ObjectLit<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ExportAll<'a>> for SerializableExportAll<'a> {
  fn from(orig: ExportAll<'a>) -> Self {
    Self {
      span: orig.span(),
      src: orig.src,
      asserts: orig.asserts,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ExportDecl", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableExportDecl<'a> {
  span: Span,
  decl: Decl<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ExportDecl<'a>> for SerializableExportDecl<'a> {
  fn from(orig: ExportDecl<'a>) -> Self {
    Self {
      span: orig.span(),
      decl: orig.decl,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ExportDefaultDecl", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableExportDefaultDecl<'a> {
  span: Span,
  decl: DefaultDecl<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ExportDefaultDecl<'a>> for SerializableExportDefaultDecl<'a> {
  fn from(orig: ExportDefaultDecl<'a>) -> Self {
    Self {
      span: orig.span(),
      decl: orig.decl,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ExportDefaultExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableExportDefaultExpr<'a> {
  span: Span,
  expr: Expr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ExportDefaultExpr<'a>> for SerializableExportDefaultExpr<'a> {
  fn from(orig: ExportDefaultExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      expr: orig.expr,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ExportDefaultSpecifier", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableExportDefaultSpecifier<'a> {
  span: Span,
  exported: &'a Ident<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ExportDefaultSpecifier<'a>> for SerializableExportDefaultSpecifier<'a> {
  fn from(orig: ExportDefaultSpecifier<'a>) -> Self {
    Self {
      span: orig.span(),
      exported: orig.exported,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ExportNamedSpecifier", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableExportNamedSpecifier<'a> {
  span: Span,
  orig: &'a Ident<'a>,
  exported: Option<&'a Ident<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ExportNamedSpecifier<'a>> for SerializableExportNamedSpecifier<'a> {
  fn from(orig: ExportNamedSpecifier<'a>) -> Self {
    Self {
      span: orig.span(),
      orig: orig.orig,
      exported: orig.exported,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ExportNamespaceSpecifier", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableExportNamespaceSpecifier<'a> {
  span: Span,
  name: &'a Ident<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ExportNamespaceSpecifier<'a>> for SerializableExportNamespaceSpecifier<'a> {
  fn from(orig: ExportNamespaceSpecifier<'a>) -> Self {
    Self {
      span: orig.span(),
      name: orig.name,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ExprOrSpread", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableExprOrSpread<'a> {
  span: Span,
  spread: Option<swc_common::Span>,
  expr: Expr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ExprOrSpread<'a>> for SerializableExprOrSpread<'a> {
  fn from(orig: ExprOrSpread<'a>) -> Self {
    Self {
      span: orig.span(),
      spread: orig.spread().clone(),
      expr: orig.expr,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ExprStmt", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableExprStmt<'a> {
  span: Span,
  expr: Expr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ExprStmt<'a>> for SerializableExprStmt<'a> {
  fn from(orig: ExprStmt<'a>) -> Self {
    Self {
      span: orig.span(),
      expr: orig.expr,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "FnDecl", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableFnDecl<'a> {
  span: Span,
  declare: bool,
  ident: &'a Ident<'a>,
  function: &'a Function<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<FnDecl<'a>> for SerializableFnDecl<'a> {
  fn from(orig: FnDecl<'a>) -> Self {
    Self {
      span: orig.span(),
      declare: orig.declare().clone(),
      ident: orig.ident,
      function: orig.function,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "FnExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableFnExpr<'a> {
  span: Span,
  ident: Option<&'a Ident<'a>>,
  function: &'a Function<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<FnExpr<'a>> for SerializableFnExpr<'a> {
  fn from(orig: FnExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      ident: orig.ident,
      function: orig.function,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ForInStmt", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableForInStmt<'a> {
  span: Span,
  left: VarDeclOrPat<'a>,
  right: Expr<'a>,
  body: Stmt<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ForInStmt<'a>> for SerializableForInStmt<'a> {
  fn from(orig: ForInStmt<'a>) -> Self {
    Self {
      span: orig.span(),
      left: orig.left,
      right: orig.right,
      body: orig.body,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ForOfStmt", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableForOfStmt<'a> {
  span: Span,
  await_token: Option<swc_common::Span>,
  left: VarDeclOrPat<'a>,
  right: Expr<'a>,
  body: Stmt<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ForOfStmt<'a>> for SerializableForOfStmt<'a> {
  fn from(orig: ForOfStmt<'a>) -> Self {
    Self {
      span: orig.span(),
      await_token: orig.await_token().clone(),
      left: orig.left,
      right: orig.right,
      body: orig.body,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ForStmt", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableForStmt<'a> {
  span: Span,
  init: Option<VarDeclOrExpr<'a>>,
  test: Option<Expr<'a>>,
  update: Option<Expr<'a>>,
  body: Stmt<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ForStmt<'a>> for SerializableForStmt<'a> {
  fn from(orig: ForStmt<'a>) -> Self {
    Self {
      span: orig.span(),
      init: orig.init,
      test: orig.test,
      update: orig.update,
      body: orig.body,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "Function", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableFunction<'a> {
  span: Span,
  is_generator: bool,
  is_async: bool,
  params: Vec<&'a Param<'a>>,
  decorators: Vec<&'a Decorator<'a>>,
  body: Option<&'a BlockStmt<'a>>,
  type_params: Option<&'a TsTypeParamDecl<'a>>,
  return_type: Option<&'a TsTypeAnn<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<Function<'a>> for SerializableFunction<'a> {
  fn from(orig: Function<'a>) -> Self {
    Self {
      span: orig.span(),
      is_generator: orig.is_generator().clone(),
      is_async: orig.is_async().clone(),
      params: orig.params,
      decorators: orig.decorators,
      body: orig.body,
      type_params: orig.type_params,
      return_type: orig.return_type,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "GetterProp", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableGetterProp<'a> {
  span: Span,
  key: PropName<'a>,
  type_ann: Option<&'a TsTypeAnn<'a>>,
  body: Option<&'a BlockStmt<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<GetterProp<'a>> for SerializableGetterProp<'a> {
  fn from(orig: GetterProp<'a>) -> Self {
    Self {
      span: orig.span(),
      key: orig.key,
      type_ann: orig.type_ann,
      body: orig.body,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "Ident", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableIdent<'a> {
  span: Span,
  sym: swc_atoms::JsWord,
  optional: bool,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<Ident<'a>> for SerializableIdent<'a> {
  fn from(orig: Ident<'a>) -> Self {
    Self {
      span: orig.span(),
      sym: orig.sym().clone(),
      optional: orig.optional().clone(),
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "IfStmt", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableIfStmt<'a> {
  span: Span,
  test: Expr<'a>,
  cons: Stmt<'a>,
  alt: Option<Stmt<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<IfStmt<'a>> for SerializableIfStmt<'a> {
  fn from(orig: IfStmt<'a>) -> Self {
    Self {
      span: orig.span(),
      test: orig.test,
      cons: orig.cons,
      alt: orig.alt,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ImportDecl", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableImportDecl<'a> {
  span: Span,
  type_only: bool,
  specifiers: Vec<ImportSpecifier<'a>>,
  src: &'a Str<'a>,
  asserts: Option<&'a ObjectLit<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ImportDecl<'a>> for SerializableImportDecl<'a> {
  fn from(orig: ImportDecl<'a>) -> Self {
    Self {
      span: orig.span(),
      type_only: orig.type_only().clone(),
      specifiers: orig.specifiers,
      src: orig.src,
      asserts: orig.asserts,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ImportDefaultSpecifier", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableImportDefaultSpecifier<'a> {
  span: Span,
  local: &'a Ident<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ImportDefaultSpecifier<'a>> for SerializableImportDefaultSpecifier<'a> {
  fn from(orig: ImportDefaultSpecifier<'a>) -> Self {
    Self {
      span: orig.span(),
      local: orig.local,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ImportNamedSpecifier", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableImportNamedSpecifier<'a> {
  span: Span,
  local: &'a Ident<'a>,
  imported: Option<&'a Ident<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ImportNamedSpecifier<'a>> for SerializableImportNamedSpecifier<'a> {
  fn from(orig: ImportNamedSpecifier<'a>) -> Self {
    Self {
      span: orig.span(),
      local: orig.local,
      imported: orig.imported,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ImportStarAsSpecifier", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableImportStarAsSpecifier<'a> {
  span: Span,
  local: &'a Ident<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ImportStarAsSpecifier<'a>> for SerializableImportStarAsSpecifier<'a> {
  fn from(orig: ImportStarAsSpecifier<'a>) -> Self {
    Self {
      span: orig.span(),
      local: orig.local,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "Invalid", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableInvalid<'a> {
  span: Span,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<Invalid<'a>> for SerializableInvalid<'a> {
  fn from(orig: Invalid<'a>) -> Self {
    Self {
      span: orig.span(),
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "JSXAttr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableJSXAttr<'a> {
  span: Span,
  name: JSXAttrName<'a>,
  value: Option<JSXAttrValue<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<JSXAttr<'a>> for SerializableJSXAttr<'a> {
  fn from(orig: JSXAttr<'a>) -> Self {
    Self {
      span: orig.span(),
      name: orig.name,
      value: orig.value,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "JSXClosingElement", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableJSXClosingElement<'a> {
  span: Span,
  name: JSXElementName<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<JSXClosingElement<'a>> for SerializableJSXClosingElement<'a> {
  fn from(orig: JSXClosingElement<'a>) -> Self {
    Self {
      span: orig.span(),
      name: orig.name,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "JSXClosingFragment", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableJSXClosingFragment<'a> {
  span: Span,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<JSXClosingFragment<'a>> for SerializableJSXClosingFragment<'a> {
  fn from(orig: JSXClosingFragment<'a>) -> Self {
    Self {
      span: orig.span(),
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "JSXElement", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableJSXElement<'a> {
  span: Span,
  opening: &'a JSXOpeningElement<'a>,
  children: Vec<JSXElementChild<'a>>,
  closing: Option<&'a JSXClosingElement<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<JSXElement<'a>> for SerializableJSXElement<'a> {
  fn from(orig: JSXElement<'a>) -> Self {
    Self {
      span: orig.span(),
      opening: orig.opening,
      children: orig.children,
      closing: orig.closing,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "JSXEmptyExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableJSXEmptyExpr<'a> {
  span: Span,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<JSXEmptyExpr<'a>> for SerializableJSXEmptyExpr<'a> {
  fn from(orig: JSXEmptyExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "JSXExprContainer", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableJSXExprContainer<'a> {
  span: Span,
  expr: JSXExpr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<JSXExprContainer<'a>> for SerializableJSXExprContainer<'a> {
  fn from(orig: JSXExprContainer<'a>) -> Self {
    Self {
      span: orig.span(),
      expr: orig.expr,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "JSXFragment", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableJSXFragment<'a> {
  span: Span,
  opening: &'a JSXOpeningFragment<'a>,
  children: Vec<JSXElementChild<'a>>,
  closing: &'a JSXClosingFragment<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<JSXFragment<'a>> for SerializableJSXFragment<'a> {
  fn from(orig: JSXFragment<'a>) -> Self {
    Self {
      span: orig.span(),
      opening: orig.opening,
      children: orig.children,
      closing: orig.closing,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "JSXMemberExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableJSXMemberExpr<'a> {
  span: Span,
  obj: JSXObject<'a>,
  prop: &'a Ident<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<JSXMemberExpr<'a>> for SerializableJSXMemberExpr<'a> {
  fn from(orig: JSXMemberExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      obj: orig.obj,
      prop: orig.prop,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "JSXNamespacedName", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableJSXNamespacedName<'a> {
  span: Span,
  ns: &'a Ident<'a>,
  name: &'a Ident<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<JSXNamespacedName<'a>> for SerializableJSXNamespacedName<'a> {
  fn from(orig: JSXNamespacedName<'a>) -> Self {
    Self {
      span: orig.span(),
      ns: orig.ns,
      name: orig.name,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "JSXOpeningElement", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableJSXOpeningElement<'a> {
  span: Span,
  self_closing: bool,
  name: JSXElementName<'a>,
  attrs: Vec<JSXAttrOrSpread<'a>>,
  type_args: Option<&'a TsTypeParamInstantiation<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<JSXOpeningElement<'a>> for SerializableJSXOpeningElement<'a> {
  fn from(orig: JSXOpeningElement<'a>) -> Self {
    Self {
      span: orig.span(),
      self_closing: orig.self_closing().clone(),
      name: orig.name,
      attrs: orig.attrs,
      type_args: orig.type_args,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "JSXOpeningFragment", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableJSXOpeningFragment<'a> {
  span: Span,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<JSXOpeningFragment<'a>> for SerializableJSXOpeningFragment<'a> {
  fn from(orig: JSXOpeningFragment<'a>) -> Self {
    Self {
      span: orig.span(),
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "JSXSpreadChild", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableJSXSpreadChild<'a> {
  span: Span,
  expr: Expr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<JSXSpreadChild<'a>> for SerializableJSXSpreadChild<'a> {
  fn from(orig: JSXSpreadChild<'a>) -> Self {
    Self {
      span: orig.span(),
      expr: orig.expr,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "JSXText", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableJSXText<'a> {
  span: Span,
  value: swc_atoms::JsWord,
  raw: swc_atoms::JsWord,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<JSXText<'a>> for SerializableJSXText<'a> {
  fn from(orig: JSXText<'a>) -> Self {
    Self {
      span: orig.span(),
      value: orig.value().clone(),
      raw: orig.raw().clone(),
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "KeyValuePatProp", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableKeyValuePatProp<'a> {
  span: Span,
  key: PropName<'a>,
  value: Pat<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<KeyValuePatProp<'a>> for SerializableKeyValuePatProp<'a> {
  fn from(orig: KeyValuePatProp<'a>) -> Self {
    Self {
      span: orig.span(),
      key: orig.key,
      value: orig.value,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "KeyValueProp", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableKeyValueProp<'a> {
  span: Span,
  key: PropName<'a>,
  value: Expr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<KeyValueProp<'a>> for SerializableKeyValueProp<'a> {
  fn from(orig: KeyValueProp<'a>) -> Self {
    Self {
      span: orig.span(),
      key: orig.key,
      value: orig.value,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "LabeledStmt", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableLabeledStmt<'a> {
  span: Span,
  label: &'a Ident<'a>,
  body: Stmt<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<LabeledStmt<'a>> for SerializableLabeledStmt<'a> {
  fn from(orig: LabeledStmt<'a>) -> Self {
    Self {
      span: orig.span(),
      label: orig.label,
      body: orig.body,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "MemberExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableMemberExpr<'a> {
  span: Span,
  computed: bool,
  obj: ExprOrSuper<'a>,
  prop: Expr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<MemberExpr<'a>> for SerializableMemberExpr<'a> {
  fn from(orig: MemberExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      computed: orig.computed().clone(),
      obj: orig.obj,
      prop: orig.prop,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "MetaPropExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableMetaPropExpr<'a> {
  span: Span,
  meta: &'a Ident<'a>,
  prop: &'a Ident<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<MetaPropExpr<'a>> for SerializableMetaPropExpr<'a> {
  fn from(orig: MetaPropExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      meta: orig.meta,
      prop: orig.prop,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "MethodProp", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableMethodProp<'a> {
  span: Span,
  key: PropName<'a>,
  function: &'a Function<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<MethodProp<'a>> for SerializableMethodProp<'a> {
  fn from(orig: MethodProp<'a>) -> Self {
    Self {
      span: orig.span(),
      key: orig.key,
      function: orig.function,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "Module", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableModule<'a> {
  span: Span,
  shebang: Option<swc_atoms::JsWord>,
  body: Vec<ModuleItem<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<Module<'a>> for SerializableModule<'a> {
  fn from(orig: Module<'a>) -> Self {
    Self {
      span: orig.span(),
      shebang: orig.shebang().clone(),
      body: orig.body,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "NamedExport", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableNamedExport<'a> {
  span: Span,
  type_only: bool,
  specifiers: Vec<ExportSpecifier<'a>>,
  src: Option<&'a Str<'a>>,
  asserts: Option<&'a ObjectLit<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<NamedExport<'a>> for SerializableNamedExport<'a> {
  fn from(orig: NamedExport<'a>) -> Self {
    Self {
      span: orig.span(),
      type_only: orig.type_only().clone(),
      specifiers: orig.specifiers,
      src: orig.src,
      asserts: orig.asserts,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "NewExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableNewExpr<'a> {
  span: Span,
  callee: Expr<'a>,
  args: Option<Vec<&'a ExprOrSpread<'a>>>,
  type_args: Option<&'a TsTypeParamInstantiation<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<NewExpr<'a>> for SerializableNewExpr<'a> {
  fn from(orig: NewExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      callee: orig.callee,
      args: orig.args,
      type_args: orig.type_args,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "Null", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableNull<'a> {
  span: Span,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<Null<'a>> for SerializableNull<'a> {
  fn from(orig: Null<'a>) -> Self {
    Self {
      span: orig.span(),
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "Number", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableNumber<'a> {
  span: Span,
  value: f64,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<Number<'a>> for SerializableNumber<'a> {
  fn from(orig: Number<'a>) -> Self {
    Self {
      span: orig.span(),
      value: orig.value().clone(),
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ObjectLit", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableObjectLit<'a> {
  span: Span,
  props: Vec<PropOrSpread<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ObjectLit<'a>> for SerializableObjectLit<'a> {
  fn from(orig: ObjectLit<'a>) -> Self {
    Self {
      span: orig.span(),
      props: orig.props,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ObjectPat", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableObjectPat<'a> {
  span: Span,
  optional: bool,
  props: Vec<ObjectPatProp<'a>>,
  type_ann: Option<&'a TsTypeAnn<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ObjectPat<'a>> for SerializableObjectPat<'a> {
  fn from(orig: ObjectPat<'a>) -> Self {
    Self {
      span: orig.span(),
      optional: orig.optional().clone(),
      props: orig.props,
      type_ann: orig.type_ann,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "OptChainExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableOptChainExpr<'a> {
  span: Span,
  question_dot_token: swc_common::Span,
  expr: Expr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<OptChainExpr<'a>> for SerializableOptChainExpr<'a> {
  fn from(orig: OptChainExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      question_dot_token: orig.question_dot_token().clone(),
      expr: orig.expr,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "Param", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableParam<'a> {
  span: Span,
  decorators: Vec<&'a Decorator<'a>>,
  pat: Pat<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<Param<'a>> for SerializableParam<'a> {
  fn from(orig: Param<'a>) -> Self {
    Self {
      span: orig.span(),
      decorators: orig.decorators,
      pat: orig.pat,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ParenExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableParenExpr<'a> {
  span: Span,
  expr: Expr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ParenExpr<'a>> for SerializableParenExpr<'a> {
  fn from(orig: ParenExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      expr: orig.expr,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "PrivateMethod", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializablePrivateMethod<'a> {
  span: Span,
  kind: MethodKind,
  is_static: bool,
  accessibility: Option<Accessibility>,
  is_abstract: bool,
  is_optional: bool,
  key: &'a PrivateName<'a>,
  function: &'a Function<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<PrivateMethod<'a>> for SerializablePrivateMethod<'a> {
  fn from(orig: PrivateMethod<'a>) -> Self {
    Self {
      span: orig.span(),
      kind: orig.kind().clone(),
      is_static: orig.is_static().clone(),
      accessibility: orig.accessibility().clone(),
      is_abstract: orig.is_abstract().clone(),
      is_optional: orig.is_optional().clone(),
      key: orig.key,
      function: orig.function,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "PrivateName", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializablePrivateName<'a> {
  span: Span,
  id: &'a Ident<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<PrivateName<'a>> for SerializablePrivateName<'a> {
  fn from(orig: PrivateName<'a>) -> Self {
    Self {
      span: orig.span(),
      id: orig.id,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "PrivateProp", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializablePrivateProp<'a> {
  span: Span,
  is_static: bool,
  computed: bool,
  accessibility: Option<Accessibility>,
  is_abstract: bool,
  is_optional: bool,
  readonly: bool,
  definite: bool,
  key: &'a PrivateName<'a>,
  value: Option<Expr<'a>>,
  type_ann: Option<&'a TsTypeAnn<'a>>,
  decorators: Vec<&'a Decorator<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<PrivateProp<'a>> for SerializablePrivateProp<'a> {
  fn from(orig: PrivateProp<'a>) -> Self {
    Self {
      span: orig.span(),
      is_static: orig.is_static().clone(),
      computed: orig.computed().clone(),
      accessibility: orig.accessibility().clone(),
      is_abstract: orig.is_abstract().clone(),
      is_optional: orig.is_optional().clone(),
      readonly: orig.readonly().clone(),
      definite: orig.definite().clone(),
      key: orig.key,
      value: orig.value,
      type_ann: orig.type_ann,
      decorators: orig.decorators,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "Regex", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableRegex<'a> {
  span: Span,
  exp: swc_atoms::JsWord,
  flags: swc_atoms::JsWord,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<Regex<'a>> for SerializableRegex<'a> {
  fn from(orig: Regex<'a>) -> Self {
    Self {
      span: orig.span(),
      exp: orig.exp().clone(),
      flags: orig.flags().clone(),
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "RestPat", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableRestPat<'a> {
  span: Span,
  dot3_token: swc_common::Span,
  arg: Pat<'a>,
  type_ann: Option<&'a TsTypeAnn<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<RestPat<'a>> for SerializableRestPat<'a> {
  fn from(orig: RestPat<'a>) -> Self {
    Self {
      span: orig.span(),
      dot3_token: orig.dot3_token().clone(),
      arg: orig.arg,
      type_ann: orig.type_ann,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ReturnStmt", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableReturnStmt<'a> {
  span: Span,
  arg: Option<Expr<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ReturnStmt<'a>> for SerializableReturnStmt<'a> {
  fn from(orig: ReturnStmt<'a>) -> Self {
    Self {
      span: orig.span(),
      arg: orig.arg,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "Script", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableScript<'a> {
  span: Span,
  shebang: Option<swc_atoms::JsWord>,
  body: Vec<Stmt<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<Script<'a>> for SerializableScript<'a> {
  fn from(orig: Script<'a>) -> Self {
    Self {
      span: orig.span(),
      shebang: orig.shebang().clone(),
      body: orig.body,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "SeqExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableSeqExpr<'a> {
  span: Span,
  exprs: Vec<Expr<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<SeqExpr<'a>> for SerializableSeqExpr<'a> {
  fn from(orig: SeqExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      exprs: orig.exprs,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "SetterProp", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableSetterProp<'a> {
  span: Span,
  key: PropName<'a>,
  param: Pat<'a>,
  body: Option<&'a BlockStmt<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<SetterProp<'a>> for SerializableSetterProp<'a> {
  fn from(orig: SetterProp<'a>) -> Self {
    Self {
      span: orig.span(),
      key: orig.key,
      param: orig.param,
      body: orig.body,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "SpreadElement", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableSpreadElement<'a> {
  span: Span,
  dot3_token: swc_common::Span,
  expr: Expr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<SpreadElement<'a>> for SerializableSpreadElement<'a> {
  fn from(orig: SpreadElement<'a>) -> Self {
    Self {
      span: orig.span(),
      dot3_token: orig.dot3_token().clone(),
      expr: orig.expr,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "Str", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableStr<'a> {
  span: Span,
  value: swc_atoms::JsWord,
  has_escape: bool,
  kind: StrKind,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<Str<'a>> for SerializableStr<'a> {
  fn from(orig: Str<'a>) -> Self {
    Self {
      span: orig.span(),
      value: orig.value().clone(),
      has_escape: orig.has_escape().clone(),
      kind: orig.kind().clone(),
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "Super", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableSuper<'a> {
  span: Span,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<Super<'a>> for SerializableSuper<'a> {
  fn from(orig: Super<'a>) -> Self {
    Self {
      span: orig.span(),
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "SwitchCase", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableSwitchCase<'a> {
  span: Span,
  test: Option<Expr<'a>>,
  cons: Vec<Stmt<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<SwitchCase<'a>> for SerializableSwitchCase<'a> {
  fn from(orig: SwitchCase<'a>) -> Self {
    Self {
      span: orig.span(),
      test: orig.test,
      cons: orig.cons,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "SwitchStmt", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableSwitchStmt<'a> {
  span: Span,
  discriminant: Expr<'a>,
  cases: Vec<&'a SwitchCase<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<SwitchStmt<'a>> for SerializableSwitchStmt<'a> {
  fn from(orig: SwitchStmt<'a>) -> Self {
    Self {
      span: orig.span(),
      discriminant: orig.discriminant,
      cases: orig.cases,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TaggedTpl", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTaggedTpl<'a> {
  span: Span,
  tag: Expr<'a>,
  exprs: Vec<Expr<'a>>,
  quasis: Vec<&'a TplElement<'a>>,
  type_params: Option<&'a TsTypeParamInstantiation<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TaggedTpl<'a>> for SerializableTaggedTpl<'a> {
  fn from(orig: TaggedTpl<'a>) -> Self {
    Self {
      span: orig.span(),
      tag: orig.tag,
      exprs: orig.exprs,
      quasis: orig.quasis,
      type_params: orig.type_params,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ThisExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableThisExpr<'a> {
  span: Span,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ThisExpr<'a>> for SerializableThisExpr<'a> {
  fn from(orig: ThisExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "ThrowStmt", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableThrowStmt<'a> {
  span: Span,
  arg: Expr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<ThrowStmt<'a>> for SerializableThrowStmt<'a> {
  fn from(orig: ThrowStmt<'a>) -> Self {
    Self {
      span: orig.span(),
      arg: orig.arg,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "Tpl", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTpl<'a> {
  span: Span,
  exprs: Vec<Expr<'a>>,
  quasis: Vec<&'a TplElement<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<Tpl<'a>> for SerializableTpl<'a> {
  fn from(orig: Tpl<'a>) -> Self {
    Self {
      span: orig.span(),
      exprs: orig.exprs,
      quasis: orig.quasis,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TplElement", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTplElement<'a> {
  span: Span,
  tail: bool,
  cooked: Option<&'a Str<'a>>,
  raw: &'a Str<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TplElement<'a>> for SerializableTplElement<'a> {
  fn from(orig: TplElement<'a>) -> Self {
    Self {
      span: orig.span(),
      tail: orig.tail().clone(),
      cooked: orig.cooked,
      raw: orig.raw,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TryStmt", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTryStmt<'a> {
  span: Span,
  block: &'a BlockStmt<'a>,
  handler: Option<&'a CatchClause<'a>>,
  finalizer: Option<&'a BlockStmt<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TryStmt<'a>> for SerializableTryStmt<'a> {
  fn from(orig: TryStmt<'a>) -> Self {
    Self {
      span: orig.span(),
      block: orig.block,
      handler: orig.handler,
      finalizer: orig.finalizer,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsArrayType", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsArrayType<'a> {
  span: Span,
  elem_type: TsType<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsArrayType<'a>> for SerializableTsArrayType<'a> {
  fn from(orig: TsArrayType<'a>) -> Self {
    Self {
      span: orig.span(),
      elem_type: orig.elem_type,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsAsExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsAsExpr<'a> {
  span: Span,
  expr: Expr<'a>,
  type_ann: TsType<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsAsExpr<'a>> for SerializableTsAsExpr<'a> {
  fn from(orig: TsAsExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      expr: orig.expr,
      type_ann: orig.type_ann,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsCallSignatureDecl", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsCallSignatureDecl<'a> {
  span: Span,
  params: Vec<TsFnParam<'a>>,
  type_ann: Option<&'a TsTypeAnn<'a>>,
  type_params: Option<&'a TsTypeParamDecl<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsCallSignatureDecl<'a>> for SerializableTsCallSignatureDecl<'a> {
  fn from(orig: TsCallSignatureDecl<'a>) -> Self {
    Self {
      span: orig.span(),
      params: orig.params,
      type_ann: orig.type_ann,
      type_params: orig.type_params,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsConditionalType", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsConditionalType<'a> {
  span: Span,
  check_type: TsType<'a>,
  extends_type: TsType<'a>,
  true_type: TsType<'a>,
  false_type: TsType<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsConditionalType<'a>> for SerializableTsConditionalType<'a> {
  fn from(orig: TsConditionalType<'a>) -> Self {
    Self {
      span: orig.span(),
      check_type: orig.check_type,
      extends_type: orig.extends_type,
      true_type: orig.true_type,
      false_type: orig.false_type,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsConstAssertion", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsConstAssertion<'a> {
  span: Span,
  expr: Expr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsConstAssertion<'a>> for SerializableTsConstAssertion<'a> {
  fn from(orig: TsConstAssertion<'a>) -> Self {
    Self {
      span: orig.span(),
      expr: orig.expr,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsConstructSignatureDecl", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsConstructSignatureDecl<'a> {
  span: Span,
  params: Vec<TsFnParam<'a>>,
  type_ann: Option<&'a TsTypeAnn<'a>>,
  type_params: Option<&'a TsTypeParamDecl<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsConstructSignatureDecl<'a>> for SerializableTsConstructSignatureDecl<'a> {
  fn from(orig: TsConstructSignatureDecl<'a>) -> Self {
    Self {
      span: orig.span(),
      params: orig.params,
      type_ann: orig.type_ann,
      type_params: orig.type_params,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsConstructorType", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsConstructorType<'a> {
  span: Span,
  is_abstract: bool,
  params: Vec<TsFnParam<'a>>,
  type_params: Option<&'a TsTypeParamDecl<'a>>,
  type_ann: &'a TsTypeAnn<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsConstructorType<'a>> for SerializableTsConstructorType<'a> {
  fn from(orig: TsConstructorType<'a>) -> Self {
    Self {
      span: orig.span(),
      is_abstract: orig.is_abstract().clone(),
      params: orig.params,
      type_params: orig.type_params,
      type_ann: orig.type_ann,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsEnumDecl", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsEnumDecl<'a> {
  span: Span,
  declare: bool,
  is_const: bool,
  id: &'a Ident<'a>,
  members: Vec<&'a TsEnumMember<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsEnumDecl<'a>> for SerializableTsEnumDecl<'a> {
  fn from(orig: TsEnumDecl<'a>) -> Self {
    Self {
      span: orig.span(),
      declare: orig.declare().clone(),
      is_const: orig.is_const().clone(),
      id: orig.id,
      members: orig.members,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsEnumMember", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsEnumMember<'a> {
  span: Span,
  id: TsEnumMemberId<'a>,
  init: Option<Expr<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsEnumMember<'a>> for SerializableTsEnumMember<'a> {
  fn from(orig: TsEnumMember<'a>) -> Self {
    Self {
      span: orig.span(),
      id: orig.id,
      init: orig.init,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsExportAssignment", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsExportAssignment<'a> {
  span: Span,
  expr: Expr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsExportAssignment<'a>> for SerializableTsExportAssignment<'a> {
  fn from(orig: TsExportAssignment<'a>) -> Self {
    Self {
      span: orig.span(),
      expr: orig.expr,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsExprWithTypeArgs", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsExprWithTypeArgs<'a> {
  span: Span,
  expr: TsEntityName<'a>,
  type_args: Option<&'a TsTypeParamInstantiation<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsExprWithTypeArgs<'a>> for SerializableTsExprWithTypeArgs<'a> {
  fn from(orig: TsExprWithTypeArgs<'a>) -> Self {
    Self {
      span: orig.span(),
      expr: orig.expr,
      type_args: orig.type_args,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsExternalModuleRef", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsExternalModuleRef<'a> {
  span: Span,
  expr: &'a Str<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsExternalModuleRef<'a>> for SerializableTsExternalModuleRef<'a> {
  fn from(orig: TsExternalModuleRef<'a>) -> Self {
    Self {
      span: orig.span(),
      expr: orig.expr,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsFnType", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsFnType<'a> {
  span: Span,
  params: Vec<TsFnParam<'a>>,
  type_params: Option<&'a TsTypeParamDecl<'a>>,
  type_ann: &'a TsTypeAnn<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsFnType<'a>> for SerializableTsFnType<'a> {
  fn from(orig: TsFnType<'a>) -> Self {
    Self {
      span: orig.span(),
      params: orig.params,
      type_params: orig.type_params,
      type_ann: orig.type_ann,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsImportEqualsDecl", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsImportEqualsDecl<'a> {
  span: Span,
  declare: bool,
  is_export: bool,
  id: &'a Ident<'a>,
  module_ref: TsModuleRef<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsImportEqualsDecl<'a>> for SerializableTsImportEqualsDecl<'a> {
  fn from(orig: TsImportEqualsDecl<'a>) -> Self {
    Self {
      span: orig.span(),
      declare: orig.declare().clone(),
      is_export: orig.is_export().clone(),
      id: orig.id,
      module_ref: orig.module_ref,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsImportType", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsImportType<'a> {
  span: Span,
  arg: &'a Str<'a>,
  qualifier: Option<TsEntityName<'a>>,
  type_args: Option<&'a TsTypeParamInstantiation<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsImportType<'a>> for SerializableTsImportType<'a> {
  fn from(orig: TsImportType<'a>) -> Self {
    Self {
      span: orig.span(),
      arg: orig.arg,
      qualifier: orig.qualifier,
      type_args: orig.type_args,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsIndexSignature", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsIndexSignature<'a> {
  span: Span,
  readonly: bool,
  params: Vec<TsFnParam<'a>>,
  type_ann: Option<&'a TsTypeAnn<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsIndexSignature<'a>> for SerializableTsIndexSignature<'a> {
  fn from(orig: TsIndexSignature<'a>) -> Self {
    Self {
      span: orig.span(),
      readonly: orig.readonly().clone(),
      params: orig.params,
      type_ann: orig.type_ann,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsIndexedAccessType", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsIndexedAccessType<'a> {
  span: Span,
  readonly: bool,
  obj_type: TsType<'a>,
  index_type: TsType<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsIndexedAccessType<'a>> for SerializableTsIndexedAccessType<'a> {
  fn from(orig: TsIndexedAccessType<'a>) -> Self {
    Self {
      span: orig.span(),
      readonly: orig.readonly().clone(),
      obj_type: orig.obj_type,
      index_type: orig.index_type,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsInferType", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsInferType<'a> {
  span: Span,
  type_param: &'a TsTypeParam<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsInferType<'a>> for SerializableTsInferType<'a> {
  fn from(orig: TsInferType<'a>) -> Self {
    Self {
      span: orig.span(),
      type_param: orig.type_param,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsInterfaceBody", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsInterfaceBody<'a> {
  span: Span,
  body: Vec<TsTypeElement<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsInterfaceBody<'a>> for SerializableTsInterfaceBody<'a> {
  fn from(orig: TsInterfaceBody<'a>) -> Self {
    Self {
      span: orig.span(),
      body: orig.body,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsInterfaceDecl", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsInterfaceDecl<'a> {
  span: Span,
  declare: bool,
  id: &'a Ident<'a>,
  type_params: Option<&'a TsTypeParamDecl<'a>>,
  extends: Vec<&'a TsExprWithTypeArgs<'a>>,
  body: &'a TsInterfaceBody<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsInterfaceDecl<'a>> for SerializableTsInterfaceDecl<'a> {
  fn from(orig: TsInterfaceDecl<'a>) -> Self {
    Self {
      span: orig.span(),
      declare: orig.declare().clone(),
      id: orig.id,
      type_params: orig.type_params,
      extends: orig.extends,
      body: orig.body,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsIntersectionType", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsIntersectionType<'a> {
  span: Span,
  types: Vec<TsType<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsIntersectionType<'a>> for SerializableTsIntersectionType<'a> {
  fn from(orig: TsIntersectionType<'a>) -> Self {
    Self {
      span: orig.span(),
      types: orig.types,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsKeywordType", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsKeywordType<'a> {
  span: Span,
  kind: TsKeywordTypeKind,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsKeywordType<'a>> for SerializableTsKeywordType<'a> {
  fn from(orig: TsKeywordType<'a>) -> Self {
    Self {
      span: orig.span(),
      kind: orig.kind().clone(),
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsLitType", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsLitType<'a> {
  span: Span,
  lit: TsLit<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsLitType<'a>> for SerializableTsLitType<'a> {
  fn from(orig: TsLitType<'a>) -> Self {
    Self {
      span: orig.span(),
      lit: orig.lit,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsMappedType", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsMappedType<'a> {
  span: Span,
  readonly: Option<TruePlusMinus>,
  optional: Option<TruePlusMinus>,
  type_param: &'a TsTypeParam<'a>,
  name_type: Option<TsType<'a>>,
  type_ann: Option<TsType<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsMappedType<'a>> for SerializableTsMappedType<'a> {
  fn from(orig: TsMappedType<'a>) -> Self {
    Self {
      span: orig.span(),
      readonly: orig.readonly().clone(),
      optional: orig.optional().clone(),
      type_param: orig.type_param,
      name_type: orig.name_type,
      type_ann: orig.type_ann,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsMethodSignature", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsMethodSignature<'a> {
  span: Span,
  readonly: bool,
  computed: bool,
  optional: bool,
  key: Expr<'a>,
  params: Vec<TsFnParam<'a>>,
  type_ann: Option<&'a TsTypeAnn<'a>>,
  type_params: Option<&'a TsTypeParamDecl<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsMethodSignature<'a>> for SerializableTsMethodSignature<'a> {
  fn from(orig: TsMethodSignature<'a>) -> Self {
    Self {
      span: orig.span(),
      readonly: orig.readonly().clone(),
      computed: orig.computed().clone(),
      optional: orig.optional().clone(),
      key: orig.key,
      params: orig.params,
      type_ann: orig.type_ann,
      type_params: orig.type_params,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsModuleBlock", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsModuleBlock<'a> {
  span: Span,
  body: Vec<ModuleItem<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsModuleBlock<'a>> for SerializableTsModuleBlock<'a> {
  fn from(orig: TsModuleBlock<'a>) -> Self {
    Self {
      span: orig.span(),
      body: orig.body,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsModuleDecl", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsModuleDecl<'a> {
  span: Span,
  declare: bool,
  global: bool,
  id: TsModuleName<'a>,
  body: Option<TsNamespaceBody<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsModuleDecl<'a>> for SerializableTsModuleDecl<'a> {
  fn from(orig: TsModuleDecl<'a>) -> Self {
    Self {
      span: orig.span(),
      declare: orig.declare().clone(),
      global: orig.global().clone(),
      id: orig.id,
      body: orig.body,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsNamespaceDecl", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsNamespaceDecl<'a> {
  span: Span,
  declare: bool,
  global: bool,
  id: &'a Ident<'a>,
  body: TsNamespaceBody<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsNamespaceDecl<'a>> for SerializableTsNamespaceDecl<'a> {
  fn from(orig: TsNamespaceDecl<'a>) -> Self {
    Self {
      span: orig.span(),
      declare: orig.declare().clone(),
      global: orig.global().clone(),
      id: orig.id,
      body: orig.body,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsNamespaceExportDecl", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsNamespaceExportDecl<'a> {
  span: Span,
  id: &'a Ident<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsNamespaceExportDecl<'a>> for SerializableTsNamespaceExportDecl<'a> {
  fn from(orig: TsNamespaceExportDecl<'a>) -> Self {
    Self {
      span: orig.span(),
      id: orig.id,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsNonNullExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsNonNullExpr<'a> {
  span: Span,
  expr: Expr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsNonNullExpr<'a>> for SerializableTsNonNullExpr<'a> {
  fn from(orig: TsNonNullExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      expr: orig.expr,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsOptionalType", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsOptionalType<'a> {
  span: Span,
  type_ann: TsType<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsOptionalType<'a>> for SerializableTsOptionalType<'a> {
  fn from(orig: TsOptionalType<'a>) -> Self {
    Self {
      span: orig.span(),
      type_ann: orig.type_ann,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsParamProp", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsParamProp<'a> {
  span: Span,
  accessibility: Option<Accessibility>,
  readonly: bool,
  decorators: Vec<&'a Decorator<'a>>,
  param: TsParamPropParam<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsParamProp<'a>> for SerializableTsParamProp<'a> {
  fn from(orig: TsParamProp<'a>) -> Self {
    Self {
      span: orig.span(),
      accessibility: orig.accessibility().clone(),
      readonly: orig.readonly().clone(),
      decorators: orig.decorators,
      param: orig.param,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsParenthesizedType", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsParenthesizedType<'a> {
  span: Span,
  type_ann: TsType<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsParenthesizedType<'a>> for SerializableTsParenthesizedType<'a> {
  fn from(orig: TsParenthesizedType<'a>) -> Self {
    Self {
      span: orig.span(),
      type_ann: orig.type_ann,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsPropertySignature", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsPropertySignature<'a> {
  span: Span,
  readonly: bool,
  computed: bool,
  optional: bool,
  key: Expr<'a>,
  init: Option<Expr<'a>>,
  params: Vec<TsFnParam<'a>>,
  type_ann: Option<&'a TsTypeAnn<'a>>,
  type_params: Option<&'a TsTypeParamDecl<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsPropertySignature<'a>> for SerializableTsPropertySignature<'a> {
  fn from(orig: TsPropertySignature<'a>) -> Self {
    Self {
      span: orig.span(),
      readonly: orig.readonly().clone(),
      computed: orig.computed().clone(),
      optional: orig.optional().clone(),
      key: orig.key,
      init: orig.init,
      params: orig.params,
      type_ann: orig.type_ann,
      type_params: orig.type_params,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsQualifiedName", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsQualifiedName<'a> {
  span: Span,
  left: TsEntityName<'a>,
  right: &'a Ident<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsQualifiedName<'a>> for SerializableTsQualifiedName<'a> {
  fn from(orig: TsQualifiedName<'a>) -> Self {
    Self {
      span: orig.span(),
      left: orig.left,
      right: orig.right,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsRestType", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsRestType<'a> {
  span: Span,
  type_ann: TsType<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsRestType<'a>> for SerializableTsRestType<'a> {
  fn from(orig: TsRestType<'a>) -> Self {
    Self {
      span: orig.span(),
      type_ann: orig.type_ann,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsThisType", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsThisType<'a> {
  span: Span,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsThisType<'a>> for SerializableTsThisType<'a> {
  fn from(orig: TsThisType<'a>) -> Self {
    Self {
      span: orig.span(),
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsTplLitType", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsTplLitType<'a> {
  span: Span,
  types: Vec<TsType<'a>>,
  quasis: Vec<&'a TplElement<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsTplLitType<'a>> for SerializableTsTplLitType<'a> {
  fn from(orig: TsTplLitType<'a>) -> Self {
    Self {
      span: orig.span(),
      types: orig.types,
      quasis: orig.quasis,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsTupleElement", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsTupleElement<'a> {
  span: Span,
  label: Option<Pat<'a>>,
  ty: TsType<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsTupleElement<'a>> for SerializableTsTupleElement<'a> {
  fn from(orig: TsTupleElement<'a>) -> Self {
    Self {
      span: orig.span(),
      label: orig.label,
      ty: orig.ty,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsTupleType", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsTupleType<'a> {
  span: Span,
  elem_types: Vec<&'a TsTupleElement<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsTupleType<'a>> for SerializableTsTupleType<'a> {
  fn from(orig: TsTupleType<'a>) -> Self {
    Self {
      span: orig.span(),
      elem_types: orig.elem_types,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsTypeAliasDecl", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsTypeAliasDecl<'a> {
  span: Span,
  declare: bool,
  id: &'a Ident<'a>,
  type_params: Option<&'a TsTypeParamDecl<'a>>,
  type_ann: TsType<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsTypeAliasDecl<'a>> for SerializableTsTypeAliasDecl<'a> {
  fn from(orig: TsTypeAliasDecl<'a>) -> Self {
    Self {
      span: orig.span(),
      declare: orig.declare().clone(),
      id: orig.id,
      type_params: orig.type_params,
      type_ann: orig.type_ann,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsTypeAnn", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsTypeAnn<'a> {
  span: Span,
  type_ann: TsType<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsTypeAnn<'a>> for SerializableTsTypeAnn<'a> {
  fn from(orig: TsTypeAnn<'a>) -> Self {
    Self {
      span: orig.span(),
      type_ann: orig.type_ann,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsTypeAssertion", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsTypeAssertion<'a> {
  span: Span,
  expr: Expr<'a>,
  type_ann: TsType<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsTypeAssertion<'a>> for SerializableTsTypeAssertion<'a> {
  fn from(orig: TsTypeAssertion<'a>) -> Self {
    Self {
      span: orig.span(),
      expr: orig.expr,
      type_ann: orig.type_ann,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsTypeLit", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsTypeLit<'a> {
  span: Span,
  members: Vec<TsTypeElement<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsTypeLit<'a>> for SerializableTsTypeLit<'a> {
  fn from(orig: TsTypeLit<'a>) -> Self {
    Self {
      span: orig.span(),
      members: orig.members,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsTypeOperator", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsTypeOperator<'a> {
  span: Span,
  op: TsTypeOperatorOp,
  type_ann: TsType<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsTypeOperator<'a>> for SerializableTsTypeOperator<'a> {
  fn from(orig: TsTypeOperator<'a>) -> Self {
    Self {
      span: orig.span(),
      op: orig.op().clone(),
      type_ann: orig.type_ann,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsTypeParam", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsTypeParam<'a> {
  span: Span,
  name: &'a Ident<'a>,
  constraint: Option<TsType<'a>>,
  default: Option<TsType<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsTypeParam<'a>> for SerializableTsTypeParam<'a> {
  fn from(orig: TsTypeParam<'a>) -> Self {
    Self {
      span: orig.span(),
      name: orig.name,
      constraint: orig.constraint,
      default: orig.default,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsTypeParamDecl", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsTypeParamDecl<'a> {
  span: Span,
  params: Vec<&'a TsTypeParam<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsTypeParamDecl<'a>> for SerializableTsTypeParamDecl<'a> {
  fn from(orig: TsTypeParamDecl<'a>) -> Self {
    Self {
      span: orig.span(),
      params: orig.params,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsTypeParamInstantiation", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsTypeParamInstantiation<'a> {
  span: Span,
  params: Vec<TsType<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsTypeParamInstantiation<'a>> for SerializableTsTypeParamInstantiation<'a> {
  fn from(orig: TsTypeParamInstantiation<'a>) -> Self {
    Self {
      span: orig.span(),
      params: orig.params,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsTypePredicate", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsTypePredicate<'a> {
  span: Span,
  asserts: bool,
  param_name: TsThisTypeOrIdent<'a>,
  type_ann: Option<&'a TsTypeAnn<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsTypePredicate<'a>> for SerializableTsTypePredicate<'a> {
  fn from(orig: TsTypePredicate<'a>) -> Self {
    Self {
      span: orig.span(),
      asserts: orig.asserts().clone(),
      param_name: orig.param_name,
      type_ann: orig.type_ann,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsTypeQuery", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsTypeQuery<'a> {
  span: Span,
  expr_name: TsTypeQueryExpr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsTypeQuery<'a>> for SerializableTsTypeQuery<'a> {
  fn from(orig: TsTypeQuery<'a>) -> Self {
    Self {
      span: orig.span(),
      expr_name: orig.expr_name,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsTypeRef", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsTypeRef<'a> {
  span: Span,
  type_name: TsEntityName<'a>,
  type_params: Option<&'a TsTypeParamInstantiation<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsTypeRef<'a>> for SerializableTsTypeRef<'a> {
  fn from(orig: TsTypeRef<'a>) -> Self {
    Self {
      span: orig.span(),
      type_name: orig.type_name,
      type_params: orig.type_params,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "TsUnionType", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableTsUnionType<'a> {
  span: Span,
  types: Vec<TsType<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<TsUnionType<'a>> for SerializableTsUnionType<'a> {
  fn from(orig: TsUnionType<'a>) -> Self {
    Self {
      span: orig.span(),
      types: orig.types,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "UnaryExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableUnaryExpr<'a> {
  span: Span,
  op: UnaryOp,
  arg: Expr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<UnaryExpr<'a>> for SerializableUnaryExpr<'a> {
  fn from(orig: UnaryExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      op: orig.op().clone(),
      arg: orig.arg,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "UpdateExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableUpdateExpr<'a> {
  span: Span,
  op: UpdateOp,
  prefix: bool,
  arg: Expr<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<UpdateExpr<'a>> for SerializableUpdateExpr<'a> {
  fn from(orig: UpdateExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      op: orig.op().clone(),
      prefix: orig.prefix().clone(),
      arg: orig.arg,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "VarDecl", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableVarDecl<'a> {
  span: Span,
  kind: VarDeclKind,
  declare: bool,
  decls: Vec<&'a VarDeclarator<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<VarDecl<'a>> for SerializableVarDecl<'a> {
  fn from(orig: VarDecl<'a>) -> Self {
    Self {
      span: orig.span(),
      kind: orig.kind().clone(),
      declare: orig.declare().clone(),
      decls: orig.decls,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "VarDeclarator", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableVarDeclarator<'a> {
  span: Span,
  definite: bool,
  name: Pat<'a>,
  init: Option<Expr<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<VarDeclarator<'a>> for SerializableVarDeclarator<'a> {
  fn from(orig: VarDeclarator<'a>) -> Self {
    Self {
      span: orig.span(),
      definite: orig.definite().clone(),
      name: orig.name,
      init: orig.init,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "WhileStmt", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableWhileStmt<'a> {
  span: Span,
  test: Expr<'a>,
  body: Stmt<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<WhileStmt<'a>> for SerializableWhileStmt<'a> {
  fn from(orig: WhileStmt<'a>) -> Self {
    Self {
      span: orig.span(),
      test: orig.test,
      body: orig.body,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "WithStmt", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableWithStmt<'a> {
  span: Span,
  obj: Expr<'a>,
  body: Stmt<'a>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<WithStmt<'a>> for SerializableWithStmt<'a> {
  fn from(orig: WithStmt<'a>) -> Self {
    Self {
      span: orig.span(),
      obj: orig.obj,
      body: orig.body,
      _phantom: PhantomData,
    }
  }
}

#[derive(Serialize)]
#[serde(rename = "YieldExpr", rename_all = "camelCase", tag = "nodeKind")]
pub struct SerializableYieldExpr<'a> {
  span: Span,
  delegate: bool,
  arg: Option<Expr<'a>>,

  #[doc(hidden)]
  #[serde(skip)]
  _phantom: PhantomData<&'a ()>,
}

impl<'a> From<YieldExpr<'a>> for SerializableYieldExpr<'a> {
  fn from(orig: YieldExpr<'a>) -> Self {
    Self {
      span: orig.span(),
      delegate: orig.delegate().clone(),
      arg: orig.arg,
      _phantom: PhantomData,
    }
  }
}
