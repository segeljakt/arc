pub mod alloc;
pub mod erased;
pub mod functions;
pub mod primitives;
pub mod strings;
pub mod vectors;

use comet::api::Collectable;
use comet::api::Finalize;
use comet::api::Trace;
use comet::gc_base::GcBase;
use dyn_clone::DynClone;

use std::fmt::Debug;
use std::hash::Hash;

use crate::prelude::*;

pub trait Garbage: Collectable + Trace + Finalize {}
impl<T> Garbage for T where T: Collectable + Trace + Finalize {}

/// Trait requirements for keying data.
pub trait Key: Sharable + Hash {}
impl<T> Key for T where T: Sharable + Hash {}

pub trait Concrete {
    type Abstract;
}

pub trait Abstract {
    type Concrete;
}

pub trait DynSendable: Send {
    type T: Sharable;
    fn into_sharable(&self, ctx: &mut Context) -> Self::T;
}

pub trait DynSharable: Garbage + DynClone + Send + Sync + Unpin + Debug {
    type T: Sendable;
    fn into_sendable(&self, ctx: &mut Context) -> Self::T;
}

trait AsyncSafe: Send + Sync + Unpin {}

pub trait Sendable: Sized + DynSendable {}
pub trait Sharable: Sized + DynSharable + Clone {}

impl<T> Sharable for T where T: Sized + DynSharable + Clone {}
impl<T> Sendable for T where T: Sized + DynSendable {}

dyn_clone::clone_trait_object!(<T> DynSharable<T = T>);

#[macro_export]
macro_rules! convert_reflexive {
    {$({$($impl_generics:tt)+})* $ty:ty} => {
        impl $(<$($impl_generics)+>)* DynSharable for $ty {
            type T = Self;
            fn into_sendable(&self, _: &mut Context) -> Self { self.clone() }
        }
        impl $(<$($impl_generics)+>)* DynSendable for $ty {
            type T = Self;
            fn into_sharable(&self, _: &mut Context) -> Self { self.clone() }
        }
    }
}

pub use convert_reflexive;
