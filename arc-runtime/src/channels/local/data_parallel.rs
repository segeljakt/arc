use derive_more::Constructor as New;
use kompact::prelude::*;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;

use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;

use crate::control::Control;
use crate::data::Data;

use crate::channels::local::broadcast as bc;

#[derive(Clone, New)]
pub struct Pushable<T: Data, K: Hash, F: Fn(&T) -> K> {
    lanes: Vec<bc::Pushable<T>>,
    parallelism: u64,
    extractor: F,
}

#[derive(New)]
pub struct Pullable<T: Data> {
    lanes: Vec<bc::Pullable<T>>,
}

/// TODO: Not really sure what channel type to use here
pub fn channel<T: Data, K: Hash, F: Fn(&T) -> K>(
    system: &KompactSystem,
    parallelism: u64,
    extractor: F,
) -> (Pushable<T, K, F>, Pullable<T>) {
    let (l, r) = (0..parallelism).map(|_| bc::channel(system)).unzip();
    (Pushable::new(l, parallelism, extractor), Pullable::new(r))
}

impl<T: Data, K: Hash, F: Fn(&T) -> K> Pushable<T, K, F> {
    pub async fn push(&self, data: T) -> Control<()> {
        let mut key = DefaultHasher::new();
        (self.extractor)(&data).hash(&mut key);
        let lane = key.finish() % self.parallelism;
        self.lanes[lane as usize].push(data).await
    }
}

impl<T: Data> Pullable<T> {
    pub async fn pull(&mut self, lane: usize) -> Control<T> {
        self.lanes[lane].pull().await
    }
}
