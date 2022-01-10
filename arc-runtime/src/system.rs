use crate::channels::local::broadcast as lbc;
use crate::channels::local::concurrent as lcc;
use crate::channels::remote::broadcast as rbc;
use crate::channels::remote::concurrent as rcc;
use crate::data::Data;

use kompact::prelude::*;
use std::sync::Arc;

pub trait SystemUtils {
    fn create_task<CD, F>(&self, f: F) -> Arc<Component<CD>>
    where
        CD: ComponentDefinition,
        F: FnOnce() -> CD;
}

impl SystemUtils for KompactSystem {
    fn create_task<C, F>(&self, f: F) -> Arc<Component<C>>
    where
        C: ComponentDefinition,
        F: FnOnce() -> C,
    {
        let task = self.create(f);
        self.start(&task);
        task
    }
}
