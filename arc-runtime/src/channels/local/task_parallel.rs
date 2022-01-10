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

#[derive(Clone, Collectable, Finalize, NoTrace)]
pub struct Pushable<T: DynSharable>(TxFuture<T, SharedFutureBoth>);

#[derive(Clone, Collectable, Finalize, NoTrace)]
pub struct Pullable<T: DynSharable>(RxFuture<T, SharedFutureBoth>);

impl<T: DynSharable> Debug for Pushable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pushable")
    }
}

impl<T: DynSharable> Debug for Pullable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pullable")
    }
}

crate::data::convert_reflexive!({T:DynSharable + Clone} Pullable<T>);
crate::channels::impl_channel!();

pub fn channel<T: DynSharable>(_: &KompactSystem) -> (Pushable<T>, Pullable<T>) {
    let (tx, rx) = crossfire::mpmc::bounded_future_both(100);
    (Pushable(tx), Pullable(rx))
}

impl<T: DynSharable> Pushable<T> {
    pub async fn push(&self, data: T) -> Control<()> {
        self.0
            .send(data)
            .await
            .map(Control::Continue)
            .unwrap_or(Control::Finished)
    }
}

impl<T: DynSharable> Pullable<T> {
    pub async fn pull(&self) -> Control<T> {
        self.0
            .recv()
            .await
            .map(Control::Continue)
            .unwrap_or(Control::Finished)
    }
}
