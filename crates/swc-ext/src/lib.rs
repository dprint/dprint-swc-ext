mod pos;
mod text_info;

pub use pos::*;
pub use text_info::*;
pub use text_lines;

/// A 0-indexed line and column type.
pub type LineAndColumnIndex = text_lines::LineAndColumnIndex;

/// A 1-indexed line and column type which should be used for
/// display purposes only (ex. in diagnostics).
pub type LineAndColumnDisplay = text_lines::LineAndColumnDisplay;

/// swc re-exports
pub mod swc {
  pub use swc_atoms as atoms;
  pub use swc_common as common;
  pub use swc_ecmascript::ast;
  pub use swc_ecmascript::parser;
}
