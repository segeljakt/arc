use comet::api::Collectable;

/// Trait requirements for sending data.
pub trait Data: 'static + Send + std::fmt::Debug + Clone + Unpin {}
impl<T> Data for T where T: 'static + Send + std::fmt::Debug + Clone + Unpin {}

/// Trait requirements for keying data.
pub trait Key: Data + std::hash::Hash {}
impl<T> Key for T where T: Data + std::hash::Hash {}

pub use time::PrimitiveDateTime as DateTime;
