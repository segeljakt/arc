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
pub mod control;
pub mod data;
pub mod executor;
pub mod gc;
pub mod timer;

pub mod prelude {
    pub use crate::channels::local::broadcast::channel as local_broadcast_channel;
    pub use crate::channels::local::broadcast::Pullable as LocalBroadcastPullable;
    pub use crate::channels::local::broadcast::Pushable as LocalBroadcastPushable;
    pub use crate::channels::local::concurrent::channel as local_concurrent_channel;
    pub use crate::channels::local::concurrent::Pullable as LocalConcurrentPullable;
    pub use crate::channels::local::concurrent::Pushable as LocalConcurrentPushable;
    pub use crate::channels::local::keyed::channel as local_keyed_channel;
    pub use crate::channels::local::keyed::Pullable as LocalKeyedPullable;
    pub use crate::channels::local::keyed::Pushable as LocalKeyedPushable;
    pub use crate::channels::remote::broadcast::channel as remote_broadcast_channel;
    pub use crate::channels::remote::broadcast::Pullable as RemoteBroadcastPullable;
    pub use crate::channels::remote::broadcast::Pushable as RemoteBroadcastPushable;
    pub use crate::channels::remote::concurrent::channel as remote_concurrent_channel;
    pub use crate::channels::remote::concurrent::Pullable as RemoteConcurrentPullable;
    pub use crate::channels::remote::concurrent::Pushable as RemoteConcurrentPushable;
    pub use crate::control::Control;
    pub use crate::data::conversions::IntoSendable;
    pub use crate::data::conversions::IntoSharable;
    pub use crate::data::Data;
    pub use crate::data::DateTime;
    pub use crate::data::Duration;
    pub use crate::data::Key;
    pub use crate::executor::Executor;

    pub use kompact::prelude::warn;
    pub use kompact::prelude::Actor;
    pub use kompact::prelude::ActorRaw;
    pub use kompact::prelude::Component;
    pub use kompact::prelude::ComponentContext;
    pub use kompact::prelude::ComponentDefinition;
    pub use kompact::prelude::ComponentDefinitionAccess;
    pub use kompact::prelude::ComponentLifecycle;
    pub use kompact::prelude::ComponentLogging;
    pub use kompact::prelude::DynamicPortAccess;
    pub use kompact::prelude::ExecuteResult;
    pub use kompact::prelude::Handled;
    pub use kompact::prelude::KompactConfig;
    pub use kompact::prelude::MsgEnvelope;
    pub use kompact::prelude::Never;
    pub use kompact::prelude::info;
}
