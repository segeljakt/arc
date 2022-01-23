use arc_runtime::prelude::*;
fn world() {
    let ctx = &mut Context::new();
    let stack: &ShadowStack = &ctx.mutator.shadow_stack();
    let value = String::from_str("Hello, world!", ctx);
    #[allow(unused_unsafe)]
    let mut x = unsafe {
        ShadowStackInternal::<String>::construct(
            stack,
            stack.head.get(),
            core::mem::transmute::<_, TraitObject>(&value as &dyn Rootable).vtable as usize,
            value,
        )
    };
    #[allow(unused_unsafe)]
    stack.head.set(unsafe { core::mem::transmute(&mut x) });
    #[allow(unused_mut)]
    let mut a = unsafe { Rooted::construct(&mut x.value) };
    let mut a = (*a).clone();
//     let y: &str = "Hello, world!";
//     let z: unit = String::push_str(*x, y, ctx);
}
