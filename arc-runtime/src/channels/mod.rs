use kompact::prelude::KompactSystem;

pub mod macros;

pub mod remote {
    pub mod broadcast;
    pub mod concurrent;
    pub mod keyed;
}
pub mod local {
    pub mod broadcast;
    pub mod concurrent;
    pub mod keyed;
}

pub mod datagen;

/// A trait for a channel which is implemented for both endpoints (`Pushable` and `Pullable`).
pub trait Channel {
    type Pushable;
    type Pullable;
    fn channel(sys: &KompactSystem) -> (Self::Pushable, Self::Pullable);
}

