use crate::prelude::*;

pub mod sharable {
    use crate::prelude::*;

    #[derive(Clone, From, Deref, DerefMut, Debug)]
    #[from(forward)]
    pub struct Vec<T: Sharable>(pub Gc<ConcreteVec<T>>);

    pub type ConcreteVec<T> = comet::alloc::vector::Vector<T, MiniMark>;

    impl<T: Sharable> Alloc<Vec<T>> for ConcreteVec<T> {
        fn alloc(self, ctx: &mut Context) -> Vec<T> {
            Vec(ctx.mutator.allocate(self, AllocationSpace::New))
        }
    }

    unsafe impl<T: Sharable> Trace for Vec<T> {
        fn trace(&mut self, vis: &mut dyn Visitor) {
            self.0.trace(vis)
        }
    }

    impl<T: Sharable> Collectable for Vec<T> {}
    unsafe impl<T: Sharable> Finalize for Vec<T> {}
    unsafe impl<T: Sharable> Send for Vec<T> {}
    unsafe impl<T: Sharable> Sync for Vec<T> {}
    impl<T: Sharable> Unpin for Vec<T> {}
}

mod sendable {
    use crate::prelude::*;

    #[derive(From)]
    #[from(forward)]
    pub struct Vec<T>(pub ConcreteVec<T>);

    pub type ConcreteVec<T> = std::vec::Vec<T>;

    unsafe impl<T> Send for Vec<T> {}
}

impl<T: Sharable> DynSharable for sharable::Vec<T> {
    type T = sendable::Vec<T::T>;
    fn into_sendable(&self, ctx: &mut Context) -> Self::T {
        self.0
            .into_iter()
            .map(|v| v.clone().into_sendable(ctx))
            .collect::<std::vec::Vec<_>>()
            .into_boxed_slice()
            .into()
    }
}

impl<T: Sendable> DynSendable for sendable::Vec<T> {
    type T = sharable::Vec<T::T>;
    fn into_sharable(&self, ctx: &mut Context) -> Self::T {
        let mut s = Vec::<T::T>::with_capacity(self.0.len(), ctx);
        for v in self.0.iter() {
            let v = v.into_sharable(ctx);
            s.0.push(&mut ctx.mutator, v);
        }
        s
    }
}

pub use sharable::Vec;

impl<T: Sharable> Vec<T> {
    pub fn new(ctx: &mut Context) -> Self {
        sharable::ConcreteVec::<T>::new(&mut ctx.mutator)
            .alloc(ctx)
            .into()
    }

    pub fn with_capacity(capacity: usize, ctx: &mut Context) -> Self {
        sharable::ConcreteVec::<T>::with_capacity(&mut ctx.mutator, capacity)
            .alloc(ctx)
            .into()
    }

    pub fn write_barrier(&mut self, ctx: &mut Context) {
        self.0.write_barrier(&mut ctx.mutator)
    }

    pub fn as_slice<'a>(&'a self, ctx: &mut Context) -> &'a [T] {
        self.0.as_slice()
    }

    pub fn as_slice_mut<'a>(&'a mut self, ctx: &mut Context) -> &'a mut [T] {
        self.0.as_slice_mut()
    }

    pub fn capacity(self, ctx: &mut Context) -> usize {
        self.0.capacity()
    }

    pub fn len(self, ctx: &mut Context) -> usize {
        self.0.len()
    }

    pub fn shrink_to(mut self, min_capacity: usize, ctx: &mut Context) {
        self.0.shrink_to(&mut ctx.mutator, min_capacity);
    }

    pub fn retain<F>(mut self, f: F, ctx: &mut Context)
    where
        F: FnMut(&T) -> bool,
    {
        self.0.retain(f);
    }

    pub fn clear(mut self, ctx: &mut Context) {
        self.0.clear();
    }

    pub fn resize(mut self, new_len: usize, value: T, ctx: &mut Context) {
        self.0.resize(&mut ctx.mutator, new_len, value);
    }

    pub fn push(mut self, value: T, ctx: &mut Context) {
        self.0.push(&mut ctx.mutator, value);
    }

    pub fn pop(mut self, ctx: &mut Context) -> Option<T> {
        self.0.pop()
    }

    pub fn remove(mut self, index: usize, ctx: &mut Context) -> T {
        self.0.remove(index)
    }

    pub fn at<'a>(&'a self, index: usize, ctx: &mut Context) -> &'a T {
        self.0.at(index)
    }

    pub fn insert(mut self, index: usize, value: T, ctx: &mut Context) {
        self.0.insert(&mut ctx.mutator, index, value);
    }

    pub fn is_empty(self, ctx: &mut Context) -> bool {
        self.0.is_empty()
    }

    pub fn dedup(mut self, ctx: &mut Context)
    where
        T: PartialEq,
    {
        self.0.dedup();
    }
}
