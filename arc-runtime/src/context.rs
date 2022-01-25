use comet::minimark::MiniMark;
use comet::minimark::MiniMarkOptions;
use comet::mutator::MutatorRef;
use kompact::prelude::*;

pub struct Context {
    pub system: KompactSystem,
    pub mutator: MutatorRef<MiniMark>,
}

impl Context {
    pub fn new() -> Self {
        let opts = MiniMarkOptions::default();
        let system = KompactConfig::default().build().unwrap();
        let mutator = comet::minimark::instantiate_minimark(opts);
        Self { system, mutator }
    }
}
