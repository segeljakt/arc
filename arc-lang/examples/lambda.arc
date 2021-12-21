# XFAIL: arc-lang %s | arc-mlir-rust-test %t - -rustinclude %s.rust-tests
# XFAIL: arc-lang %s | arc-mlir-rust-test %t-canon - -rustinclude %s.rust-tests -canonicalize

def test() {
    val a = 1;
    val b = 2;
    val x = fun(c): a + b + c;
    x(3);
}
