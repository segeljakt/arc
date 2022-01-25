use comet::minimark::MiniMark;
use comet::minimark::MiniMarkOptions;
use comet::mutator::MutatorRef;
use kompact::prelude::*;

pub struct Context {
    pub system: System,
    pub mutator: MutatorRef<MiniMark>,
}

pub type System = impl kompact::prelude::SystemHandle;

fn get_handle<T: Actor + ComponentDefinition>(c: &ComponentDefinitionAccess<T>) -> impl kompact::prelude::SystemHandle {
    let x =c.ctx().system();
    x
}

impl Context {
    pub fn new() -> Self {
        let opts = MiniMarkOptions::default();
        let system = KompactConfig::default().build().unwrap();
        let mutator = comet::minimark::instantiate_minimark(opts);
        Self { system, mutator }
    }
}

impl<T: Actor + ComponentDefinition> From<kompact::prelude::ComponentContext<T>> for Ctx<T> {
    fn from(ctx: kompact::prelude::ComponentContext<T>) -> Self {
        let opts = MiniMarkOptions::default();
        let mutator = comet::minimark::instantiate_minimark(opts);
        Self {
            kompact: ctx,
            mutator,
        }
    }
}
