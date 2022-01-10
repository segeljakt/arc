use crate::context::Context;
use comet::minimark::MiniMark;

pub use comet::api::Collectable;
pub type Gc<T> = comet::api::Gc<T, MiniMark>;

pub trait Alloc<T> {
    #[inline(always)]
    fn alloc(self, ctx: &mut Context) -> T;
}

macro_rules! alloc_identity {
    { $ty:ty } => {
        impl Alloc<$ty> for $ty {
            fn alloc<'i>(self, ctx: &'i mut Context) -> $ty {
                self
            }
        }
    }
}

pub(crate) use alloc_identity;
