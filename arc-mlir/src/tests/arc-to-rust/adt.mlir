// RUN: arc-mlir-rust-test %t %s -rustinclude %s.rust-tests
// RUN: arc-mlir-rust-test %t-canon %s -rustinclude %s.rust-tests -canonicalize
// RUN: arc-mlir-rust-test %t-roundtrip-scf %s -rustinclude %s.rust-tests -canonicalize -remove-scf -canonicalize -to-scf -canonicalize

module @arctorustadt {

func @ok0(%in : !arc.adt<"i32">) -> () {
    return
  }

  func @ok2(%in : !arc.adt<"i32">) -> !arc.adt<"i32"> {
    return %in : !arc.adt<"i32">
  }

  func @ok3(%pair : !arc.adt<"(i32, bool)">) -> !arc.adt<"(i32, bool)"> {
    return %pair : !arc.adt<"(i32, bool)">
  }

  func @ok4() -> !arc.adt<"i32"> {
    %out = arc.adt_constant "4711" : !arc.adt<"i32">
    return %out : !arc.adt<"i32">
  }

  func @ok5() -> !arc.adt<"(i32, bool)"> {
    %pair = arc.adt_constant "(17, false)" : !arc.adt<"(i32, bool)">
    return %pair : !arc.adt<"(i32, bool)">
  }

}
