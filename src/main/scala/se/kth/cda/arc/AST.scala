package se.kth.cda.arc

import org.antlr.v4.runtime._
import org.antlr.v4.runtime.tree._
import java.io.PrintStream
import scala.util.{ Try, Success, Failure }

sealed trait ASTNode {
  def inputText: String;
  def line: Int;
}
object AST {
  case class Symbol(name: String, token: Option[Token] = None, scopeId: Int = 0) extends ASTNode {
    def text: String = name;
    override def inputText: String = token.map(_.getText).getOrElse("<no associated token>");
    override def line: Int = token.map(_.getLine).getOrElse(-1);
  }
  case class Program(macros: List[Macro], expr: Expr, ctx: ArcParser.ProgramContext) extends ASTNode {
    override def inputText: String = ctx.getText;
    override def line: Int = ctx.getStart.getLine;
  }
  case class Macro(name: Symbol, parameters: Vector[Symbol], body: Expr, ctx: ArcParser.MacroContext) extends ASTNode {
    override def inputText: String = ctx.getText;
    override def line: Int = ctx.getStart.getLine;
  }
  case class Expr(kind: ExprKind, ty: Type, ctx: ParserRuleContext, annotations: Option[Annotations] = None) extends ASTNode {
    override def inputText: String = ctx.getText;
    override def line: Int = ctx.getStart.getLine;
  }
  case class Annotations(params: Vector[(String, Any)]);
  case class Parameter(name: Symbol, ty: Type);
  sealed trait ExprKind;
  object ExprKind {
    case class Let(name: Symbol, bindingTy: Type, value: Expr, body: Expr) extends ExprKind;
    case class Lambda(params: Vector[Parameter], body: Expr) extends ExprKind;

    sealed trait Literal[T] extends ExprKind {
      def raw: String;
      def value: T;
    }
    object Literal {
      case class I8(raw: String, value: Int) extends Literal[Int];
      def tryI8(raw: String, value: Int): Try[I8] =
        tryLiteral[Int, I8]("i8", value, -128, 127, (v) => I8(raw, v));
      case class I16(raw: String, value: Int) extends Literal[Int];
      def tryI16(raw: String, value: Int): Try[I16] =
        tryLiteral[Int, I16]("i16", value, -32768, 32767, (v) => I16(raw, v));
      case class I32(raw: String, value: Int) extends Literal[Int];
      def tryI32(raw: String, value: Int): Try[I32] =
        tryLiteral[Int, I32]("i32", value, Int.MinValue, Int.MaxValue, (v) => I32(raw, v));
      case class I64(raw: String, value: Long) extends Literal[Long];
      def tryI64(raw: String, value: Long): Try[I64] =
        tryLiteral[Long, I64]("i64", value, Long.MinValue, Long.MaxValue, (v) => I64(raw, v));
      case class U8(raw: String, value: Int) extends Literal[Int];
      def tryU8(raw: String, value: Int): Try[U8] =
        tryLiteral[Int, U8]("u8", value, 0, 255, (v) => U8(raw, v));
      case class U16(raw: String, value: Int) extends Literal[Int];
      def tryU16(raw: String, value: Int): Try[U16] =
        tryLiteral[Int, U16]("u16", value, 0, 65535, (v) => U16(raw, v));
      case class U32(raw: String, value: Long) extends Literal[Long];
      def tryU32(raw: String, value: Long): Try[U32] =
        tryLiteral[Long, U32]("u32", value, 0, 4294967295l, (v) => U32(raw, v));
      case class U64(raw: String, value: BigInt) extends Literal[BigInt];
      def tryU64(raw: String, value: BigInt): Try[U64] =
        tryLiteral[BigInt, U64]("u64", value, BigInt(0), BigInt(Long.MaxValue) << 1, (v) => U64(raw, v));
      case class F32(raw: String, value: Float) extends Literal[Float];
      case class F64(raw: String, value: Double) extends Literal[Double];
      case class Bool(raw: String, value: Boolean) extends Literal[Boolean];
      case class UnitL(raw: String, value: Unit = ()) extends Literal[Unit];
      case class StringL(raw: String, value: String) extends Literal[String];

      def tryLiteral[V, L <: Literal[V]](ty: String, value: V, lowerBound: V, upperBound: V, constr: (V) => L)(implicit ord: Ordering[V]): Try[L] = {
        if (ord.gteq(value, lowerBound) && ord.lteq(value, upperBound)) {
          Success(constr(value))
        } else {
          Failure(new LiteralException(value.toString(), ty, (lowerBound.toString(), upperBound.toString())))
        }
      }

      class LiteralException(message: String) extends Exception(message) {

        def this(value: String, ty: String, bounds: (String, String)) {
          this(s"$ty literal value does not fit into bounds: ${bounds._1} <= $value <= ${bounds._2}")
        }
        def this(message: String, cause: Throwable) {
          this(message)
          initCause(cause)
        }

        def this(cause: Throwable) {
          this(Option(cause).map(_.toString).orNull, cause)
        }

        def this() {
          this(null: String)
        }
      }
    }
    case class Cast(ty: Types.Scalar, expr: Expr) extends ExprKind;
    case class ToVec(expr: Expr) extends ExprKind;
    case class Ident(name: Symbol) extends ExprKind;
    case class MakeStruct(elems: Vector[Expr]) extends ExprKind;
    case class MakeVec(elems: Vector[Expr]) extends ExprKind;
    case class If(cond: Expr, onTrue: Expr, onFalse: Expr) extends ExprKind;
    case class Select(cond: Expr, onTrue: Expr, onFalse: Expr) extends ExprKind;
    case class Iterate(initial: Expr, updateFunc: Expr) extends ExprKind;
    case class Broadcast(expr: Expr) extends ExprKind;
    case class Serialize(expr: Expr) extends ExprKind;
    case class Deserialize(ty: Type, expr: Expr) extends ExprKind;
    case class CUDF(reference: Either[Symbol, Expr], args: Vector[Expr], returnType: Type) extends ExprKind;
    case class Zip(params: Vector[Expr]) extends ExprKind;
    case class For(iterator: Iter, builder: Expr, body: Expr) extends ExprKind;
    case class Len(expr: Expr) extends ExprKind;
    case class Lookup(data: Expr, key: Expr) extends ExprKind;
    case class Slice(data: Expr, index: Expr, size: Expr) extends ExprKind;
    case class Sort(data: Expr, keyFunc: Expr) extends ExprKind;
    case class Negate(expr: Expr) extends ExprKind;
    case class Not(expr: Expr) extends ExprKind;
    case class UnaryOp(kind: UnaryOpKind.UnaryOpKind, expr: Expr) extends ExprKind;
    case class Merge(builder: Expr, value: Expr) extends ExprKind;
    case class Result(expr: Expr) extends ExprKind;
    case class NewBuilder(ty: BuilderType, arg: Option[Expr]) extends ExprKind;
    case class BinOp(kind: BinOpKind, left: Expr, right: Expr) extends ExprKind;
    case class Application(funcExpr: Expr, params: Vector[Expr]) extends ExprKind;
    case class Projection(structExpr: Expr, index: Int) extends ExprKind;
    case class Ascription(expr: Expr, ty: Type) extends ExprKind;

  }

  object IterKind extends Enumeration {
    type IterKind = Value;
    val ScalarIter, // A standard scalar iterator.
    SimdIter, // A vector iterator.
    FringeIter, // A fringe iterator, handling the fringe of a vector iter.
    NdIter, // multi-dimensional nd-iter
    RangeIter, // An iterator over a range
    NextIter, // An iterator over a function that returns one value at a time
    UnknownIter // iterator still needs to be inferred from data types
    = Value;
  }

  case class Iter(kind: IterKind.IterKind, data: Expr,
                  start: Option[Expr] = None, end: Option[Expr] = None,
                  stride: Option[Expr] = None, strides: Option[Expr] = None, shape: Option[Expr] = None);

  object UnaryOpKind extends Enumeration {
    type UnaryOpKind = Value;
    val Exp, Sin, Cos, Tan, ASin, ACos, ATan, Sinh, Cosh, Tanh, Log, Erf, Sqrt = Value;
    def print(k: UnaryOpKind): String = k match {
      case Exp  => "exp"
      case Sin  => "sin"
      case Cos  => "cos"
      case Tan  => "tan"
      case ASin => "asin"
      case ACos => "acos"
      case ATan => "atan"
      case Sinh => "sinh"
      case Cosh => "cosh"
      case Tanh => "tanh"
      case Log  => "log"
      case Erf  => "erf"
      case Sqrt => "sqrt"
    }
  }

  sealed trait BinOpKind {
    def isInfix: Boolean;
    def symbol: String;
  }
  object BinOpKind {
    case object Min extends BinOpKind {
      override def isInfix = false;
      override def symbol = "min";
    }
    case object Max extends BinOpKind {
      override def isInfix = false;
      override def symbol = "max";
    }
    case object Pow extends BinOpKind {
      override def isInfix = false;
      override def symbol = "pow";
    }
    case object Mult extends BinOpKind {
      override def isInfix = true;
      override def symbol = "*";
    }
    case object Div extends BinOpKind {
      override def isInfix = true;
      override def symbol = "/";
    }
    case object Modulo extends BinOpKind {
      override def isInfix = true;
      override def symbol = "%";
    }
    case object Add extends BinOpKind {
      override def isInfix = true;
      override def symbol = "+";
    }
    case object Sub extends BinOpKind {
      override def isInfix = true;
      override def symbol = "-";
    }
    case object LessThan extends BinOpKind {
      override def isInfix = true;
      override def symbol = "<";
    }
    case object GreaterThan extends BinOpKind {
      override def isInfix = true;
      override def symbol = ">";
    }
    case object LEq extends BinOpKind {
      override def isInfix = true;
      override def symbol = "<=";
    }
    case object GEq extends BinOpKind {
      override def isInfix = true;
      override def symbol = ">=";
    }
    case object Equals extends BinOpKind {
      override def isInfix = true;
      override def symbol = "==";
    }
    case object NEq extends BinOpKind {
      override def isInfix = true;
      override def symbol = "!=";
    }
    case object And extends BinOpKind {
      override def isInfix = true;
      override def symbol = "&";
    }
    case object Xor extends BinOpKind {
      override def isInfix = true;
      override def symbol = "^";
    }
    case object Or extends BinOpKind {
      override def isInfix = true;
      override def symbol = "|";
    }
    case object LogicalAnd extends BinOpKind {
      override def isInfix = true;
      override def symbol = "&&";
    }
    case object LogicalOr extends BinOpKind {
      override def isInfix = true;
      override def symbol = "||";
    }
  }

}
