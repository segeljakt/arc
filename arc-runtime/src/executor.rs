use crate::channels::local::broadcast as lbc;
use crate::channels::local::concurrent as lcc;
use crate::channels::remote::broadcast as rbc;
use crate::channels::remote::concurrent as rcc;
use crate::data::Data;
use std::lazy::OnceCell;

use kompact::prelude::*;
use std::sync::Arc;

pub struct Executor {
    cell: OnceCell<KompactSystem>,
}

impl Executor {
    pub const fn new() -> Self {
        Self {
            cell: OnceCell::new(),
        }
    }
    pub fn init(&self, value: KompactSystem) {
        assert!(self.cell.set(value).is_ok())
    }
    pub fn create_task<C: ComponentDefinition>(&self, f: impl FnOnce() -> C) -> Arc<Component<C>> {
        let task = self.create(f);
        self.start(&task);
        task
    }
}

impl std::ops::Deref for Executor {
    type Target = KompactSystem;

    fn deref(&self) -> &Self::Target {
        self.cell.get().unwrap()
    }
}

unsafe impl Sync for Executor {}
