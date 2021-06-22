//! Library for compiling arc-scripts inside `build.rs` files.

#![allow(unused)]

mod build;

#[derive(Default)]
pub struct Builder {
    compile_unused_sources: bool,
    source_dirs: Vec<String>,
    prefix: Option<String>,
    enable_optimisations: bool,
}

impl Builder {
    /// Compiles all scripts, regardless of whether they are used or not.
    pub fn compile_unused_sources(self, compile_unused_sources: bool) -> Self {
        Self {
            compile_unused_sources,
            ..self
        }
    }
    /// Compiles all scripts, regardless of whether they are used or not.
    pub fn enable_optimisations(self, enable_optimisations: bool) -> Self {
        Self {
            enable_optimisations,
            ..self
        }
    }
    /// Prefixes output filename with
    pub fn prefix_output_filename_with(self, prefix: impl Into<String>) -> Self {
        Self {
            prefix: Some(prefix.into()),
            ..self
        }
    }
    /// Start compiling from these directories. By default, compilation starts from the
    /// build-script's directory.
    pub fn source_dirs<'i>(self, source_dirs: impl AsRef<[&'i str]>) -> Self {
        Self {
            source_dirs: source_dirs.as_ref().iter().map(|x| x.to_string()).collect(),
            ..self
        }
    }
}
