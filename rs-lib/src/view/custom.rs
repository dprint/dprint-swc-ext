use super::generated::AssignTargetPat;
use super::generated::BindingIdent;
use super::generated::Expr;
use super::generated::Ident;
use super::generated::Pat;
use super::generated::SimpleAssignTarget;
use crate::swc::ast::Id;
use crate::swc::common::SyntaxContext;

impl<'a> Ident<'a> {
  pub fn to_id(&self) -> Id {
    (self.sym().clone(), self.ctxt())
  }

  pub fn ctxt(&self) -> SyntaxContext {
    self.inner.span.ctxt
  }
}

impl<'a> BindingIdent<'a> {
  pub fn to_id(&self) -> Id {
    self.id.to_id()
  }

  pub fn ctxt(&self) -> SyntaxContext {
    self.id.ctxt()
  }
}

impl<'a> SimpleAssignTarget<'a> {
  pub fn as_expr(&self) -> Expr<'a> {
    match self {
      SimpleAssignTarget::Ident(node) => Expr::Ident(node.id),
      SimpleAssignTarget::Member(node) => Expr::Member(node),
      SimpleAssignTarget::SuperProp(node) => Expr::SuperProp(node),
      SimpleAssignTarget::Paren(node) => Expr::Paren(node),
      SimpleAssignTarget::OptChain(node) => Expr::OptChain(node),
      SimpleAssignTarget::TsAs(node) => Expr::TsAs(node),
      SimpleAssignTarget::TsSatisfies(node) => Expr::TsSatisfies(node),
      SimpleAssignTarget::TsNonNull(node) => Expr::TsNonNull(node),
      SimpleAssignTarget::TsTypeAssertion(node) => Expr::TsTypeAssertion(node),
      SimpleAssignTarget::TsInstantiation(node) => Expr::TsInstantiation(node),
      SimpleAssignTarget::Invalid(node) => Expr::Invalid(node),
    }
  }
}

impl<'a> AssignTargetPat<'a> {
  pub fn as_pat(&self) -> Pat<'a> {
    match self {
      AssignTargetPat::Array(node) => Pat::Array(node),
      AssignTargetPat::Object(node) => Pat::Object(node),
      AssignTargetPat::Invalid(node) => Pat::Invalid(node),
    }
  }
}
