use kompact::prelude::*;
use std::marker::PhantomData;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;

use crate::data::Sharable;
use crate::control::Control;

use crate::prelude::*;

#[derive(Clone, Collectable, Finalize, NoTrace, NoSerde, NoDebug)]
pub struct Pushable<T: Sharable>(Sender<T>);

#[derive(Collectable, Finalize, NoTrace, NoSerde, NoDebug)]
pub struct Pullable<T: Sharable>(Sender<T>, Receiver<T>);

impl<T: Sharable> Clone for Pullable<T> {
    fn clone(&self) -> Self {
        Pullable(self.0.clone(), self.0.subscribe())
    }
}

crate::data::convert_reflexive!({T: Sharable} Pushable<T>);
crate::data::convert_reflexive!({T: Sharable} Pullable<T>);

crate::channels::impl_channel!();

/// TODO: Processing will currently only stop if all pullers are dropped.
pub fn channel<T: Sharable>(_: &mut Context) -> (Pushable<T>, Pullable<T>) {
    let (l, r) = tokio::sync::broadcast::channel(100);
    (Pushable(l.clone()), Pullable(l, r))
}

impl<T: Sharable> Pushable<T> {
    pub async fn push(&self, data: T) -> Control<()> {
        self.0
            .send(data)
            .map(|_| Control::Continue(()))
            .unwrap_or(Control::Finished)
    }
}

impl<T: Sharable + Clone> Pullable<T> {
    pub async fn pull(&mut self) -> Control<T> {
        self.1
            .recv()
            .await
            .map(Control::Continue)
            .unwrap_or(Control::Finished)
    }
}
