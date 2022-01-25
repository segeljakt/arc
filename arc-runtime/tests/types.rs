// #![allow(unused)]
//
// mod basic1 {
//     use arc_runtime::prelude::*;
//     #[rewrite]
//     pub struct Point {
//         pub x: i32,
//         pub y: i32,
//     }
//
//     #[rewrite]
//     pub enum Foo {
//         FooBar(i32),
//         FooBaz(f32),
//     }
// }
//
// mod basic2 {
//     use arc_runtime::prelude::*;
//     #[rewrite]
//     pub struct A {
//         pub b: B,
//     }
//
//     #[rewrite]
//     pub struct B {
//         pub c: i32,
//     }
// }
//
// mod basic3 {
//     use arc_runtime::prelude::*;
//     #[rewrite]
//     pub enum A {
//         AB(B),
//         AC(C),
//     }
//
//     #[rewrite]
//     pub struct B {
//         pub v: i32,
//     }
//
//     #[rewrite]
//     pub struct C {}
// }
//
mod list {
    use arc_runtime::prelude::*;

    #[rewrite]
    pub enum List {
        ListCons(Cons),
        ListNil(unit),
    }

    #[rewrite]
    pub struct Cons {
        pub v: i32,
        pub t: List,
    }

    #[rewrite(main)]
    #[test]
    fn test() {
        let l: List = enwrap!(ListNil, unit);
        let h: Cons = new!(Cons { v: 5, t: l.clone() });
        let l: List = enwrap!(ListCons, h.clone());
        let h: Cons = unwrap!(ListCons, l.clone());
        assert_eq!(h.clone().v, 5);
    }
}

mod structs {
    use arc_runtime::prelude::*;

    #[rewrite]
    pub struct Foo {
        pub a: i32,
        pub b: Bar,
    }

    #[rewrite]
    pub struct Bar {}

    #[rewrite(main)]
    #[test]
    fn test() {
        let b = new!(Bar {});
        let _f = new!(Foo { a: 0, b: b.clone() });
    }
}

mod unit {
    use arc_runtime::prelude::*;

    #[rewrite]
    pub enum Foo {
        FooBar(unit),
    }

    #[rewrite(main)]
    #[test]
    fn test() {
        let _x = enwrap!(FooBar, unit);
    }
}
