pub mod conversions;
pub mod enums;
pub mod functions;
pub mod primitives;
pub mod strings;
pub mod structs;
pub mod values;

use comet::api::Collectable;
pub use time::PrimitiveDateTime as DateTime;
pub use time::Duration;

/// Trait requirements for sending data.
pub trait Data: 'static + Send + std::fmt::Debug + Clone + Unpin {}
impl<T> Data for T where T: 'static + Send + std::fmt::Debug + Clone + Unpin {}

/// Trait requirements for keying data.
pub trait Key: Data + std::hash::Hash {}
impl<T> Key for T where T: Data + std::hash::Hash {}
