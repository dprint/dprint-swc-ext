mod comments;
mod tokens;
mod types;

#[allow(clippy::all)]
#[rustfmt::skip]
mod generated;

#[cfg(feature = "serialize")]
mod serialize;

pub use comments::CommentsIterator;
pub use generated::*;
pub use types::*;

#[cfg(feature = "serialize")]
pub use serialize::*;

// swc re-exports
pub use swc_common::comments::{Comment, CommentKind};
pub use swc_common::{BytePos, Span, Spanned};
pub use swc_ecmascript::parser::token::{Token, TokenAndSpan};
