//! Library for compiling arc-scripts inside `build.rs` files.

#![allow(unused)]

mod stage;
mod compile;

pub use compile::process_root;
pub use stage::fun::Fun;
pub use stage::partial::Field;
pub use stage::script::Script;

pub struct Builder;

impl Builder {
    /// Finds all main.arc files in the crate and compiles them. Currently, there are two caveats:
    /// * All scripts are compiled if one is changed.
    /// * All main.arc files are compiled, even unused ones. However, other files ending with .arc 
    /// are only compiled if they are depended on directly or transitively by a main.arc file.
    pub fn process_root() {
        compile::process_root();
    }

    pub fn stage(path: &str) -> Script {
        Script::new("str")
    }
}