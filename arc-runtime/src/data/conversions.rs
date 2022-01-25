use crate::context::Context;
use crate::data::Data;
use crate::data::alloc::Collectable;

/// Converts an item into something which can be sent across threads.
pub trait IntoSendable {
    type T: Send;
    fn into_sendable(self, ctx: &mut Context) -> Self::T;
}

/// Converts an item into something which can be shared within a thread.
pub trait IntoSharable {
    type T: Data;
    fn into_sharable(self, ctx: &mut Context) -> Self::T;
}

#[macro_export]
macro_rules! convert_reflexive {
    { $ty:ty } => {
        impl IntoSendable for $ty {
            type T = Self;
            fn into_sendable(self, _: &mut Context) -> Self { self }
        }
        impl IntoSharable for $ty {
            type T = Self;
            fn into_sharable(self, _: &mut Context) -> Self { self }
        }
    }
}

pub(crate) use convert_reflexive;
