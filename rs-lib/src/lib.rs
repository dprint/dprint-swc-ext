#![warn(clippy::print_stdout)]

mod comments;
mod custom;
mod source_file;
mod tokens;
mod types;

#[allow(clippy::all)]
#[rustfmt::skip]
mod generated;

#[cfg(feature = "serialize")]
mod serialize;

pub use comments::CommentsIterator;
pub use custom::*;
pub use generated::*;
pub use source_file::*;
pub use types::*;

#[cfg(feature = "serialize")]
pub use serialize::*;

#[cfg(test)]
mod test_helpers;
