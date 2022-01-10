use derive_more::Constructor as New;
use kompact::prelude::*;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;

use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;

use crate::prelude::Control;
use crate::prelude::Collectable;
use crate::prelude::Finalize;
use crate::prelude::NoDebug;
use crate::prelude::NoSerde;
use crate::prelude::NoTrace;
use crate::prelude::Visitor;
use crate::prelude::Trace;
use crate::prelude::Deserialize;
use crate::prelude::Deserializer;
use crate::prelude::Serialize;
use crate::prelude::Serializer;
use crate::prelude::Sharable;
use crate::prelude::Context;
use crate::prelude::DynSharable;
use crate::prelude::DynSendable;

use crate::channels::local::broadcast as bc;

#[derive(Clone, New, Collectable, Finalize, NoTrace, NoSerde, NoDebug)]
pub struct Pushable<T: Sharable, K: Sharable + Hash> {
    lanes: Vec<bc::Pushable<T>>,
    parallelism: u64,
    extractor: fn(&T) -> K,
}

#[derive(Clone, New, Collectable, Finalize, NoTrace, NoSerde, NoDebug)]
pub struct Pullable<T: Sharable> {
    lanes: Vec<bc::Pullable<T>>,
}

crate::data::convert_reflexive!({T: Sharable, K: Sharable+Hash} Pushable<T, K>);
crate::data::convert_reflexive!({T: Sharable} Pullable<T>);

/// TODO: Not really sure what channel type to use here
pub fn channel<T: Sharable, K: Sharable + Hash>(
    parallelism: u64,
    extractor: fn(&T) -> K,
    ctx: &mut Context,
) -> (Pushable<T, K>, Pullable<T>) {
    let (l, r) = (0..parallelism).map(|_| bc::channel(ctx)).unzip();
    (Pushable::new(l, parallelism, extractor), Pullable::new(r))
}

impl<T: Sharable, K: Sharable + Hash> Pushable<T, K> {
    pub async fn push(&self, data: T) -> Control<()> {
        let mut key = DefaultHasher::new();
        (self.extractor)(&data).hash(&mut key);
        let lane = key.finish() % self.parallelism;
        self.lanes[lane as usize].push(data).await
    }
}

impl<T: Sharable> Pullable<T> {
    pub async fn pull(&mut self, lane: usize) -> Control<T> {
        self.lanes[lane].pull().await
    }
}
