// RUN: arc-mlir %s -split-input-file -verify-diagnostics

module @toplevel {
  func @main() {
    %a = arith.constant -3.40282347E+38 : f32 // expected-note {{prior use here}}
    %b = arith.constant 0 : i1
    %c = arith.constant 1 : i1
    %d = arith.constant 0 : i1

    // expected-error@+1 {{use of value '%a' expects different type than prior uses: 'i1' vs 'f32'}}
    %0 = "arc.make_vector"(%a, %b, %c, %d) : (i1, i1, i1, i1) -> tensor<4xi1>
    return
  }
}

// -----

module @toplevel {
  func @main() {
    %b = arith.constant 0 : i1
    %c = arith.constant 1 : i1
    %d = arith.constant 0 : i1

    // expected-error@+2 {{'arc.make_vector' op requires the same element type for all operands and results}}
    // expected-note@+1 {{see current operation:}}
    %1 = "arc.make_vector"(%b, %b, %c, %d) : (i1, i1, i1, i1) -> tuple<i1,i1,i1,i1>
    return
  }
}

// -----

module @toplevel {
  func @main() {
    %b = arith.constant 0 : i1
    %c = arith.constant 1 : i1
    %d = arith.constant 0 : i1

    // expected-error@+2 {{'arc.make_vector' op requires the same element type for all operands and results}}
    // expected-note@+1 {{see current operation:}}
    %1 = "arc.make_vector"(%b, %b, %c, %d) : (i1, i1, i1, i1) -> tensor<f32>
    return
  }
}

// -----

module @toplevel {
  func @main() {
    %b = arith.constant 0 : i1
    %c = arith.constant 1 : i1
    %d = arith.constant 0 : i1

    // expected-error@+2 {{'arc.make_vector' op result #0 must be 1D tensor of any type values, but got 'tensor<4x4xi1>'}}
    // expected-note@+1 {{see current operation:}}
    %1 = "arc.make_vector"(%b, %b, %c, %d) : (i1, i1, i1, i1) -> tensor<4x4xi1>
    return
  }
}

// -----

module @toplevel {
  func @main() {
    %b = arith.constant 0 : i1
    %c = arith.constant 1 : i1
    %d = arith.constant 0 : i1

    // expected-error@+2 {{'arc.make_vector' op result must have static shape: expected 'tensor<4xi1>'}}
    // expected-note@+1 {{see current operation:}}
    %1 = "arc.make_vector"(%b, %b, %c, %d) : (i1, i1, i1, i1) -> tensor<?xi1>
    return
  }
}

// -----

module @toplevel {
  func @main() {
    %b = arith.constant 0 : i1
    %c = arith.constant 1 : i1
    %d = arith.constant 0 : i1

    // expected-error@+2 {{'arc.make_vector' op result does not match the number of operands: expected 5 but found 4 operands}}
    // expected-note@+1 {{see current operation:}}
    %1 = "arc.make_vector"(%b, %b, %c, %d) : (i1, i1, i1, i1) -> tensor<5xi1>
    return
  }
}
