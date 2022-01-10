use crate::context::Context;
use comet::minimark::MiniMark;

pub use comet::api::Collectable;
pub type Gc<T> = comet::api::Gc<T, MiniMark>;
