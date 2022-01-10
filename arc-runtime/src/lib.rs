// #![feature(fn_traits)]
// #![feature(unboxed_closures)]
// #![feature(arbitrary_self_types)]
// #![feature(async_closure)]
// #![feature(async_stream)]
// #![feature(stream_from_iter)]
#![feature(try_trait_v2)]
#![allow(unused)]
#![allow(clippy::type_complexity)]

pub mod client;
pub mod control;
pub mod data;
pub mod executor;
pub mod channels {
    pub mod remote {
        pub mod broadcast;
        pub mod concurrent;
    }
    pub mod local {
        pub mod broadcast;
        pub mod concurrent;
    }
}
pub mod gc;
pub mod datagen;
pub mod system;

pub mod prelude {
    pub use crate::channels;
    pub use crate::client::*;
    pub use crate::control::*;
    pub use crate::data::*;
    pub use crate::datagen::*;
    pub use crate::executor::*;
    pub use crate::system::*;
    pub use crate::gc::*;
    pub use kompact::prelude::*;
    pub use std::any::Any;
    pub use std::any::TypeId;
    pub use std::marker::PhantomData;
    pub use std::sync::Arc;
    pub use std::time::Duration;
    pub use time::PrimitiveDateTime as DateTime;
}
