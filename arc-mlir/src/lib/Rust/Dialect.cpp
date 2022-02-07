//===- Rust IR Dialect registration in MLIR -===//
//
// Copyright 2019 The MLIR Authors.
// Copyright 2019 KTH Royal Institute of Technology.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// =============================================================================
//
// This file implements the dialect for the Rust IR: custom type parsing and
// operation verification.
//
//===----------------------------------------------------------------------===//

#include "Rust/Rust.h"
#include "Rust/RustPrinterStream.h"
#include <llvm/Support/CommandLine.h>
#include <llvm/Support/FileSystem.h>
#include <llvm/Support/Path.h>
#include <llvm/Support/raw_ostream.h>
#include <mlir/IR/BuiltinOps.h>
#include <mlir/IR/BuiltinTypes.h>
#include <mlir/IR/DialectImplementation.h>

#include "Rust/RustDialect.cpp.inc"

using namespace mlir;
using namespace rust;
using namespace types;

static llvm::cl::opt<std::string>
    crateNameOverride("rustcratename",
                      llvm::cl::desc("Override name of output crate"),
                      llvm::cl::value_desc("cratename"));

static llvm::cl::opt<std::string>
    rustModuleFile("rustfile",
                   llvm::cl::desc("Write all rust output to a single file"),
                   llvm::cl::value_desc("filename"));

static llvm::cl::opt<std::string>
    rustInclude("rustinclude",
                llvm::cl::desc("Include this file into the generated module"),
                llvm::cl::value_desc("filename"));

//===----------------------------------------------------------------------===//
// RustDialect
//===----------------------------------------------------------------------===//

void RustDialect::initialize() {
  addOperations<
#define GET_OP_LIST
#include "Rust/Rust.cpp.inc"
      >();
  addTypes<RustType>();
  addTypes<RustEnumType>();
  addTypes<RustStructType>();
  addTypes<RustStreamType>();
  addTypes<RustSinkStreamType>();
  addTypes<RustSourceStreamType>();
  addTypes<RustTensorType>();
  addTypes<RustTupleType>();

  auto ctx = getContext();

  floatTy = RustType::get(ctx, "f32");
  doubleTy = RustType::get(ctx, "f64");
  boolTy = RustType::get(ctx, "bool");
  i8Ty = RustType::get(ctx, "i8");
  i16Ty = RustType::get(ctx, "i16");
  i32Ty = RustType::get(ctx, "i32");
  i64Ty = RustType::get(ctx, "i64");
  u8Ty = RustType::get(ctx, "u8");
  u16Ty = RustType::get(ctx, "u16");
  u32Ty = RustType::get(ctx, "u32");
  u64Ty = RustType::get(ctx, "u64");
  noneTy = RustType::get(ctx, "unit");
}

//===----------------------------------------------------------------------===//
// RustDialect Type Parsing
//===----------------------------------------------------------------------===//

Type RustDialect::parseType(DialectAsmParser &parser) const {
  //  StringRef type;
  StringRef tyData = parser.getFullSymbolSpec();
  // if (failed(parser.parseKeyword(&type)))
  //   return nullptr;
  return RustType::get(getContext(), tyData);
}

//===----------------------------------------------------------------------===//
// RustDialect Type Printing
//===----------------------------------------------------------------------===//

void RustDialect::printType(Type type, DialectAsmPrinter &os) const {
  if (auto t = type.dyn_cast<RustType>())
    t.print(os);
  else if (auto t = type.dyn_cast<RustStreamType>())
    t.print(os);
  else if (auto t = type.dyn_cast<RustSinkStreamType>())
    t.print(os);
  else if (auto t = type.dyn_cast<RustSourceStreamType>())
    t.print(os);
  else if (auto t = type.dyn_cast<RustStructType>())
    t.print(os);
  else if (auto t = type.dyn_cast<RustTensorType>())
    t.print(os);
  else if (auto t = type.dyn_cast<RustTupleType>())
    t.print(os);
  else if (auto t = type.dyn_cast<RustEnumType>())
    t.print(os);
  else
    llvm_unreachable("Unhandled Rust type");
}

//===----------------------------------------------------------------------===//
// Rust Operations
//===----------------------------------------------------------------------===//

/// Hook for FunctionLike verifier.
LogicalResult RustFuncOp::verifyType() {
  Type type = getTypeAttr().getValue();
  if (!type.isa<FunctionType>())
    return emitOpError("requires '" + getTypeAttrName() +
                       "' attribute of function type");

  if ((*this)->hasAttr("arc.task_name") && (*this)->hasAttr("arc.mod_name") &&
      (*this)->hasAttr("arc.is_event_handler")) {
    // We want to check:
    //  * The first argument is a struct
    //  * The second argument is an enum
    //  * The this argument is a stream type
    // This is enough to avoid segfaults in later stages, errors undetected here
    // will trigger type errors when the generated Rust is compiled.
    if (getNumArguments() != 3)
      return emitOpError(
          ": task event handlers are expected to have 3 arguments, found " +
          Twine(getNumArguments()));
    if (!front().getArgument(0).getType().isa<RustStructType>())
      return emitOpError(": The first argument to a task event handler is "
                         "expected to be a struct");
    if (!front().getArgument(1).getType().isa<RustEnumType>())
      return emitOpError(": The second argument to a task event handler is "
                         "expected to be an enum");
    if (!front().getArgument(2).getType().isa<RustStreamType>())
      return emitOpError(": The third argument to a task event handler is "
                         "expected to be a stream");
  }
  return success();
}

/// Hook for FunctionLike verifier.
LogicalResult RustExtFuncOp::verifyType() {
  Type type = getTypeAttr().getValue();
  if (!type.isa<FunctionType>())
    return emitOpError("requires '" + getTypeAttrName() +
                       "' attribute of function type");
  return success();
}

/// Verifies the body of the function.
LogicalResult RustFuncOp::verifyBody() {
  unsigned numFuncArguments = getNumArguments();
  unsigned numBlockArguments = empty() ? 0 : front().getNumArguments();
  if (numBlockArguments != numFuncArguments)
    return emitOpError() << "expected " << numFuncArguments
                         << " arguments to body region, found "
                         << numBlockArguments;

  ArrayRef<Type> funcArgTypes = getType().getInputs();
  for (unsigned i = 0; i < numFuncArguments; ++i) {
    Type blockArgType = front().getArgument(i).getType();
    if (funcArgTypes[i] != blockArgType)
      return emitOpError() << "expected body region argument #" << i
                           << " to be of type " << funcArgTypes[i] << ", found "
                           << blockArgType;
  }

  return success();
}

static LogicalResult verify(RustReturnOp returnOp) {
  RustFuncOp function = returnOp->getParentOfType<RustFuncOp>();

  if (!function)
    return returnOp.emitOpError("expects 'rust.func' parent");

  FunctionType funType = function.getType();

  if (funType.getNumResults() == 0 && returnOp.operands())
    return returnOp.emitOpError("cannot return a value from a void function");

  if (!returnOp.operands() && funType.getNumResults())
    return returnOp.emitOpError("operation must return a ")
           << funType.getResult(0) << " value";

  if (!funType.getNumResults())
    return success();

  Type returnType = returnOp.getOperand(0).getType();
  Type funReturnType = funType.getResult(0);

  if (funReturnType != returnType) {
    return returnOp.emitOpError("result type does not match the type of the "
                                "function: expected ")
           << funReturnType << " but found " << returnType;
  }
  return success();
}

//===----------------------------------------------------------------------===//
// RustDialect Rust Printing
//===----------------------------------------------------------------------===//
namespace rust {
RustPrinterStream &operator<<(RustPrinterStream &os, const Value &v) {
  return os.print(v);
}

RustPrinterStream &operator<<(RustPrinterStream &os, const Type &type) {
  if (auto t = type.dyn_cast<RustType>())
    os.print(os.getBodyStream(), t.getRustType());
  else if (auto t = type.dyn_cast<RustStructType>())
    os.print(os.getBodyStream(), t);
  else if (auto t = type.dyn_cast<RustStreamType>())
    os.print(os.getBodyStream(), t);
  else if (auto t = type.dyn_cast<RustSinkStreamType>())
    os.print(os.getBodyStream(), t);
  else if (auto t = type.dyn_cast<RustSourceStreamType>())
    os.print(os.getBodyStream(), t);
  else if (auto t = type.dyn_cast<RustTensorType>())
    t.printAsRust(os);
  else if (auto t = type.dyn_cast<RustTupleType>())
    t.printAsRust(os);
  else if (auto t = type.dyn_cast<RustEnumType>())
    os.print(os.getBodyStream(), t);
  else if (auto t = type.dyn_cast<FunctionType>())
    os.print(os.getBodyStream(), t);
  else
    os << "<not-a-rust-type>";
  return os;
}
} // namespace rust

LogicalResult rust::writeModuleAsInline(ModuleOp module, llvm::raw_ostream &o) {

  if (!module.getName()) {
    emitError(module.getLoc())
        << "Rust module is missing a name (is the module implicitly created?)";
    return failure();
  }

  RustPrinterStream PS(module.getName()->str(), rustInclude);

  for (Operation &operation : module) {
    if (RustFuncOp op = dyn_cast<RustFuncOp>(operation))
      op.writeRust(PS);
    else if (RustExtFuncOp op = dyn_cast<RustExtFuncOp>(operation))
      op.writeRust(PS);
  }

  PS.flush(o);

  return success();
}

static RustPrinterStream &writeRust(Operation &operation,
                                    RustPrinterStream &PS) {
  if (RustReturnOp op = dyn_cast<RustReturnOp>(operation))
    op.writeRust(PS);
  else if (RustConstantOp op = dyn_cast<RustConstantOp>(operation))
    op.writeRust(PS);
  else if (RustUnaryOp op = dyn_cast<RustUnaryOp>(operation))
    op.writeRust(PS);
  else if (RustBinaryOp op = dyn_cast<RustBinaryOp>(operation))
    op.writeRust(PS);
  else if (RustBinaryRcOp op = dyn_cast<RustBinaryRcOp>(operation))
    op.writeRust(PS);
  else if (RustCallOp op = dyn_cast<RustCallOp>(operation))
    op.writeRust(PS);
  else if (RustCallIndirectOp op = dyn_cast<RustCallIndirectOp>(operation))
    op.writeRust(PS);
  else if (RustCompOp op = dyn_cast<RustCompOp>(operation))
    op.writeRust(PS);
  else if (RustEnumAccessOp op = dyn_cast<RustEnumAccessOp>(operation))
    op.writeRust(PS);
  else if (RustEnumCheckOp op = dyn_cast<RustEnumCheckOp>(operation))
    op.writeRust(PS);
  else if (RustFieldAccessOp op = dyn_cast<RustFieldAccessOp>(operation))
    op.writeRust(PS);
  else if (RustIfOp op = dyn_cast<RustIfOp>(operation))
    op.writeRust(PS);
  else if (RustBlockResultOp op = dyn_cast<RustBlockResultOp>(operation))
    op.writeRust(PS);
  else if (RustLoopOp op = dyn_cast<RustLoopOp>(operation))
    op.writeRust(PS);
  else if (RustLoopBreakOp op = dyn_cast<RustLoopBreakOp>(operation))
    op.writeRust(PS);
  else if (RustLoopConditionOp op = dyn_cast<RustLoopConditionOp>(operation))
    op.writeRust(PS);
  else if (RustLoopYieldOp op = dyn_cast<RustLoopYieldOp>(operation))
    op.writeRust(PS);
  else if (RustMakeEnumOp op = dyn_cast<RustMakeEnumOp>(operation))
    op.writeRust(PS);
  else if (RustMakeStructOp op = dyn_cast<RustMakeStructOp>(operation))
    op.writeRust(PS);
  else if (RustMethodCallOp op = dyn_cast<RustMethodCallOp>(operation))
    op.writeRust(PS);
  else if (RustTensorOp op = dyn_cast<RustTensorOp>(operation))
    op.writeRust(PS);
  else if (RustTupleOp op = dyn_cast<RustTupleOp>(operation))
    op.writeRust(PS);
  else if (RustEmitOp op = dyn_cast<RustEmitOp>(operation))
    op.writeRust(PS);
  else if (RustPanicOp op = dyn_cast<RustPanicOp>(operation))
    op.writeRust(PS);
  else {
    operation.emitError("Unsupported operation");
  }
  return PS;
}

void RustCallOp::writeRust(RustPrinterStream &PS) {
  bool has_result = getNumResults();
  if (has_result) {
    auto r = getResult(0);
    PS << "let ";
    PS.printAsArg(r) << ":" << r.getType() << " = ";
  }

  StringRef callee = getCallee();
  StringAttr calleeName = StringAttr::get(this->getContext(), getCallee());
  Operation *target = SymbolTable::lookupNearestSymbolFrom(*this, calleeName);
  if (target && target->hasAttr("arc.rust_name"))
    callee = target->getAttrOfType<StringAttr>("arc.rust_name").getValue();

  PS << callee << "(";
  for (auto a : getOperands())
    PS << a << ", ";
  PS << ")";
  PS << ";\n";
}

void RustCallIndirectOp::writeRust(RustPrinterStream &PS) {
  bool has_result = getNumResults();
  if (has_result) {
    auto r = getResult(0);
    PS << "let ";
    PS.printAsArg(r) << ":" << r.getType() << " = ";
  }
  PS << "(" << getCallee() << ")(";
  for (auto a : getArgOperands())
    PS << a << ", ";
  PS << ")";
  PS << ";\n";
}

// Write this function as Rust code to os
void RustFuncOp::writeRust(RustPrinterStream &PS) {
  bool isTask = (*this)->hasAttr("arc.task_name") &&
                (*this)->hasAttr("arc.mod_name") &&
                (*this)->hasAttr("arc.is_event_handler");
  bool isMethod = (*this)->hasAttr("arc.task_name") &&
                  (*this)->hasAttr("arc.mod_name") &&
                  !(*this)->hasAttr("arc.is_event_handler");

  if (isTask) {
    auto modName =
        (*this)->getAttrOfType<StringAttr>("arc.mod_name").getValue();
    auto taskName =
        (*this)->getAttrOfType<StringAttr>("arc.task_name").getValue();

    // Construct rewrite directive
    PS << "#[rewrite(";
    mlir::ModuleOp m = cast<mlir::ModuleOp>(getOperation()->getParentOp());
    for (auto f : m.getOps<RustFuncOp>()) {
      if (f->hasAttr("arc.is_event_handler"))
        PS << "on_event = \"" << f.getName() << "\", ";
      if (f->hasAttr("arc.is_init"))
        PS << "on_start = \"" << f.getName() << "\", ";
    }
    PS << ")]\n";

    // Generate the named type for the task
    PS << "mod " << modName << "{\n";
    // The state type
    PS << "pub struct " << taskName << " {\n";
    RustStructType st = front().getArgument(0).getType().cast<RustStructType>();
    for (unsigned i = 0; i < st.getNumFields(); i++) {
      PS << st.getFieldName(i) << ": " << st.getFieldType(i) << ",\n";
    }
    PS << "}\n";
    std::string modBaseName = modName.str() + "::";
    std::string stateTypeName = modBaseName + taskName.str();
    PS.addAlias(st, stateTypeName);

    RustEnumType inTy = front().getArgument(1).getType().cast<RustEnumType>();
    PS << "#[rewrite]\n"
       << "pub enum IInterface {\n";
    for (unsigned i = 0; i < inTy.getNumVariants(); i++) {
      PS << inTy.getVariantName(i) << "(" << inTy.getVariantType(i) << "),\n";
    }
    PS << "}\n";
    PS.addAlias(inTy, modBaseName + "IInterface");

    RustStreamType outStream =
        front().getArgument(2).getType().cast<RustStreamType>();
    RustEnumType outTy = outStream.getType().cast<RustEnumType>();
    PS << "#[rewrite]\n"
       << "pub enum OInterface {\n";
    for (unsigned i = 0; i < outTy.getNumVariants(); i++) {
      PS << outTy.getVariantName(i) << "(" << outTy.getVariantType(i) << "),\n";
    }
    PS << "}\n";
    PS.addAlias(outTy, modBaseName + "OInterface");

    PS << "}\n";
  }
  if (isTask || isMethod) {
    PS << "impl "
       << (*this)->getAttrOfType<StringAttr>("arc.mod_name").getValue()
       << "::" << (*this)->getAttrOfType<StringAttr>("arc.task_name").getValue()
       << " {\n";
  }

  PS << "// " << (isTask ? "Task" : "") << (isMethod ? "Method" : "") << "\n";
  PS << "pub fn ";
  if ((*this)->hasAttr("arc.rust_name"))
    PS << (*this)->getAttrOfType<StringAttr>("arc.rust_name").getValue();
  else
    PS << getName();
  PS << "(";

  // Dump the function arguments
  unsigned numFuncArguments = getNumArguments();
  for (unsigned i = 0; i < numFuncArguments; i++) {
    Value v = front().getArgument(i);
    if (isTask && i == 1) {
      PS.addAlias(v, "event");
      PS << ", event : " << v.getType();
    } else if ((isTask || isMethod) && i == 0) {
      PS << "&mut self";
      PS.addAlias(v, "self");
    } else if (isTask && i == 2) {
      // We skip this argument
    } else {
      if (i != 0)
        PS << ", ";
      PS.printAsArg(v) << ": " << v.getType();
    }
  }
  PS << ") ";
  if (getType().getNumResults()) { // The return type
    PS << "-> " << getType().getResult(0) << " ";
  }

  // Dumping the body
  PS << "{\n";
  for (Operation &operation : this->body().front()) {
    ::writeRust(operation, PS);
  }
  PS << "}\n";
  if (isTask || isMethod)
    PS << "}\n";
  PS.clearAliases();
}

// Write this function as Rust code to os
void RustExtFuncOp::writeRust(RustPrinterStream &PS) {}

void RustReturnOp::writeRust(RustPrinterStream &PS) {
  if (getNumOperands())
    PS << "return " << getOperand(0) << ";\n";
  else
    PS << "return;\n";
}

void RustConstantOp::writeRust(RustPrinterStream &PS) { PS.getConstant(*this); }

void RustUnaryOp::writeRust(RustPrinterStream &PS) {
  auto r = getResult();
  PS << "let ";
  PS.printAsArg(r) << ":" << r.getType() << " = " << getOperator() << "("
                   << getOperand() << ")"
                   << ";\n";
}

void RustLoopOp::writeRust(RustPrinterStream &PS) {
  PS << "// Loop variables\n";
  Block::BlockArgListType before_args = before().front().getArguments();
  Block::BlockArgListType after_args = after().front().getArguments();
  OperandRange inits = this->inits();

  assert(inits.size() == before_args.size());

  // Construct a mutable variable for each loop variable
  for (unsigned idx = 0; idx < inits.size(); idx++) {
    Value v = before_args[idx];
    Value i = inits[idx];

    PS << "let mut ";
    PS.printAsArg(v) << ":" << v.getType() << " = ";
    PS.print(i) << ";\n";
  }

  // Construct variables for catching the result
  if (getNumResults() != 0)
    PS << "let (";
  for (unsigned i = 0; i < getNumResults(); i++) {
    auto r = getResult(i);
    PS.printAsArg(r) << ",";
  }
  if (getNumResults() != 0)
    PS << ") : (";
  for (unsigned i = 0; i < getNumResults(); i++) {
    auto r = getResult(i);
    PS << r.getType() << ",";
  }
  if (getNumResults() != 0)
    PS << ") = ";

  // Emit the loop body
  PS << "loop {\n";
  PS << "// Before\n";
  for (Operation &operation : before().front())
    ::writeRust(operation, PS);
  RustLoopConditionOp cond = getConditionOp();
  auto passed_on = cond.args();
  PS << "// Pass on state from the before to the after part\n";
  assert(passed_on.size() == after_args.size());

  for (unsigned idx = 0; idx < after_args.size(); idx++) {
    Value v = after_args[idx];
    Value i = passed_on[idx];

    PS << "let ";
    PS.printAsArg(v) << ":" << v.getType() << " = ";
    PS.print(i) << ";\n";
  }

  PS << "// After\n";
  for (Operation &operation : after().front())
    ::writeRust(operation, PS);

  RustLoopYieldOp yield = getYieldOp();
  auto updated = yield.results();
  PS << "// Update the loop variables for the next iteration\n";
  assert(before_args.size() == updated.size());
  for (unsigned idx = 0; idx < before_args.size(); idx++) {
    Value v = before_args[idx];
    Value u = updated[idx];

    PS.printAsLValue(v) << " = ";
    PS.print(u) << ";\n";
  }

  PS << "};\n";
}

static LogicalResult verify(RustLoopBreakOp breakOp) {
  // HasParent<"RustLoopOp"> in the .td apparently only looks at the
  // immediate parent and not all parents. Therefore we have to check
  // that we are inside a loop here.
  RustLoopOp loopOp = breakOp->getParentOfType<RustLoopOp>();
  if (!loopOp)
    return breakOp.emitOpError("must be inside a rust.loop region");

  // Now check that what we return matches the type of the parent
  unsigned noofResults = breakOp.getNumOperands();
  unsigned noofParentResults = loopOp.getNumResults();

  if (noofResults != noofParentResults)
    return breakOp.emitOpError("returns ")
           << noofResults << " values parent expects " << noofParentResults;

  auto breakTypes = breakOp.getOperandTypes();
  auto loopTypes = loopOp.getResultTypes();
  for (unsigned i = 0; i < noofResults; i++)
    if (breakTypes[i] != loopTypes[i])
      return breakOp.emitOpError(
          "type signature does not match signature of parent 'rust.loop'");

  return success();
}

void RustLoopBreakOp::writeRust(RustPrinterStream &PS) {
  PS << "break";
  if (getNumOperands() != 1) {
    PS << " (";
    for (auto arg : results())
      PS << arg << ",";
    PS << ")";
  }
  PS << ";\n";
}

void RustLoopConditionOp::writeRust(RustPrinterStream &PS) {
  PS << "// Loop condition\n";
  PS << "if !" << getOperand(0) << " {\n";
  PS << "break";
  if (getNumOperands() != 1) {
    PS << " (";
    for (auto arg : args())
      PS << arg << ",";
    PS << ")";
  }
  PS << ";\n";
  PS << "}\n";
}

void RustLoopYieldOp::writeRust(RustPrinterStream &PS) {
  PS << "// Loop yield\n";
}

void RustMakeEnumOp::writeRust(RustPrinterStream &PS) {
  auto r = getResult();
  RustEnumType et = r.getType().cast<RustEnumType>();
  PS << "let ";
  PS.printAsArg(r) << ":" << et << " = enwrap!(" << et << "::" << variant()
                   << ", ";
  if (values().size())
    PS << values()[0];
  else
    PS << "()";
  PS << ");\n";
}

void RustMakeStructOp::writeRust(RustPrinterStream &PS) {
  auto r = getResult();
  RustStructType st = r.getType().cast<RustStructType>();
  PS << "let ";
  PS.printAsArg(r) << ":" << st << " = new!(" << st << " { ";
  auto args = operands();
  for (unsigned i = 0; i < args.size(); i++) {
    if (i != 0)
      PS << ", ";
    auto v = args[i];
    PS << st.getFieldName(i) << " : " << v;
  }
  PS << "});\n";
}

void RustMethodCallOp::writeRust(RustPrinterStream &PS) {
  auto r = getResult();
  PS << "let ";
  PS.printAsArg(r) << ":" << r.getType() << " = " << obj() << "." << getMethod()
                   << "(";
  auto args = operands();
  for (unsigned i = 0; i < args.size(); i++) {
    if (i != 0)
      PS << ", ";
    auto v = args[i];
    PS << v;
  }
  PS << ");\n";
}

void RustBinaryOp::writeRust(RustPrinterStream &PS) {
  auto r = getResult();
  PS << "let ";
  PS.printAsArg(r) << ":" << r.getType() << " = " << LHS() << " "
                   << getOperator() << " " << RHS() << ";\n";
}

void RustBinaryRcOp::writeRust(RustPrinterStream &PS) {
  auto r = getResult();
  PS << "let ";
  PS.printAsArg(r) << ":" << r.getType() << " = Rc::new(&*" << LHS() << " "
                   << getOperator() << " &*" << RHS() << ");\n";
}

void RustCompOp::writeRust(RustPrinterStream &PS) {
  auto r = getResult();
  PS << "let ";
  PS.printAsArg(r) << ":" << r.getType() << " = " << LHS() << " "
                   << getOperator() << " " << RHS() << ";\n";
}

void RustEmitOp::writeRust(RustPrinterStream &PS) {
  PS << "self.emit(" << value() << ");\n";
}

void RustEnumAccessOp::writeRust(RustPrinterStream &PS) {
  auto r = getResult();
  RustEnumType et = theEnum().getType().cast<RustEnumType>();
  PS << "let ";
  PS.printAsArg(r) << ":" << r.getType() << " = unwrap!(" << et
                   << "::" << getVariant() << ", " << theEnum() << ");\n";
}

void RustEnumCheckOp::writeRust(RustPrinterStream &PS) {
  auto r = getResult();
  RustEnumType et = theEnum().getType().cast<RustEnumType>();
  PS << "let ";
  PS.printAsArg(r) << ":" << r.getType() << " = is!(" << et
                   << "::" << getVariant() << ", " << theEnum() << ");\n";
}

void RustFieldAccessOp::writeRust(RustPrinterStream &PS) {
  auto r = getResult();
  PS << "let ";
  PS.printAsArg(r) << ":" << r.getType() << " = access!(" << aggregate() << ", "
                   << getField() << ");\n";
}

void RustIfOp::writeRust(RustPrinterStream &PS) {
  if (getNumResults() != 0) {
    auto r = getResult(0);
    PS << "let ";
    PS.printAsArg(r) << ":" << r.getType() << " =";
  }
  // No clone is needed here as it will be inserted by the block
  // result.
  PS << " if " << getOperand() << " {\n";
  for (Operation &operation : thenRegion().front())
    ::writeRust(operation, PS);
  PS << "} else {\n";
  for (Operation &operation : elseRegion().front())
    ::writeRust(operation, PS);
  PS << "};\n";
}

static LogicalResult verify(RustIfOp ifOp) {
  // Check that the terminators are a rust.loop.break or a
  // rust.block.result.
  auto &thenTerm = ifOp.thenRegion().getBlocks().back().back();
  auto &elseTerm = ifOp.elseRegion().getBlocks().back().back();

  if ((isa<RustBlockResultOp>(thenTerm) || isa<RustLoopBreakOp>(thenTerm) ||
       isa<RustReturnOp>(thenTerm)) &&
      (isa<RustBlockResultOp>(elseTerm) || isa<RustLoopBreakOp>(elseTerm) ||
       isa<RustReturnOp>(elseTerm)))
    return success();
  return ifOp.emitOpError("expects terminators to be 'rust.loop.break' or "
                          "'rust.block.result' operations");
}

//===----------------------------------------------------------------------===//
// RustLoopOp, stolen from SCF
//===----------------------------------------------------------------------===//

RustLoopConditionOp RustLoopOp::getConditionOp() {
  return cast<RustLoopConditionOp>(before().front().getTerminator());
}

RustLoopYieldOp RustLoopOp::getYieldOp() {
  return cast<RustLoopYieldOp>(after().front().getTerminator());
}

void RustBlockResultOp::writeRust(RustPrinterStream &PS) {
  if (getNumOperands() == 0) {
    PS << "// No value\n";
    return;
  }
  auto r = getOperand(0);
  PS << r << "\n";
}

void RustPanicOp::writeRust(RustPrinterStream &PS) {
  PS << "panic!(";
  if (msg().hasValue())
    PS << "\"" << msg().getValue() << "\"";
  PS << ");\n";
}

void RustTensorOp::writeRust(RustPrinterStream &PS) {
  auto r = getResult();
  PS << "let ";
  PS.printAsArg(r) << ":" << r.getType()
                   << " = Rc::new(Array::from_shape_vec((";
  RustTensorType t = result().getType().cast<RustTensorType>();
  for (int64_t d : t.getDimensions())
    PS << d << ", ";
  PS << "), vec![";
  auto args = values();
  for (unsigned i = 0; i < args.size(); i++) {
    auto v = args[i];
    PS << v << ", ";
  }
  PS << "]).unwrap());\n";
}

void RustTupleOp::writeRust(RustPrinterStream &PS) {
  auto r = getResult();
  PS << "let ";
  PS.printAsArg(r) << ":" << r.getType() << " = (";
  auto args = operands();
  for (unsigned i = 0; i < args.size(); i++) {
    auto v = args[i];
    PS << v << ", ";
  }
  PS << ");\n";
}

//===----------------------------------------------------------------------===//
// Crate versions
//===----------------------------------------------------------------------===//
namespace rust {
const char *CrateVersions::ndarray = "0.13.0";
} // namespace rust

//===----------------------------------------------------------------------===//
// Rust types
//===----------------------------------------------------------------------===//
namespace rust {
namespace types {

std::string getTypeString(Type type) {
  if (auto t = type.dyn_cast<RustStructType>())
    return t.getRustType();
  if (auto t = type.dyn_cast<RustStreamType>())
    return t.getRustType();
  if (auto t = type.dyn_cast<RustSinkStreamType>())
    return t.getRustType();
  if (auto t = type.dyn_cast<RustSourceStreamType>())
    return t.getRustType();
  if (auto t = type.dyn_cast<RustTupleType>())
    return t.getRustType();
  if (auto t = type.dyn_cast<RustTensorType>())
    return t.getRustType();
  if (auto t = type.dyn_cast<RustType>())
    return t.getRustType().str();
  if (auto t = type.dyn_cast<RustEnumType>())
    return t.getRustType();
  return "<unsupported type>";
}

static std::string getTypeSignature(Type type) {
  if (auto t = type.dyn_cast<RustStreamType>())
    return t.getSignature();
  if (auto t = type.dyn_cast<RustSinkStreamType>())
    return t.getSignature();
  if (auto t = type.dyn_cast<RustSourceStreamType>())
    return t.getSignature();
  if (auto t = type.dyn_cast<RustStructType>())
    return t.getSignature();
  if (auto t = type.dyn_cast<RustTupleType>())
    return t.getSignature();
  if (auto t = type.dyn_cast<RustTensorType>())
    return t.getSignature();
  if (auto t = type.dyn_cast<RustType>())
    return t.getSignature();
  if (auto t = type.dyn_cast<RustEnumType>())
    return t.getSignature();
  return "<unsupported type>";
}

struct RustTypeStorage : public TypeStorage {
  RustTypeStorage(std::string type) : rustType(type) {}

  std::string rustType;

  using KeyTy = std::string;

  bool operator==(const KeyTy &key) const { return key == KeyTy(rustType); }

  static llvm::hash_code hashKey(const KeyTy &key) {
    return llvm::hash_combine(key);
  }

  static RustTypeStorage *construct(TypeStorageAllocator &allocator,
                                    const KeyTy &key) {
    return new (allocator.allocate<RustTypeStorage>()) RustTypeStorage(key);
  }

  raw_ostream &printAsRust(raw_ostream &os) const {
    os << rustType;
    return os;
  }
  std::string getSignature() const;
};

RustType RustType::get(MLIRContext *context, StringRef type) {
  return Base::get(context, type);
}

StringRef RustType::getRustType() const { return getImpl()->rustType; }

void RustType::print(DialectAsmPrinter &os) const { os << getRustType(); }

raw_ostream &RustType::printAsRust(raw_ostream &os) const {
  return getImpl()->printAsRust(os);
}

bool RustType::isBool() const { return getRustType().equals("bool"); }

bool RustType::isUnit() const { return getRustType().equals("()"); }

RustType RustType::getFloatTy(RustDialect *dialect) { return dialect->floatTy; }

RustType RustType::getDoubleTy(RustDialect *dialect) {
  return dialect->doubleTy;
}

RustType RustType::getNoneTy(RustDialect *dialect) { return dialect->noneTy; }

RustType RustType::getIntegerTy(RustDialect *dialect, IntegerType ty) {
  switch (ty.getWidth()) {
  case 1:
    return dialect->boolTy;
  case 8:
    return ty.isUnsigned() ? dialect->u8Ty : dialect->i8Ty;
  case 16:
    return ty.isUnsigned() ? dialect->u16Ty : dialect->i16Ty;
  case 32:
    return ty.isUnsigned() ? dialect->u32Ty : dialect->i32Ty;
  case 64:
    return ty.isUnsigned() ? dialect->u64Ty : dialect->i64Ty;
  default:
    return emitError(UnknownLoc::get(dialect->getContext()), "unhandled type"),
           nullptr;
  }
}

std::string RustType::getSignature() const { return getImpl()->getSignature(); }

std::string RustTypeStorage::getSignature() const { return rustType; }

//===----------------------------------------------------------------------===//
// RustEnumType
//===----------------------------------------------------------------------===//

struct RustEnumTypeStorage : public TypeStorage {
  RustEnumTypeStorage(ArrayRef<RustEnumType::EnumVariantTy> fields, unsigned id)
      : enumVariants(fields.begin(), fields.end()), id(id) {
    std::string str;
    llvm::raw_string_ostream s(str);
    s << "Enum";

    for (auto &f : fields) {
      StringRef fieldName = f.first.getValue();
      s << fieldName.size() << fieldName;
      RustType vt = f.second.dyn_cast<RustType>();
      if (vt && vt.isUnit())
        s << "unit";
      else
        s << getTypeSignature(f.second);
    }
    s << "End";
    signature = s.str();
  }

  SmallVector<RustEnumType::EnumVariantTy, 4> enumVariants;
  unsigned id;

  using KeyTy = ArrayRef<RustEnumType::EnumVariantTy>;

  bool operator==(const KeyTy &key) const { return key == KeyTy(enumVariants); }

  static llvm::hash_code hashKey(const KeyTy &key) {
    return llvm::hash_combine(key);
  }

  static RustEnumTypeStorage *construct(TypeStorageAllocator &allocator,
                                        const KeyTy &key) {
    return new (allocator.allocate<RustEnumTypeStorage>())
        RustEnumTypeStorage(key, idCounter++);
  }

  RustPrinterStream &printAsRust(RustPrinterStream &os) const;
  raw_ostream &printAsRustNamedType(raw_ostream &os) const;
  void print(DialectAsmPrinter &os) const { os << getRustType(); }
  StringRef getVariantName(unsigned idx) const;
  Type getVariantType(unsigned idx) const;
  unsigned getNumVariants() const;

  std::string getRustType() const;
  unsigned getEnumTypeId() const;

  void emitNestedTypedefs(rust::RustPrinterStream &os) const;
  std::string getSignature() const;

private:
  static unsigned idCounter;
  std::string signature;
};

unsigned RustEnumTypeStorage::idCounter = 0;

RustEnumType RustEnumType::get(RustDialect *dialect,
                               ArrayRef<EnumVariantTy> fields) {
  return Base::get(dialect->getContext(), fields);
}

void RustEnumType::print(DialectAsmPrinter &os) const { getImpl()->print(os); }

RustPrinterStream &RustEnumType::printAsRust(RustPrinterStream &os) const {
  return getImpl()->printAsRust(os);
}

raw_ostream &RustEnumType::printAsRustNamedType(raw_ostream &os) const {
  return getImpl()->printAsRustNamedType(os);
}

StringRef RustEnumType::getVariantName(unsigned idx) const {
  return getImpl()->getVariantName(idx);
}

StringRef RustEnumTypeStorage::getVariantName(unsigned idx) const {
  return enumVariants[idx].first.getValue();
}

Type RustEnumType::getVariantType(unsigned idx) const {
  return getImpl()->getVariantType(idx);
}

Type RustEnumTypeStorage::getVariantType(unsigned idx) const {
  return enumVariants[idx].second;
}

unsigned RustEnumType::getNumVariants() const {
  return getImpl()->getNumVariants();
}

unsigned RustEnumTypeStorage::getNumVariants() const {
  return enumVariants.size();
}
std::string RustEnumType::getRustType() const {
  return getImpl()->getRustType();
}

unsigned RustEnumTypeStorage::getEnumTypeId() const { return id; }

unsigned RustEnumType::getEnumTypeId() const {
  return getImpl()->getEnumTypeId();
}
std::string RustEnumTypeStorage::getRustType() const { return signature; }

RustPrinterStream &
RustEnumTypeStorage::printAsRust(RustPrinterStream &ps) const {

  llvm::raw_ostream &os = ps.getNamedTypesStream();
  // First ensure that any structs used by this struct are defined
  emitNestedTypedefs(ps);

  os << "#[rewrite]\n";

  os << "pub enum ";
  printAsRustNamedType(os) << " {\n";

  for (unsigned i = 0; i < enumVariants.size(); i++) {
    os << "  " << enumVariants[i].first.getValue() << "("
       << getTypeString(enumVariants[i].second) << "),\n";
  }
  os << "\n}\n";
  return ps;
}

void RustEnumType::emitNestedTypedefs(rust::RustPrinterStream &ps) const {
  return getImpl()->emitNestedTypedefs(ps);
}

void RustEnumTypeStorage::emitNestedTypedefs(
    rust::RustPrinterStream &ps) const {
  // First ensure that any structs used by this tuple are defined
  for (unsigned i = 0; i < enumVariants.size(); i++)
    if (enumVariants[i].second.isa<RustEnumType>())
      ps.writeEnumDefiniton(enumVariants[i].second.cast<RustEnumType>());
    else if (enumVariants[i].second.isa<RustTupleType>())
      enumVariants[i].second.cast<RustTupleType>().emitNestedTypedefs(ps);
}

raw_ostream &RustEnumTypeStorage::printAsRustNamedType(raw_ostream &os) const {

  os << signature;
  return os;
}

std::string RustEnumType::getSignature() const {
  return getImpl()->getSignature();
}

std::string RustEnumTypeStorage::getSignature() const { return signature; }

//===----------------------------------------------------------------------===//
// RustStreamType
//===----------------------------------------------------------------------===//

struct RustStreamTypeStorage : public TypeStorage {
  RustStreamTypeStorage(Type item, unsigned id) : item(item), id(id) {
    std::string str;
    llvm::raw_string_ostream s(str);
    s << "Stream";
    s << getTypeSignature(item);
    s << "End";
    signature = s.str();
  }

  Type item;
  unsigned id;

  using KeyTy = Type;

  bool operator==(const KeyTy &key) const { return key == KeyTy(item); }

  static llvm::hash_code hashKey(const KeyTy &key) {
    return llvm::hash_combine(key);
  }

  static RustStreamTypeStorage *construct(TypeStorageAllocator &allocator,
                                          const KeyTy &key) {
    return new (allocator.allocate<RustStreamTypeStorage>())
        RustStreamTypeStorage(key, idCounter++);
  }

  RustPrinterStream &printAsRust(RustPrinterStream &os) const;
  raw_ostream &printAsRustNamedType(raw_ostream &os) const;
  void print(DialectAsmPrinter &os) const { os << getRustType(); }

  std::string getRustType() const;
  Type getType() const;
  unsigned getStreamTypeId() const;

  void emitNestedTypedefs(rust::RustPrinterStream &os) const;
  std::string getSignature() const;

private:
  static unsigned idCounter;
  std::string signature;
};

unsigned RustStreamTypeStorage::idCounter = 0;

RustStreamType RustStreamType::get(RustDialect *dialect, Type item) {
  return Base::get(dialect->getContext(), item);
}

void RustStreamType::print(DialectAsmPrinter &os) const {
  getImpl()->print(os);
}

RustPrinterStream &RustStreamType::printAsRust(RustPrinterStream &os) const {
  return getImpl()->printAsRust(os);
}

std::string RustStreamType::getRustType() const {
  return getImpl()->getRustType();
}

unsigned RustStreamTypeStorage::getStreamTypeId() const { return id; }

std::string RustStreamTypeStorage::getRustType() const { return signature; }

RustPrinterStream &
RustStreamTypeStorage::printAsRust(RustPrinterStream &ps) const {

  llvm::raw_ostream &os = ps.getNamedTypesStream();
  // First ensure that any structs used by this struct are defined
  emitNestedTypedefs(ps);

  os << "<a stream should not be output as rust>\n";
  return ps;
}

void RustStreamType::emitNestedTypedefs(rust::RustPrinterStream &ps) const {
  return getImpl()->emitNestedTypedefs(ps);
}

void RustStreamTypeStorage::emitNestedTypedefs(
    rust::RustPrinterStream &ps) const {
  // XXXX?
}

raw_ostream &
RustStreamTypeStorage::printAsRustNamedType(raw_ostream &os) const {

  os << signature;
  return os;
}

std::string RustStreamType::getSignature() const {
  return getImpl()->getSignature();
}

std::string RustStreamTypeStorage::getSignature() const { return signature; }

Type RustStreamType::getType() const { return getImpl()->getType(); }

Type RustStreamTypeStorage::getType() const { return item; }

//===----------------------------------------------------------------------===//
// RustSinkStreamType
//===----------------------------------------------------------------------===//
struct RustSinkStreamTypeStorage : public TypeStorage {
  RustSinkStreamTypeStorage(Type item, unsigned id) : item(item), id(id) {
    std::string str;
    llvm::raw_string_ostream s(str);
    s << "PushableStream";
    s << getTypeSignature(item);
    s << "End";
    signature = s.str();
  }

  Type item;
  unsigned id;

  using KeyTy = Type;

  bool operator==(const KeyTy &key) const { return key == KeyTy(item); }

  static llvm::hash_code hashKey(const KeyTy &key) {
    return llvm::hash_combine(key);
  }

  static RustSinkStreamTypeStorage *construct(TypeStorageAllocator &allocator,
                                              const KeyTy &key) {
    return new (allocator.allocate<RustSinkStreamTypeStorage>())
        RustSinkStreamTypeStorage(key, idCounter++);
  }

  RustPrinterStream &printAsRust(RustPrinterStream &os) const;
  raw_ostream &printAsRustNamedType(raw_ostream &os) const;
  void print(DialectAsmPrinter &os) const { os << getRustType(); }

  std::string getRustType() const;
  Type getType() const;
  unsigned getStreamTypeId() const;

  void emitNestedTypedefs(rust::RustPrinterStream &os) const;
  std::string getSignature() const;

private:
  static unsigned idCounter;
  std::string signature;
};
unsigned RustSinkStreamTypeStorage::idCounter = 0;

RustSinkStreamType RustSinkStreamType::get(RustDialect *dialect, Type item) {
  return Base::get(dialect->getContext(), item);
}

void RustSinkStreamType::print(DialectAsmPrinter &os) const {
  getImpl()->print(os);
}

RustPrinterStream &
RustSinkStreamType::printAsRust(RustPrinterStream &os) const {
  return getImpl()->printAsRust(os);
}

std::string RustSinkStreamType::getRustType() const {
  return getImpl()->getRustType();
}

unsigned RustSinkStreamTypeStorage::getStreamTypeId() const { return id; }

std::string RustSinkStreamTypeStorage::getRustType() const { return signature; }

RustPrinterStream &
RustSinkStreamTypeStorage::printAsRust(RustPrinterStream &ps) const {

  llvm::raw_ostream &os = ps.getNamedTypesStream();
  // First ensure that any structs used by this struct are defined
  emitNestedTypedefs(ps);

  os << "<a stream should not be output as rust>\n";
  return ps;
}

void RustSinkStreamType::emitNestedTypedefs(rust::RustPrinterStream &ps) const {
  return getImpl()->emitNestedTypedefs(ps);
}

void RustSinkStreamTypeStorage::emitNestedTypedefs(
    rust::RustPrinterStream &ps) const {
  // XXXX?
}

raw_ostream &
RustSinkStreamTypeStorage::printAsRustNamedType(raw_ostream &os) const {

  os << signature;
  return os;
}

std::string RustSinkStreamType::getSignature() const {
  return getImpl()->getSignature();
}

std::string RustSinkStreamTypeStorage::getSignature() const {
  return signature;
}

Type RustSinkStreamType::getType() const { return getImpl()->getType(); }

Type RustSinkStreamTypeStorage::getType() const { return item; }

//===----------------------------------------------------------------------===//
// RustSourceStreamType
//===----------------------------------------------------------------------===//
struct RustSourceStreamTypeStorage : public TypeStorage {
  RustSourceStreamTypeStorage(Type item, unsigned id) : item(item), id(id) {
    std::string str;
    llvm::raw_string_ostream s(str);
    s << "Pullable";
    s << getTypeSignature(item);
    s << "End";
    signature = s.str();
  }

  Type item;
  unsigned id;

  using KeyTy = Type;

  bool operator==(const KeyTy &key) const { return key == KeyTy(item); }

  static llvm::hash_code hashKey(const KeyTy &key) {
    return llvm::hash_combine(key);
  }

  static RustSourceStreamTypeStorage *construct(TypeStorageAllocator &allocator,
                                                const KeyTy &key) {
    return new (allocator.allocate<RustSourceStreamTypeStorage>())
        RustSourceStreamTypeStorage(key, idCounter++);
  }

  RustPrinterStream &printAsRust(RustPrinterStream &os) const;
  raw_ostream &printAsRustNamedType(raw_ostream &os) const;
  void print(DialectAsmPrinter &os) const { os << getRustType(); }

  std::string getRustType() const;
  Type getType() const;
  unsigned getStreamTypeId() const;

  void emitNestedTypedefs(rust::RustPrinterStream &os) const;
  std::string getSignature() const;

private:
  static unsigned idCounter;
  std::string signature;
};

unsigned RustSourceStreamTypeStorage::idCounter = 0;

RustSourceStreamType RustSourceStreamType::get(RustDialect *dialect,
                                               Type item) {
  return Base::get(dialect->getContext(), item);
}

void RustSourceStreamType::print(DialectAsmPrinter &os) const {
  getImpl()->print(os);
}

RustPrinterStream &
RustSourceStreamType::printAsRust(RustPrinterStream &os) const {
  return getImpl()->printAsRust(os);
}

std::string RustSourceStreamType::getRustType() const {
  return getImpl()->getRustType();
}

unsigned RustSourceStreamTypeStorage::getStreamTypeId() const { return id; }

std::string RustSourceStreamTypeStorage::getRustType() const {
  return signature;
}

RustPrinterStream &
RustSourceStreamTypeStorage::printAsRust(RustPrinterStream &ps) const {

  llvm::raw_ostream &os = ps.getNamedTypesStream();
  // First ensure that any structs used by this struct are defined
  emitNestedTypedefs(ps);

  os << "<a stream should not be output as rust>\n";
  return ps;
}

void RustSourceStreamType::emitNestedTypedefs(
    rust::RustPrinterStream &ps) const {
  return getImpl()->emitNestedTypedefs(ps);
}

void RustSourceStreamTypeStorage::emitNestedTypedefs(
    rust::RustPrinterStream &ps) const {
  // XXXX?
}

raw_ostream &
RustSourceStreamTypeStorage::printAsRustNamedType(raw_ostream &os) const {

  os << signature;
  return os;
}

std::string RustSourceStreamType::getSignature() const {
  return getImpl()->getSignature();
}

std::string RustSourceStreamTypeStorage::getSignature() const {
  return signature;
}

Type RustSourceStreamType::getType() const { return getImpl()->getType(); }

Type RustSourceStreamTypeStorage::getType() const { return item; }

//===----------------------------------------------------------------------===//
// RustStructType
//===----------------------------------------------------------------------===//

struct RustStructTypeStorage : public TypeStorage {
  RustStructTypeStorage(ArrayRef<RustStructType::StructFieldTy> fields,
                        unsigned id)
      : structFields(fields.begin(), fields.end()), id(id) {
    std::string str;
    llvm::raw_string_ostream s(str);
    s << "Struct";

    for (auto &f : fields) {
      StringRef fieldName = f.first.getValue();
      s << fieldName.size() << fieldName;
      s << getTypeSignature(f.second);
    }
    s << "End";
    signature = s.str();
  }

  SmallVector<RustStructType::StructFieldTy, 4> structFields;
  unsigned id;

  using KeyTy = ArrayRef<RustStructType::StructFieldTy>;

  bool operator==(const KeyTy &key) const { return key == KeyTy(structFields); }

  static llvm::hash_code hashKey(const KeyTy &key) {
    return llvm::hash_combine(key);
  }

  static RustStructTypeStorage *construct(TypeStorageAllocator &allocator,
                                          const KeyTy &key) {
    return new (allocator.allocate<RustStructTypeStorage>())
        RustStructTypeStorage(key, idCounter++);
  }

  RustPrinterStream &printAsRust(RustPrinterStream &os) const;
  raw_ostream &printAsRustNamedType(raw_ostream &os) const;
  void print(DialectAsmPrinter &os) const { os << getRustType(); }

  unsigned getNumFields() const;
  StringRef getFieldName(unsigned idx) const;
  Type getFieldType(unsigned idx) const;

  std::string getRustType() const;
  unsigned getStructTypeId() const;

  void emitNestedTypedefs(rust::RustPrinterStream &os) const;
  std::string getSignature() const;

private:
  static unsigned idCounter;
  std::string signature;
};

unsigned RustStructTypeStorage::idCounter = 0;

RustStructType RustStructType::get(RustDialect *dialect,
                                   ArrayRef<StructFieldTy> fields) {
  return Base::get(dialect->getContext(), fields);
}

void RustStructType::print(DialectAsmPrinter &os) const {
  getImpl()->print(os);
}

RustPrinterStream &RustStructType::printAsRust(RustPrinterStream &os) const {
  return getImpl()->printAsRust(os);
}

raw_ostream &RustStructType::printAsRustNamedType(raw_ostream &os) const {
  return getImpl()->printAsRustNamedType(os);
}

unsigned RustStructType::getNumFields() const {
  return getImpl()->getNumFields();
}

unsigned RustStructTypeStorage::getNumFields() const {
  return structFields.size();
}

StringRef RustStructType::getFieldName(unsigned idx) const {
  return getImpl()->getFieldName(idx);
}

StringRef RustStructTypeStorage::getFieldName(unsigned idx) const {
  return structFields[idx].first.getValue();
}

Type RustStructType::getFieldType(unsigned idx) const {
  return getImpl()->getFieldType(idx);
}

Type RustStructTypeStorage::getFieldType(unsigned idx) const {
  return structFields[idx].second;
}

std::string RustStructType::getRustType() const {
  return getImpl()->getRustType();
}

unsigned RustStructTypeStorage::getStructTypeId() const { return id; }

unsigned RustStructType::getStructTypeId() const {
  return getImpl()->getStructTypeId();
}
std::string RustStructTypeStorage::getRustType() const { return signature; }

RustPrinterStream &
RustStructTypeStorage::printAsRust(RustPrinterStream &ps) const {

  llvm::raw_ostream &os = ps.getNamedTypesStream();
  // First ensure that any structs used by this struct are defined
  emitNestedTypedefs(ps);

  os << "#[rewrite]\n";

  os << "pub struct ";
  printAsRustNamedType(os) << " {\n  ";

  for (unsigned i = 0; i < structFields.size(); i++) {
    if (i != 0)
      os << ",\n  ";
    os << "  " << structFields[i].first.getValue() << " : ";
    ps.printAsRust(os, structFields[i].second);
  }
  os << "\n}\n";
  return ps;
}

void RustStructType::emitNestedTypedefs(rust::RustPrinterStream &ps) const {
  return getImpl()->emitNestedTypedefs(ps);
}

void RustStructTypeStorage::emitNestedTypedefs(
    rust::RustPrinterStream &ps) const {
  // First ensure that any structs used by this tuple are defined
  for (unsigned i = 0; i < structFields.size(); i++)
    if (structFields[i].second.isa<RustStructType>())
      ps.writeStructDefiniton(structFields[i].second.cast<RustStructType>());
    else if (structFields[i].second.isa<RustTupleType>())
      structFields[i].second.cast<RustTupleType>().emitNestedTypedefs(ps);
}

raw_ostream &
RustStructTypeStorage::printAsRustNamedType(raw_ostream &os) const {

  os << signature;
  return os;
}

std::string RustStructType::getSignature() const {
  return getImpl()->getSignature();
}

std::string RustStructTypeStorage::getSignature() const { return signature; }

//===----------------------------------------------------------------------===//
// RustTensorType
//===----------------------------------------------------------------------===//

struct RustTensorTypeStorage : public TypeStorage {
  using KeyTy = std::pair<Type, ArrayRef<int64_t>>;

  RustTensorTypeStorage(KeyTy key)
      : elementTy(key.first), dimensions(key.second.begin(), key.second.end()) {
  }

  Type elementTy;
  SmallVector<int64_t, 3> dimensions;

  bool operator==(const KeyTy &key) const {
    return key == KeyTy(elementTy, dimensions);
  }

  static llvm::hash_code hashKey(const KeyTy &key) {
    return llvm::hash_combine(key);
  }

  static RustTensorTypeStorage *construct(TypeStorageAllocator &allocator,
                                          const KeyTy &key) {
    return new (allocator.allocate<RustTensorTypeStorage>())
        RustTensorTypeStorage(key);
  }

  RustPrinterStream &printAsRust(RustPrinterStream &os) const;
  void print(DialectAsmPrinter &os) const { os << getRustType(); }

  std::string getRustType() const;

  ArrayRef<int64_t> getDimensions() const { return dimensions; }
  std::string getSignature() const;
};

RustTensorType RustTensorType::get(RustDialect *dialect, Type elementTy,
                                   ArrayRef<int64_t> dimensions) {
  return Base::get(dialect->getContext(), elementTy, dimensions);
}

void RustTensorType::print(DialectAsmPrinter &os) const {
  getImpl()->print(os);
}

RustPrinterStream &RustTensorType::printAsRust(RustPrinterStream &os) const {
  return getImpl()->printAsRust(os);
}

std::string RustTensorType::getRustType() const {
  return getImpl()->getRustType();
}

std::string RustTensorTypeStorage::getRustType() const {
  std::string str;
  llvm::raw_string_ostream s(str);

  s << "Rc<Array<" << getTypeString(elementTy) << ", Dim<[Ix; "
    << dimensions.size() << "]>>>";
  return s.str();
}

RustPrinterStream &
RustTensorTypeStorage::printAsRust(RustPrinterStream &ps) const {
  ps.registerDirective("rc-import", "use std::rc::Rc;\n");
  ps.registerDirective("ndarray-import", "use ndarray::{Array,Dim,Ix};\n");
  ps.registerDependency("ndarray",
                        (Twine("\"") + CrateVersions::ndarray + "\"").str());
  ps << getRustType();
  return ps;
}

ArrayRef<int64_t> RustTensorType::getDimensions() const {
  return getImpl()->getDimensions();
}

std::string RustTensorType::getSignature() const {
  return getImpl()->getSignature();
}

std::string RustTensorTypeStorage::getSignature() const {
  std::string str;
  llvm::raw_string_ostream s(str);

  s << "TensorT" << getTypeSignature(elementTy) << "x" << dimensions.size();
  return s.str();
}

//===----------------------------------------------------------------------===//
// RustTupleType
//===----------------------------------------------------------------------===//

struct RustTupleTypeStorage : public TypeStorage {
  RustTupleTypeStorage(ArrayRef<Type> fields)
      : tupleFields(fields.begin(), fields.end()) {}

  SmallVector<Type, 4> tupleFields;

  using KeyTy = ArrayRef<Type>;

  bool operator==(const KeyTy &key) const { return key == KeyTy(tupleFields); }

  static llvm::hash_code hashKey(const KeyTy &key) {
    return llvm::hash_combine(key);
  }

  static RustTupleTypeStorage *construct(TypeStorageAllocator &allocator,
                                         const KeyTy &key) {
    return new (allocator.allocate<RustTupleTypeStorage>())
        RustTupleTypeStorage(key);
  }

  RustPrinterStream &printAsRust(RustPrinterStream &os) const;
  void print(DialectAsmPrinter &os) const { os << getRustType(); }

  std::string getRustType() const;

  void emitNestedTypedefs(rust::RustPrinterStream &ps) const;

  std::string getSignature() const;
};

RustTupleType RustTupleType::get(RustDialect *dialect, ArrayRef<Type> fields) {
  return Base::get(dialect->getContext(), fields);
}

void RustTupleType::print(DialectAsmPrinter &os) const { getImpl()->print(os); }

RustPrinterStream &RustTupleType::printAsRust(RustPrinterStream &os) const {
  return getImpl()->printAsRust(os);
}

std::string RustTupleType::getRustType() const {
  return getImpl()->getRustType();
}

std::string RustTupleTypeStorage::getRustType() const {
  std::string str;
  llvm::raw_string_ostream s(str);

  s << "(";
  for (unsigned i = 0; i < tupleFields.size(); i++)
    s << getTypeString(tupleFields[i]) << ", ";
  s << ")";
  return s.str();
}

RustPrinterStream &
RustTupleTypeStorage::printAsRust(RustPrinterStream &ps) const {
  emitNestedTypedefs(ps);

  ps.registerDirective("rc-import", "use std::rc::Rc;\n");
  ps << getRustType();
  return ps;
}

void RustTupleType::emitNestedTypedefs(rust::RustPrinterStream &ps) const {
  return getImpl()->emitNestedTypedefs(ps);
}

void RustTupleTypeStorage::emitNestedTypedefs(
    rust::RustPrinterStream &ps) const {
  // First ensure that any structs used by this tuple are defined
  for (unsigned i = 0; i < tupleFields.size(); i++)
    if (tupleFields[i].isa<RustStructType>())
      ps.writeStructDefiniton(tupleFields[i].cast<RustStructType>());
    else if (tupleFields[i].isa<RustTupleType>())
      tupleFields[i].cast<RustTupleType>().emitNestedTypedefs(ps);
}

std::string RustTupleType::getSignature() const {
  return getImpl()->getSignature();
}

std::string RustTupleTypeStorage::getSignature() const {
  std::string str;
  llvm::raw_string_ostream s(str);

  s << "Tuple";
  for (unsigned i = 0; i < tupleFields.size(); i++)
    s << "T" << getTypeSignature(tupleFields[i]);
  return s.str();
}

} // namespace types

static bool isAnyRustType(Type type) {
  if (type.isa<RustType>() || type.isa<RustStructType>() ||
      type.isa<RustTupleType>() || type.isa<RustTensorType>() ||
      type.isa<RustEnumType>() || type.isa<RustStreamType>() ||
      type.isa<RustSinkStreamType>() || type.isa<RustSourceStreamType>())
    return true;
  if (type.isa<FunctionType>())
    return isRustFunctionType(type);
  return false;
}

bool isRustFunctionType(Type type) {
  if (FunctionType fty = type.dyn_cast<FunctionType>()) {
    for (Type t : fty.getInputs())
      if (!isAnyRustType(t))
        return false;
    for (Type t : fty.getResults())
      if (!isAnyRustType(t))
        return false;
    return true;
  }
  return false;
}
} // namespace rust

//===----------------------------------------------------------------------===//
// TableGen'd op method definitions
//===----------------------------------------------------------------------===//

#define GET_OP_CLASSES
#include "Rust/Rust.cpp.inc"
