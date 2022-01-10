use crossfire::mpmc::RxFuture;
use crossfire::mpmc::SharedFutureBoth;
use crossfire::mpmc::TxFuture;
use kompact::prelude::*;
use std::marker::PhantomData;

use crate::channels::Channel;
use crate::control::Control;
use crate::data::DynSharable;
use std::fmt::Debug;

use crate::prelude::*;

#[derive(Clone, Collectable, Finalize, NoTrace, NoSerde, NoDebug)]
pub struct Pushable<T: Sharable>(TxFuture<T, SharedFutureBoth>);

#[derive(Clone, Collectable, Finalize, NoTrace, NoSerde, NoDebug)]
pub struct Pullable<T: Sharable>(RxFuture<T, SharedFutureBoth>);

crate::data::convert_reflexive!({T: Sharable} Pullable<T>);
crate::data::convert_reflexive!({T: Sharable} Pushable<T>);

crate::channels::impl_channel!();

pub fn channel<T: Sharable>(_: &mut Context) -> (Pushable<T>, Pullable<T>) {
    let (tx, rx) = crossfire::mpmc::bounded_future_both(100);
    (Pushable(tx), Pullable(rx))
}

impl<T: Sharable> Pushable<T> {
    pub async fn push(&self, data: T) -> Control<()> {
        self.0
            .send(data)
            .await
            .map(Control::Continue)
            .unwrap_or(Control::Finished)
    }
}

impl<T: Sharable> Pullable<T> {
    pub async fn pull(&self) -> Control<T> {
        self.0
            .recv()
            .await
            .map(Control::Continue)
            .unwrap_or(Control::Finished)
    }
}
