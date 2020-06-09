//===- Dialect definition for the Arc IR ----------------------------------===//
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
// Defines the types of the Arc dialect.
//
//===----------------------------------------------------------------------===//

#ifndef ARC_TYPES_H_
#define ARC_TYPES_H_

#include <mlir/IR/Dialect.h>
#include <mlir/IR/StandardTypes.h>

using namespace mlir;

namespace arc {
namespace types {

//===----------------------------------------------------------------------===//
// Arc Type Kinds
//===----------------------------------------------------------------------===//

enum Kind { Appender = Type::Kind::FIRST_PRIVATE_EXPERIMENTAL_0_TYPE, Struct };

//===----------------------------------------------------------------------===//
// Arc Type Functions
//===----------------------------------------------------------------------===//

bool isValueType(Type type);
bool isBuilderType(Type type);

//===----------------------------------------------------------------------===//
// Arc Type Storages
//===----------------------------------------------------------------------===//

struct BuilderTypeStorage;
struct AppenderTypeStorage;
struct StructTypeStorage;

//===----------------------------------------------------------------------===//
// Arc Types
//===----------------------------------------------------------------------===//

class BuilderType : public Type {
public:
  using ImplType = BuilderTypeStorage;
  using Type::Type;

  Type getMergeType() const;
  Type getResultType() const;
};

class AppenderType
    : public Type::TypeBase<AppenderType, BuilderType, AppenderTypeStorage> {
public:
  using Base::Base;

  static bool kindof(unsigned kind) { return kind == Appender; }
  static AppenderType get(Type mergeType, RankedTensorType resultType);
  static AppenderType getChecked(Type mergeType, RankedTensorType resultType,
                                 Location loc);
  static LogicalResult
  verifyConstructionInvariants(Location loc, Type mergeType,
                               RankedTensorType resultType);
  static Type parse(DialectAsmParser &parser);
  void print(DialectAsmPrinter &os) const;
};

class StructType
    : public mlir::Type::TypeBase<StructType, mlir::Type, StructTypeStorage> {
public:
  using Base::Base;

  typedef std::pair<mlir::StringAttr, mlir::Type> FieldTy;

  static bool kindof(unsigned kind) { return kind == arc::types::Struct; }

  static StructType get(llvm::ArrayRef<FieldTy> elementTypes);

  /// Returns the fields of this struct type.
  llvm::ArrayRef<FieldTy> getFields() const;

  /// Returns the number of fields held by this struct.
  size_t getNumFields() const;

  static Type parse(DialectAsmParser &parser);
  void print(DialectAsmPrinter &os) const;
};
} // namespace types
} // namespace arc

#endif // ARC_TYPES_H_
