use comet::minimark::MiniMark;
use comet::minimark::MiniMarkOptions;
use comet::mutator::MutatorRef;
use kompact::prelude::*;

use std::sync::Arc;

pub struct Context {
    pub component: Arc<dyn CoreContainer>,
    pub mutator: MutatorRef<MiniMark>,
}

impl Context {
    pub fn new(component: Arc<dyn CoreContainer>, mutator: MutatorRef<MiniMark>) -> Self {
        Self { component, mutator }
    }
    pub fn launch<C, F>(&self, f: F)
    where
        F: FnOnce() -> C,
        C: ComponentDefinition + 'static,
    {
        let c = self.component.system().create(f);
        self.component.system().start(&c);
    }
}
