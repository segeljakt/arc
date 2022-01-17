#[macro_export]
/// Push an event into a channel.
macro_rules! push {
    ($channel:expr, $data:expr) => {
        $channel.push($data).await?
    };
}

#[macro_export]
/// Pull an event from a channel.
macro_rules! pull {
    ($channel:expr) => {
        $channel.pull().await?
    };
}

#[macro_export]
/// A trait which is implemented for all channel-types.
macro_rules! impl_channel {
    () => {
        impl<T: Data> crate::channels::Channel for Pushable<T> {
            type Pushable = Self;
            type Pullable = Pullable<T>;

            fn channel(sys: &kompact::prelude::KompactSystem) -> (Self::Pushable, Self::Pullable) {
                channel(sys)
            }
        }

        impl<T: Data> crate::channels::Channel for Pullable<T> {
            type Pushable = Pushable<T>;
            type Pullable = Self;

            fn channel(sys: &kompact::prelude::KompactSystem) -> (Self::Pushable, Self::Pullable) {
                channel(sys)
            }
        }
    };
}
