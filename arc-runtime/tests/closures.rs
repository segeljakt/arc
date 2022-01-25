#[allow(non_camel_case_types)]
mod test_closure {

    use arc_runtime::prelude::*;

    // Generates two datatypes:
    // * Function<I, O>
    // * FunctionTag
    declare_functions!(f);

    struct Env {
        b: i32,
    }

    #[derive(Clone, Debug)]
    struct Args(i32);

    impl Collectable for Args {}
    unsafe impl Trace for Args {}
    unsafe impl Finalize for Args {}

    convert_reflexive!(Args);

    fn f(Args(a): Args, env: ErasedPtr, _: &mut Context) -> i32 {
        let env: Box<Env> = unsafe { ErasablePtr::unerase(env) };
        let b = env.b;
        a + b
    }

    unsafe impl<I: Data, O: Data> Trace for Function<I, O> {
        fn trace(&mut self, _vis: &mut dyn Visitor) {
            match self.tag {
                FunctionTag::f => {
                    let mut env: Box<Env> = unsafe { ErasablePtr::unerase(self.env) };
                    env.b.trace(_vis);
                }
            }
        }
    }

    #[test]
    fn test() {
        let ctx = &mut Context::new();
        let x = function!(f, Env { b: 1 });
        let _z = call!(x, Args(1), ctx);
    }
}

#[allow(non_camel_case_types)]
#[cfg(test)]
mod test_toplevel {

    use arc_runtime::prelude::*;

    declare_functions!(f);

    #[derive(Clone, Debug)]
    struct Args(i32);

    impl Collectable for Args {}
    unsafe impl Trace for Args {}
    unsafe impl Finalize for Args {}

    convert_reflexive!(Args);

    fn f(Args(a): Args, _: ErasedPtr, _: &mut Context) -> i32 {
        a + a
    }

    unsafe impl<I: Data, O: Data> Trace for Function<I, O> {
        fn trace(&mut self, _vis: &mut dyn Visitor) {}
    }

    #[test]
    fn test() {
        let ctx = &mut Context::new();
        let x = function!(f);
        let _z = call!(x, Args(1), ctx);
        let _w = direct_call!(f, Args(1), ctx);
    }
}
