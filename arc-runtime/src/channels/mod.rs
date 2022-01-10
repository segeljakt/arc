use kompact::prelude::KompactSystem;

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
    fn channel(sys: &KompactSystem) -> (Self::Pushable, Self::Pullable);
}

macro_rules! impl_channel {
    () => {
        impl<T: Sharable> crate::channels::Channel for Pushable<T> {
            type Pushable = Self;
            type Pullable = Pullable<T>;

            fn channel(sys: &kompact::prelude::KompactSystem) -> (Self::Pushable, Self::Pullable) {
                channel(sys)
            }
        }

        impl<T: Sharable> crate::channels::Channel for Pullable<T> {
            type Pushable = Pushable<T>;
            type Pullable = Self;

            fn channel(sys: &kompact::prelude::KompactSystem) -> (Self::Pushable, Self::Pullable) {
                channel(sys)
            }
        }
    };
}

pub(crate) use impl_channel;
