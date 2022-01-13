#![allow(unused)]

mod basic1 {
    use arc_codegen::*;
    #[rewrite]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    #[rewrite]
    pub enum Foo {
        FooBar(i32),
        FooBaz(f32),
    }
}

mod basic2 {
    use arc_codegen::*;
    #[rewrite]
    pub struct A {
        pub b: B,
    }

    #[rewrite]
    pub struct B {
        pub c: i32,
    }
}

mod basic3 {
    use arc_codegen::*;
    #[rewrite]
    pub enum A {
        AB(B),
        AC(C),
    }

    #[rewrite]
    pub struct B {
        pub v: i32,
    }

    #[rewrite]
    pub struct C {}
}

mod list {
    use arc_codegen::*;

    #[rewrite]
    pub enum List {
        ListCons(Cons),
        ListNil(Nil),
    }

    #[rewrite]
    pub struct Cons {
        pub val: i32,
        pub tail: List,
    }

    #[rewrite]
    pub struct Nil {}

    #[test]
    fn test() {
        let l: List = enwrap!(ListNil, new!(Nil {}));
        let l: List = enwrap!(ListCons, new!(Cons { val: 5, tail: l }));
        let h: Cons = unwrap!(ListCons, l);
        assert_eq!(h.val, 5);
    }
}

mod structs {
    use arc_codegen::*;

    #[rewrite]
    pub struct Foo {
        pub a: i32,
        pub b: Bar,
    }

    #[rewrite]
    pub struct Bar {}

    #[test]
    fn test() {
        let x: Foo = new!(Foo { a: 0, b: new!(Bar {}) });
    }
}

mod unit {
    use arc_codegen::*;

    #[rewrite]
    pub enum Foo {
        FooBar(unit),
    }

    #[test]
    fn test() {
        let x: Foo = enwrap!(FooBar, ());
    }
}
