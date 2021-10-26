//====- LowerToRust.cpp - Lowering from Arc+MLIR to Rust --===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// This file implements a conversion of arc-mlir functions containing
// blocking operations to FSMs. Boilerplate stolen from the MLIR Toy
// dialect.
//
//===----------------------------------------------------------------------===//

#include "Arc/Passes.h"
#include "mlir/Analysis/Liveness.h"
#include "mlir/IR/BuiltinTypes.h"
#include "mlir/IR/BuiltinOps.h"
#include "mlir/IR/MLIRContext.h"
#include "mlir/Support/LLVM.h"
#include "mlir/Support/LogicalResult.h"
#include <mlir/Transforms/DialectConversion.h>

using namespace mlir;
using namespace arc;

//===----------------------------------------------------------------------===//
// ToFSMPass
//===----------------------------------------------------------------------===//

/// This is a lowering of arc operations to the Rust dialect.
namespace {
struct ToFSMPass : public ToFSMBase<ToFSMPass> {
  void runOnOperation() final;
};
} // end anonymous namespace.

void ToFSMPass::runOnOperation() {
  // getOperation().dump();
  // myAnalysis.dump();

  getOperation().walk([&](mlir::Operation *op) {
    if (FuncOp f = dyn_cast<mlir::FuncOp>(op)) {
      llvm::errs() << "=== Function ===\n";
      llvm::errs() << f << "\n";

      Liveness live(f);
      live.dump();
    }
  });

  // getOperation().walk([&](mlir::Operation *op) {
  //   // process Operation `op`.
  //   llvm::errs() << *op << "\n";
  // });

  // signalPassFailure();
}

std::unique_ptr<OperationPass<ModuleOp>> arc::createToFSMPass() {
  return std::make_unique<ToFSMPass>();
}
