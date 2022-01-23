pub mod alloc;
pub mod conversions;
pub mod primitives;
pub mod strings;
pub mod vectors;

use crate::data::conversions::IntoSendable;
use crate::data::conversions::IntoSharable;
use comet::api::Collectable;
use comet::api::Finalize;
use comet::api::Trace;
use comet::gc_base::GcBase;

use std::fmt::Debug;
use std::hash::Hash;

pub trait Data:
    'static + Send + Sync + Clone + Debug + Unpin + IntoSendable + Collectable + Finalize + Trace
{
}

impl<T> Data for T where
    T: 'static
        + Send
        + Sync
        + Clone
        + Debug
        + Unpin
        + IntoSendable
        + Collectable
        + Finalize
        + Trace
{
}

/// Trait requirements for keying data.
pub trait Key: Data + Hash {}
impl<T> Key for T where T: Data + Hash {}
