#[cfg(test)]
mod test {
    use comet::api::Collectable;
    use comet::api::Finalize;
    use comet::api::Gc;
    use comet::api::Trace;
    use comet::api::Visitor;
    use comet::gc_base::AllocationSpace;
    use comet::gc_base::GcBase;
    use comet::letroot;
    use comet::minimark::instantiate_minimark;
    use comet::minimark::MiniMark;
    use comet::minimark::MiniMarkOptions;

    pub enum List<T: Collectable> {
        Nil,
        Cons(T, Gc<List<T>, MiniMark>),
    }

    unsafe impl<T: Collectable> Trace for List<T> {
        fn trace(&mut self, vis: &mut dyn Visitor) {
            if let Self::Cons(data, next) = self {
                data.trace(vis);
                next.trace(vis);
            }
        }
    }

    unsafe impl<T: Collectable> Finalize for List<T> {}

    impl<T: Collectable> Collectable for List<T> {}

    #[test]
    fn run() {
        let mut opts = MiniMarkOptions::default();

        let mut mutator = instantiate_minimark(opts);
        let stack = mutator.shadow_stack();

        letroot!(l = stack, mutator.allocate(List::Nil, AllocationSpace::New));

        for i in 0..100 {
            *l = mutator.allocate(List::Cons(i, *l), AllocationSpace::New);
        }

        for i in 0..100 {
            match **l {
                List::Nil => break,
                List::Cons(data, tail) => {
                    assert_eq!(data, 99 - i);
                    *l = tail;
                }
            }
        }
    }
}
