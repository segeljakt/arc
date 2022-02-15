// RUN: arc-mlir-rust-test %t %s -rustinclude %s.rust-tests
// RUN: arc-mlir-rust-test %t-canon %s -rustinclude %s.rust-tests -canonicalize
// RUN: arc-mlir-rust-test %t-roundtrip-scf %s -rustinclude %s.rust-tests -canonicalize -remove-scf -canonicalize -to-scf -canonicalize

module @toplevel {

  func private @cellsi32(%_0: si32) -> !arc.adt<"Cell<i32>">
    attributes {arc.rust_name="cell"}

  func @caller0(%0: si32) -> !arc.adt<"Cell<i32>"> {
    %r = call @cellsi32(%0) : (si32) -> !arc.adt<"Cell<i32>">
    return %r : !arc.adt<"Cell<i32>">
  }


  func @caller1(%0: si32) -> !arc.adt<"Cell<i32>"> {
    %f = constant @cellsi32 : (si32) -> !arc.adt<"Cell<i32>">
    %r = call_indirect %f(%0) : (si32) -> !arc.adt<"Cell<i32>">
    return %r : !arc.adt<"Cell<i32>">
  }


}
