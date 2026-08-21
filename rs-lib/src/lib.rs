pub mod common;

#[cfg(test)]
pub(crate) mod test_helpers;

#[cfg(feature = "view")]
pub mod view;

/// swc re-exports
pub mod swc {
  pub use swc_atoms as atoms;
  pub use swc_common as common;
  pub use swc_ecma_ast as ast;
  pub use swc_ecma_lexer as lexer;
  pub use swc_ecma_parser as parser;
}
