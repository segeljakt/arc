# RUN: arc-lang %s | arc-mlir-rust-test %t - -rustinclude %s.rust-tests
# RUN: arc-lang %s | arc-mlir-rust-test %t-canon - -rustinclude %s.rust-tests -canonicalize

enum Foo[T] {
    Bar(T)
}

def qux() {
    Foo::Bar(1);
    Foo::Bar(1.0);
}
