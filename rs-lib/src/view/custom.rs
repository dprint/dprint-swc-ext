use super::generated::BindingIdent;
use super::generated::Ident;
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
