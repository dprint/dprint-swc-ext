pub mod common;

#[cfg(test)]
pub(crate) mod test_helpers;

#[cfg(feature = "view")]
pub mod view;

/// swc re-exports
pub mod swc {
  pub use swc_atoms as atoms;
  pub use swc_common as common;
  pub use swc_ecmascript::ast;
  pub use swc_ecmascript::parser;
}
