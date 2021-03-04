//===- Arc IR Dialect registration in MLIR --------------------------------===//
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

#include "Arc/Types.h"
#include <llvm/Support/raw_ostream.h>
#include <mlir/IR/Builders.h>
#include <mlir/IR/BuiltinTypes.h>
#include <mlir/IR/DialectImplementation.h>

using namespace mlir;
using namespace arc;
using namespace types;

namespace arc {
namespace types {

//===----------------------------------------------------------------------===//
// Shared Functions
//===----------------------------------------------------------------------===//

bool isValueType(Type type) {
  if (type.isa<BFloat16Type>() || type.isa<Float16Type>() ||
      type.isa<Float32Type>() || type.isa<Float64Type>() ||
      type.isa<IntegerType>() || type.isa<VectorType>() ||
      type.isa<RankedTensorType>() || type.isa<UnrankedTensorType>() ||
      type.isa<UnrankedMemRefType>() || type.isa<MemRefType>() ||
      type.isa<ComplexType>() || type.isa<ComplexType>() ||
      type.isa<NoneType>())
    return true;
  if (type.isa<TupleType>()) {
    for (auto t : type.cast<TupleType>().getTypes())
      if (!isValueType(t))
        return false;
    return true;
  }
  return false;
}

//===----------------------------------------------------------------------===//
// ArconType
//===----------------------------------------------------------------------===//

struct ArconTypeStorage : public TypeStorage {
  ArconTypeStorage(Type containedTy, std::string keyword)
      : TypeStorage(), containedType(containedTy), keyword(keyword) {}

  using KeyTy = Type;

  bool operator==(const KeyTy &key) const { return key == containedType; }

  Type containedType;
  std::string keyword;
};

Type ArconType::getContainedType() const {
  return static_cast<ImplType *>(impl)->containedType;
}

StringRef ArconType::getKeyword() const {
  return static_cast<ImplType *>(impl)->keyword;
}

bool isBuilderType(Type type) { return type.isa<AppenderType>(); }

void ArconType::print(DialectAsmPrinter &os) const {
  os << getKeyword() << "<" << getContainedType() << ">";
}

//===----------------------------------------------------------------------===//
// ArconAppenderType
//===----------------------------------------------------------------------===//
struct ArconAppenderTypeStorage : public ArconTypeStorage {
  using KeyTy = Type;

  ArconAppenderTypeStorage(Type elementType)
      : ArconTypeStorage(elementType, "arcon.appender") {}

  static ArconAppenderTypeStorage *
  construct(mlir::TypeStorageAllocator &allocator, const KeyTy &key) {
    return new (allocator.allocate<ArconValueTypeStorage>())
        ArconAppenderTypeStorage(key);
  }
};

ArconAppenderType ArconAppenderType::get(mlir::Type elementType) {
  mlir::MLIRContext *ctx = elementType.getContext();
  return Base::get(ctx, elementType);
}

/// Returns the element type of this stream type.
mlir::Type ArconAppenderType::getType() const { return getContainedType(); }

Type ArconAppenderType::parse(DialectAsmParser &parser) {
  if (parser.parseLess())
    return nullptr;

  mlir::Type elementType;
  if (parser.parseType(elementType))
    return nullptr;

  if (parser.parseGreater())
    return Type();
  return ArconAppenderType::get(elementType);
}

//===----------------------------------------------------------------------===//
// ArconMapType
//===----------------------------------------------------------------------===//
struct ArconMapTypeStorage : public ArconTypeStorage {
  using KeyTy = std::pair<Type, Type>;

  ArconMapTypeStorage(Type keyType, Type valueType)
      : ArconTypeStorage(keyType, "arcon.map"), valueType(valueType) {}

  static llvm::hash_code hashKey(const KeyTy &key) {
    return llvm::hash_combine(key.first, key.second);
  }

  static KeyTy getKey(Type keyType, Type valueType) {
    return KeyTy(keyType, valueType);
  }

  bool operator==(const KeyTy &key) const {
    return key.first == containedType && key.second == valueType;
  }

  static ArconMapTypeStorage *construct(mlir::TypeStorageAllocator &allocator,
                                        const KeyTy &key) {
    return new (allocator.allocate<ArconMapTypeStorage>())
        ArconMapTypeStorage(key.first, key.second);
  }

  Type valueType;
};

ArconMapType ArconMapType::get(mlir::Type keyType, mlir::Type valueType) {
  mlir::MLIRContext *ctx = keyType.getContext();
  return Base::get(ctx, keyType, valueType);
}

mlir::Type ArconMapType::getKeyType() const { return getContainedType(); }

mlir::Type ArconMapType::getValueType() const {

  return static_cast<ImplType *>(impl)->valueType;
}

Type ArconMapType::parse(DialectAsmParser &parser) {
  if (parser.parseLess())
    return nullptr;

  mlir::Type keyType;
  if (parser.parseType(keyType))
    return nullptr;

  if (parser.parseComma())
    return nullptr;

  mlir::Type valueType;
  if (parser.parseType(valueType))
    return nullptr;

  if (parser.parseGreater())
    return Type();
  return ArconMapType::get(keyType, valueType);
}

void ArconMapType::print(DialectAsmPrinter &os) const {
  os << getKeyword() << "<" << getKeyType() << ", " << getValueType() << ">";
}

//===----------------------------------------------------------------------===//
// ArconValueType
//===----------------------------------------------------------------------===//
struct ArconValueTypeStorage : public ArconTypeStorage {
  using KeyTy = Type;

  ArconValueTypeStorage(Type elementType)
      : ArconTypeStorage(elementType, "arcon.value") {}

  static ArconValueTypeStorage *construct(mlir::TypeStorageAllocator &allocator,
                                          const KeyTy &key) {
    return new (allocator.allocate<ArconValueTypeStorage>())
        ArconValueTypeStorage(key);
  }
};

ArconValueType ArconValueType::get(mlir::Type elementType) {
  mlir::MLIRContext *ctx = elementType.getContext();
  return Base::get(ctx, elementType);
}

/// Returns the element type of this stream type.
mlir::Type ArconValueType::getType() const { return getContainedType(); }

Type ArconValueType::parse(DialectAsmParser &parser) {
  if (parser.parseLess())
    return nullptr;

  mlir::Type elementType;
  if (parser.parseType(elementType))
    return nullptr;

  if (parser.parseGreater())
    return Type();
  return ArconValueType::get(elementType);
}

//===----------------------------------------------------------------------===//
// BuilderType
//===----------------------------------------------------------------------===//

struct BuilderTypeStorage : public TypeStorage {
  BuilderTypeStorage(Type mergeTy, Type resultTy)
      : TypeStorage(), mergeType(mergeTy), resultType(resultTy) {}

  using KeyTy = std::pair<Type, Type>;

  bool operator==(const KeyTy &key) const {
    return key.first == mergeType && key.second == resultType;
  }

  Type mergeType;
  Type resultType;
};

Type BuilderType::getMergeType() const {
  return static_cast<ImplType *>(impl)->mergeType;
}

Type BuilderType::getResultType() const {
  return static_cast<ImplType *>(impl)->resultType;
}

//===----------------------------------------------------------------------===//
// AppenderType
//===----------------------------------------------------------------------===//

struct AppenderTypeStorage : public BuilderTypeStorage {
  AppenderTypeStorage(Type mergeTy, RankedTensorType resultTy)
      : BuilderTypeStorage(mergeTy, resultTy) {}

  using KeyTy = std::pair<Type, RankedTensorType>;

  static llvm::hash_code hashKey(const KeyTy &key) {
    return llvm::hash_combine(key.first, key.second);
  }

  static KeyTy getKey(Type mergeType, RankedTensorType resultType) {
    return KeyTy(mergeType, resultType);
  }

  static AppenderTypeStorage *construct(TypeStorageAllocator &allocator,
                                        const KeyTy &key) {
    return new (allocator.allocate<AppenderTypeStorage>())
        AppenderTypeStorage(key.first, key.second);
  }
};

AppenderType AppenderType::get(Type mergeType, RankedTensorType resultType) {
  return Base::get(mergeType.getContext(), mergeType, resultType);
}

AppenderType
AppenderType::getChecked(function_ref<InFlightDiagnostic()> emitError,
                         Type mergeType, RankedTensorType resultType,
                         Location loc) {
  return Base::getChecked(emitError, mergeType.getContext(), mergeType,
                          resultType);
}

Type AppenderType::parse(DialectAsmParser &parser) {
  if (parser.parseLess())
    return nullptr;
  Location loc = parser.getEncodedSourceLoc(parser.getCurrentLocation());
  mlir::Type mergeType;
  if (parser.parseType(mergeType))
    return nullptr;
  if (parser.parseGreater())
    return nullptr;

  ArrayRef<int64_t> shape = {};
  RankedTensorType resultType = RankedTensorType::getChecked(
      mlir::detail::getDefaultDiagnosticEmitFn(loc), shape, mergeType);

  return AppenderType::getChecked(mlir::detail::getDefaultDiagnosticEmitFn(loc),
                                  mergeType, resultType, loc);
}

void AppenderType::print(DialectAsmPrinter &os) const {
  os << "appender" << '<' << getMergeType() << '>';
}

LogicalResult AppenderType::verify(function_ref<InFlightDiagnostic()> emitError,
                                   Type mergeType,
                                   RankedTensorType resultType) {
  if (!isValueType(mergeType)) {
    return emitError() << "appender merge type must be a value type: found "
                       << mergeType;
  }
  return success();
}

//===----------------------------------------------------------------------===//
// StreamType
//===----------------------------------------------------------------------===//
struct StreamTypeStorage : public ArconTypeStorage {
  using KeyTy = Type;

  StreamTypeStorage(Type elementType)
      : ArconTypeStorage(elementType, "stream") {}

  static StreamTypeStorage *construct(mlir::TypeStorageAllocator &allocator,
                                      const KeyTy &key) {
    return new (allocator.allocate<StreamTypeStorage>()) StreamTypeStorage(key);
  }
};

StreamType StreamType::get(mlir::Type elementType) {
  mlir::MLIRContext *ctx = elementType.getContext();
  return Base::get(ctx, elementType);
}

/// Returns the element type of this stream type.
mlir::Type StreamType::getType() const { return getContainedType(); }

Type StreamType::parse(DialectAsmParser &parser) {
  if (parser.parseLess())
    return nullptr;

  mlir::Type elementType;
  if (parser.parseType(elementType))
    return nullptr;

  if (parser.parseGreater())
    return Type();
  return StreamType::get(elementType);
}

//===----------------------------------------------------------------------===//
// StructType
//===----------------------------------------------------------------------===//
struct StructTypeStorage : public mlir::TypeStorage {
  using KeyTy = llvm::ArrayRef<StructType::FieldTy>;

  StructTypeStorage(llvm::ArrayRef<StructType::FieldTy> elementTypes)
      : fields(elementTypes) {}

  bool operator==(const KeyTy &key) const { return key == fields; }

  static llvm::hash_code hashKey(const KeyTy &key) {
    return llvm::hash_value(key);
  }

  static KeyTy getKey(llvm::ArrayRef<StructType::FieldTy> elementTypes) {
    return KeyTy(elementTypes);
  }

  static StructTypeStorage *construct(mlir::TypeStorageAllocator &allocator,
                                      const KeyTy &key) {
    llvm::ArrayRef<StructType::FieldTy> elementTypes = allocator.copyInto(key);

    return new (allocator.allocate<StructTypeStorage>())
        StructTypeStorage(elementTypes);
  }

  llvm::ArrayRef<StructType::FieldTy> fields;
};

StructType StructType::get(llvm::ArrayRef<StructType::FieldTy> elementTypes) {
  assert(!elementTypes.empty() && "expected at least 1 element type");

  mlir::MLIRContext *ctx = elementTypes.front().second.getContext();
  return Base::get(ctx, elementTypes);
}

/// Returns the element types of this struct type.
llvm::ArrayRef<StructType::FieldTy> StructType::getFields() const {
  // 'getImpl' returns a pointer to the internal storage instance.
  return getImpl()->fields;
}

size_t StructType::getNumFields() const { return getFields().size(); }

Type StructType::parse(DialectAsmParser &parser) {
  if (parser.parseLess())
    return nullptr;
  Builder &builder = parser.getBuilder();

  SmallVector<StructType::FieldTy, 3> elementTypes;
  do {
    StringRef name;
    if (parser.parseKeyword(&name) || parser.parseColon())
      return nullptr;

    StructType::FieldTy elementType;
    elementType.first = StringAttr::get(builder.getContext(), name);
    if (parser.parseType(elementType.second))
      return nullptr;

    elementTypes.push_back(elementType);
  } while (succeeded(parser.parseOptionalComma()));

  if (parser.parseGreater())
    return Type();
  return StructType::get(elementTypes);
}

void StructType::print(DialectAsmPrinter &os) const {
  // Print the struct type according to the parser format.
  os << "struct<";
  auto fields = getFields();
  for (unsigned i = 0; i < getNumFields(); i++) {
    if (i != 0)
      os << ", ";
    os << fields[i].first.getValue() << " : " << fields[i].second;
  }
  os << '>';
}

} // namespace types
} // namespace arc