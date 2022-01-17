use kompact::prelude::*;
use std::marker::PhantomData;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;

use crate::control::Control;
use crate::data::Data;

#[derive(Clone)]
pub struct Pushable<T: Data>(Sender<T>);

pub struct Pullable<T: Data>(Sender<T>, Receiver<T>);

impl<T: Data> Clone for Pullable<T> {
    fn clone(&self) -> Self {
        Pullable(self.0.clone(), self.0.subscribe())
    }
}

crate::impl_channel!();

/// TODO: Processing will currently only stop if all pullers are dropped.
pub fn channel<T: Data>(_: &KompactSystem) -> (Pushable<T>, Pullable<T>) {
    let (l, r) = tokio::sync::broadcast::channel(100);
    (Pushable(l.clone()), Pullable(l, r))
}

impl<T: Data> Pushable<T> {
    pub async fn push(&self, data: T) -> Control<()> {
        self.0
            .send(data)
            .map(|_| Control::Continue(()))
            .unwrap_or(Control::Finished)
    }
}

impl<T: Data> Pullable<T> {
    pub async fn pull(&mut self) -> Control<T> {
        self.1
            .recv()
            .await
            .map(Control::Continue)
            .unwrap_or(Control::Finished)
    }
}
