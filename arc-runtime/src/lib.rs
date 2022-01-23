// #![feature(fn_traits)]
// #![feature(unboxed_closures)]
// #![feature(arbitrary_self_types)]
// #![feature(async_closure)]
// #![feature(async_stream)]
// #![feature(stream_from_iter)]
#![feature(try_trait_v2)]
#![feature(once_cell)]
#![allow(unused)]
#![allow(clippy::type_complexity)]

pub mod channels;
pub mod context;
pub mod control;
pub mod data;
pub mod executor;
pub mod macros;
pub mod operators;
pub mod task;
pub mod timer;

pub mod prelude {
    // Data types
    pub use crate::channels;
    pub use crate::channels::Channel;
    pub use crate::context::Context;
    pub use crate::control::Control;
    pub use crate::control::Control::Continue;
    pub use crate::control::Control::Finished;
    pub use crate::data::alloc::Alloc;
    pub use crate::data::conversions::IntoSendable;
    pub use crate::data::conversions::IntoSharable;
    pub use crate::data::primitives::bool;
    pub use crate::data::primitives::char;
    pub use crate::data::primitives::f32;
    pub use crate::data::primitives::f64;
    pub use crate::data::primitives::i128;
    pub use crate::data::primitives::i16;
    pub use crate::data::primitives::i32;
    pub use crate::data::primitives::i64;
    pub use crate::data::primitives::i8;
    pub use crate::data::primitives::u128;
    pub use crate::data::primitives::u16;
    pub use crate::data::primitives::u32;
    pub use crate::data::primitives::u64;
    pub use crate::data::primitives::u8;
    pub use crate::data::primitives::unit;
    pub use crate::data::strings::String;
    pub use crate::data::Data;
    pub use crate::data::Key;
    pub use crate::executor::Executor;
    pub use crate::task::message::TaskMessage;

    // Macros
    pub use crate::access;
    pub use crate::enwrap;
    pub use crate::fun_type;
    pub use crate::fun_val;
    pub use crate::is;
    pub use crate::letroot;
    pub use crate::new;
    pub use crate::pull;
    pub use crate::pull_transition;
    pub use crate::push;
    pub use crate::push_transition;
    pub use crate::terminate;
    pub use crate::transition;
    pub use crate::unwrap;
    pub use crate::val;
    pub use crate::wait;
    pub use macros::rewrite;

    // Re-exports
    pub use kompact::prelude::info;
    pub use kompact::prelude::warn;
    pub use kompact::prelude::Actor;
    pub use kompact::prelude::ActorRaw;
    pub use kompact::prelude::Component;
    pub use kompact::prelude::ComponentContext;
    pub use kompact::prelude::ComponentDefinition;
    pub use kompact::prelude::ComponentDefinitionAccess;
    pub use kompact::prelude::ComponentLifecycle;
    pub use kompact::prelude::ComponentLogging;
    pub use kompact::prelude::DeadletterBox;
    pub use kompact::prelude::DynamicPortAccess;
    pub use kompact::prelude::ExecuteResult;
    pub use kompact::prelude::Handled;
    pub use kompact::prelude::KompactConfig;
    pub use kompact::prelude::MsgEnvelope;
    pub use kompact::prelude::NetMessage;
    pub use kompact::prelude::NetworkConfig;
    pub use kompact::prelude::Never;

    pub use crate::data::alloc::Gc;

    pub use comet::api::Collectable;
    pub use comet::api::Finalize;
    pub use comet::api::Trace;
    pub use comet::api::Visitor;
    pub use comet::gc_base::AllocationSpace;
    pub use comet::minimark::MiniMark;
    pub use comet::mopa::TraitObject;
    pub use comet::shadow_stack::Rootable;
    pub use comet::shadow_stack::Rooted;
    pub use comet::shadow_stack::ShadowStack;
    pub use comet::shadow_stack::ShadowStackInternal;

    pub use derive_more::Constructor as New;
    pub use derive_more::Deref;
    pub use derive_more::DerefMut;
    pub use derive_more::From;

    pub use time::macros::date;
    pub use time::macros::time;
    pub use time::Duration;
    pub use time::PrimitiveDateTime as DateTime;

    pub use futures::future::BoxFuture;
    pub use futures::future::FutureExt;

    pub use paste::paste;

    pub use replace_with::replace_with_or_abort_and_return;

    pub use hexf::hexf32;
    pub use hexf::hexf64;

    pub use std::any::Any;
    pub use std::any::TypeId;
    pub use std::cell::UnsafeCell;
    pub use std::future::Future;
    pub use std::pin::Pin;
    pub use std::sync::Arc;
    pub use std::task::Context as PollContext;
    pub use std::task::Poll;
    pub use std::task::Poll::Pending;
    pub use std::task::Poll::Ready;
}
