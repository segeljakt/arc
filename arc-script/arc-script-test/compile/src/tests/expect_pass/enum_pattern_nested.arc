# XFAIL: *
# RUN: arc-script run --output=MLIR %s | arc-mlir

enum Baz {
    Some(i32),
    None,
}

enum Foo {
    Bar(Baz),
    None,
}

fun main() {
    if let Foo::Bar(Baz::Some(x)) = Foo::Bar(Baz::Some(5)) {
        unit
    } else {
        unit
    }
}