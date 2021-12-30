# RUN: arc-lang %s | arc-mlir-rust-test %t - -rustinclude %s.rust-tests
# RUN: arc-lang %s | arc-mlir-rust-test %t-canon - -rustinclude %s.rust-tests -canonicalize

def test() {
# ANCHOR: example
# Pattern matching on enums
val a = Some(3);

match a {
    Some(2) => 2,
    Some(x) => x,
    None => 0
};
# ANCHOR_END: example
}
