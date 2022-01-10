use kompact::prelude::*;
use std::marker::PhantomData;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;

use crate::control::Control;
use crate::data::Data;

pub struct Pushable<T: Data>(Sender<T>);
pub struct Pullable<T: Data>(Receiver<T>);

pub fn channel<T: Data>(_: &KompactSystem) -> (Pushable<T>, Pullable<T>) {
    let (l, r) = tokio::sync::broadcast::channel(100);
    (Pushable(l), Pullable(r))
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
        self.0
            .recv()
            .await
            .map(Control::Continue)
            .unwrap_or(Control::Finished)
    }
}
