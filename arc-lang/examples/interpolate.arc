# RUN: arc-lang %s | arc-mlir-rust-test %t - -rustinclude %s.rust-tests
# RUN: arc-lang %s | arc-mlir-rust-test %t-canon - -rustinclude %s.rust-tests -canonicalize

def test() {
  val a = 1;
  " $a  $a  $a ";
  " ${a+a+a}  ${a+a+1}  ${a+a+1} ";
}
