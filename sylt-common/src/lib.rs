pub mod block;
pub mod blob;
pub mod error;
pub mod op;
pub mod prog;
pub mod rc;
pub mod ty;
pub mod upvalue;
pub mod value;

pub use block::{Block, BlockLinkState};
pub use blob::Blob;
pub use op::Op;
pub use ty::Type;
pub use upvalue::UpValue;
pub use value::{IterFn, MatchableValue, Value};

/// A linkable external function. Created either manually or using
/// [sylt_macro::extern_function].
pub type RustFunction = fn(&[Value], bool) -> Result<Value, error::RuntimeError>;
