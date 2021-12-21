type token =
  | ParenL
  | ParenR
  | ParenLR
  | BrackL
  | BrackR
  | BrackLR
  | PoundBraceL
  | BraceL
  | BraceR
  | BraceLR
  | AngleL
  | AngleR
  | AngleLR
(*= Operators ==============================================================*)
  | Bang
  | Neq
  | Percent
  | Star
  | StarStar
  | Plus
  | Comma
  | Minus
  | Dot
  | DotDot
  | DotDotEq
  | Slash
  | Colon
  | ColonColon
  | Semi
  | Leq
  | Eq
  | EqEq
  | Imply
  | Geq
  | AtSign
  | Underscore
  | Bar
  | BarBar
  | Ampersand
(*= Keywords ================================================================*)
  | After
  | And
  | As
  | Break
  | Band
  | Bor
  | Bxor
  | By
  | Class
  | Continue
  | Def
  | Desc
  | Duration
  | Else
  | Emit
  | Enum
  | Every
  | Extern
  | For
  | From
  | Fun
  | Group
  | If
  | In
  | Instance
  | Is
  | Join
  | Let
  | Loop
  | Match
  | Mod
  | Not
  | On
  | Or
  | Of
  | Order
  | Return
  | Reduce
  | Step
  | Task
  | Type
  | Val
  | Var
  | Where
  | Window
  | While
  | Unwrap
  | Enwrap
  | Use
  | Xor
  | Yield
(*= Identifiers and Literals ================================================*)
  | Name of string
  | Int of int
  | Float of float
  | Bool of bool
  | Char of char
  | String of string
  | Unit
  | DurationNs of int
  | DurationUs of int
  | DurationMs of int
  | DurationS of int
  | DurationM of int
  | DurationH of int
  | DurationD of int
  | DurationW of int
(*     LitDurationMo, *)
(*     LitDurationY, *)
  | Date of string
  | DateTime of string
  | DateTimeZone of string
  | Eof
