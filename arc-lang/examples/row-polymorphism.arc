# RUN: arc-lang %s | arc-mlir-rust-test %t - -rustinclude %s.rust-tests
# RUN: arc-lang %s | arc-mlir-rust-test %t-canon - -rustinclude %s.rust-tests -canonicalize

def foo[T](x: #{y:i32|T}): i32 = x.y

def bar(x: #{y:i32, z:i32}): i32 = x.y + x.z

def test() {
    val x = #{y:5, z:5};
    val y = #{y:5, z:5, w:9};
    foo(x);
    foo(y);
    bar(x);
}
