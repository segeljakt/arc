use crossfire::mpmc::RxFuture;
use crossfire::mpmc::SharedFutureBoth;
use crossfire::mpmc::TxFuture;
use std::marker::PhantomData;
use kompact::prelude::*;

use crate::control::Control;
use crate::data::Data;

pub struct Channel<T: Data>(PhantomData<T>);

pub struct Pushable<T: Data>(TxFuture<T, SharedFutureBoth>);
pub struct Pullable<T: Data>(RxFuture<T, SharedFutureBoth>);

pub fn channel<T:Data>(_: &KompactSystem) -> (Pushable<T>, Pullable<T>) {
    let (tx, rx) = crossfire::mpmc::bounded_future_both(100);
    (Pushable(tx), Pullable(rx))
}

impl<T: Data> Pushable<T> {
    pub async fn push(&self, data: T) -> Control<()> {
        self.0
            .send(data)
            .await
            .map(Control::Continue)
            .unwrap_or(Control::Finished)
    }
}

impl<T: Data> Pullable<T> {
    pub async fn pull(&self) -> Control<T> {
        self.0
            .recv()
            .await
            .map(Control::Continue)
            .unwrap_or(Control::Finished)
    }
}
