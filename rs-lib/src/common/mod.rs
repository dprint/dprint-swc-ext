mod comments;
mod pos;
mod text_info;
mod tokens;
mod types;

pub use comments::*;
pub use pos::*;
pub use text_info::*;
pub use text_lines;
pub use tokens::*;
pub use types::*;

/// A 0-indexed line and column type.
pub type LineAndColumnIndex = text_lines::LineAndColumnIndex;

/// A 1-indexed line and column type which should be used for
/// display purposes only (ex. in diagnostics).
pub type LineAndColumnDisplay = text_lines::LineAndColumnDisplay;
