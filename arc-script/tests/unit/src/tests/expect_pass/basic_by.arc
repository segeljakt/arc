# RUN: arc-lang %s | arc-mlir-rust-test %t - -rustinclude %s.rust-tests
# RUN: arc-lang %s | arc-mlir-rust-test %t-canon - -rustinclude %s.rust-tests -canonicalize

fun test(): i32 {
    val x = 1 by 2;
    val v = x.value;
    val k = x.key;
    v + k
}
