use kompact::prelude::KompactSystem;
use crate::context::Context;

pub mod macros;

pub mod remote {
    pub mod broadcast;
    pub mod data_parallel;
    pub mod task_parallel;
    pub mod window;
}
pub mod local {
    pub mod broadcast;
    pub mod data_parallel;
    pub mod task_parallel;
    pub mod window;
}

/// A trait for a channel which is implemented for both endpoints (`Pushable` and `Pullable`).
pub trait Channel {
    type Pushable;
    type Pullable;
    fn channel(ctx: &mut Context) -> (Self::Pushable, Self::Pullable);
}

macro_rules! impl_channel {
    () => {
        impl<T: Sharable> crate::channels::Channel for Pushable<T> {
            type Pushable = Self;
            type Pullable = Pullable<T>;

            fn channel(ctx: &mut Context) -> (Self::Pushable, Self::Pullable) {
                channel(ctx)
            }
        }

        impl<T: Sharable> crate::channels::Channel for Pullable<T> {
            type Pushable = Pushable<T>;
            type Pullable = Self;

            fn channel(ctx: &mut Context) -> (Self::Pushable, Self::Pullable) {
                channel(ctx)
            }
        }
    };
}

pub(crate) use impl_channel;
