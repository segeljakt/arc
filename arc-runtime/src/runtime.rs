use comet::minimark::MiniMark;
use comet::minimark::MiniMarkOptions;
use comet::mutator::MutatorRef;
use kompact::prelude::*;

pub struct Runtime {
    pub system: KompactSystem,
}

impl Runtime {
    pub fn new() -> Self {
        let system = KompactConfig::default().build().unwrap();
        Self { system }
    }
}
