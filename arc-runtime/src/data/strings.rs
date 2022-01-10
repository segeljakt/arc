use crate::prelude::*;

pub mod sharable {
    use crate::prelude::*;

    #[derive(Clone, From, Deref, DerefMut, Debug)]
    #[from(forward)]
    pub struct String(pub Gc<ConcreteString>);

    pub type ConcreteString = comet::alloc::string::String<MiniMark>;

    impl Alloc<String> for ConcreteString {
        fn alloc(self, ctx: &mut Context) -> String {
            String(ctx.mutator.allocate(self, AllocationSpace::New).into())
        }
    }

    unsafe impl Trace for String {
        fn trace(&mut self, vis: &mut dyn Visitor) {
            self.0.trace(vis)
        }
    }

    impl Collectable for String {}
    unsafe impl Finalize for String {}
    unsafe impl Send for String {}
    unsafe impl Sync for String {}
    impl Unpin for String {}
}

mod sendable {
    use crate::prelude::*;

    #[derive(From, Send, Serialize, Deserialize)]
    #[from(forward)]
    pub struct String(pub ConcreteString);

    pub type ConcreteString = Box<str>;
}

impl DynSharable for sharable::String {
    type T = sendable::String;
    fn into_sendable(&self, ctx: &mut Context) -> Self::T {
        self.0.to_string().into()
    }
}

impl DynSendable for sendable::String {
    type T = sharable::String;
    fn into_sharable(&self, ctx: &mut Context) -> Self::T {
        String::from_str(self.0.as_ref(), ctx)
    }
}

pub use sharable::String;

impl String {
    pub fn new(ctx: &mut Context) -> String {
        sharable::ConcreteString::new(&mut ctx.mutator).alloc(ctx)
    }

    pub fn with_capacity(capacity: usize, ctx: &mut Context) -> String {
        sharable::ConcreteString::with_capacity(&mut ctx.mutator, capacity).alloc(ctx)
    }

    pub fn push(mut self, ch: char, ctx: &mut Context) {
        self.0.push(&mut ctx.mutator, ch)
    }

    pub fn push_str(mut self, s: &str, ctx: &mut Context) {
        self.0.push_str(&mut ctx.mutator, s)
    }

    pub fn from_str(s: &str, ctx: &mut Context) -> String {
        let mut new = sharable::ConcreteString::with_capacity(&mut ctx.mutator, s.len());
        new.push_str(&mut ctx.mutator, s);
        new.alloc(ctx)
    }

    pub fn remove(&mut self, idx: usize, _: &mut Context) -> char {
        self.0.remove(idx)
    }

    pub fn insert(&mut self, idx: usize, ch: char, ctx: &mut Context) {
        self.0.insert(&mut ctx.mutator, idx, ch)
    }

    pub fn is_empty(&mut self, _: &mut Context) -> bool {
        self.0.is_empty()
    }

    pub fn split_off(&mut self, at: usize, ctx: &mut Context) -> String {
        self.0.split_off(&mut ctx.mutator, at).alloc(ctx)
    }

    pub fn clear(&mut self, _: &mut Context) {
        self.0.clear()
    }

    pub fn len(&self, _: &mut Context) -> usize {
        self.0.len()
    }
}
