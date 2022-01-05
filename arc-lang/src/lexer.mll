{
  open Lexing
  open Token

  exception SyntaxError of string

  let next_line lexbuf =
    let pos = lexbuf.lex_curr_p in
    lexbuf.lex_curr_p <-
      { pos with pos_bol = pos.pos_cnum;
                pos_lnum = pos.pos_lnum + 1
      }
}

let int = ['0'-'9'] ['0'-'9']*
let digit = ['0'-'9']
let frac = '.' digit*
let exp = ['e' 'E'] ['-' '+']? digit+
let float = digit+ frac? exp?
let percentage = digit+ frac? '%'
let whitespace = [' ' '\t']+
let newline = '\r' | '\n' | "\r\n"
let name = ['a'-'z' 'A'-'Z' '_'] ['a'-'z' 'A'-'Z' '0'-'9' '_']*
let unit = "unit"

rule main =
  parse
  | "("        { ParenL }
  | ")"        { ParenR }
  | "["        { BrackL }
  | "]"        { BrackR }
  | "#{"       { PoundBraceL }
  | "{"        { BraceL }
  | "}"        { BraceR }
  | "<"        { AngleL }
  | ">"        { AngleR }
(*= Operators ==============================================================*)
  | "!"        { Bang }
  | "!="       { Neq }
  | "%"        { Percent }
  | "*"        { Star }
  | "**"       { StarStar }
  | "+"        { Plus }
  | ","        { Comma }
  | "-"        { Minus }
  | "."        { Dot }
  | ".."       { DotDot }
  | "..="      { DotDotEq }
  | "/"        { Slash }
  | ":"        { Colon }
  | "::"       { ColonColon }
  | ";"        { Semi }
  | "<="       { Leq }
  | "="        { Eq }
  | "=="       { EqEq }
  | "=>"       { Imply }
  | ">="       { Geq }
  | "_"        { Underscore }
  | "|"        { Bar }
  | "@"        { AtSign }
(*= Keywords ================================================================*)
  | "and"      { And }
  | "as"       { As }
  | "break"    { Break }
  | "band"     { Band }
  | "bor"      { Bor }
  | "bxor"     { Bxor }
  | "class"    { Class }
  | "continue" { Continue }
  | "def"      { Def }
  | "desc"     { Desc }
  | "duration" { Duration }
  | "else"     { Else }
  | "emit"     { Emit }
  | "enum"     { Enum }
  | "extern"   { Extern }
  | "for"      { For }
  | "from"     { From }
  | "fun"      { Fun }
  | "group"    { Group }
  | "if"       { If }
  | "in"       { In }
  | "instance" { Instance }
  | "join"     { Join }
  | "loop"     { Loop }
  | "match"    { Match }
  | "mod"      { Mod }
  | "not"      { Not }
  | "on"       { On }
  | "or"       { Or }
  | "order"    { Or }
  | "of"       { Of }
  | "return"   { Return }
  | "reduce"   { Reduce }
  | "step"     { Step }
  | "task"     { Task }
  | "type"     { Type }
  | "val"      { Val }
  | "var"      { Var }
  | "where"    { Where }
  | "while"    { While }
  | "window"   { Window }
  | "use"      { Use }
  | "xor"      { Xor }
  | "yield"    { Yield }
(*= Identifiers and Literals ================================================*)
  | name       { Name (Lexing.lexeme lexbuf) }
  | int        { Int (int_of_string (Lexing.lexeme lexbuf)) }
  | float      { Float (float_of_string (Lexing.lexeme lexbuf)) }
  | percentage {
    begin
      let i1 = lexbuf.lex_curr_pos - 1
      and i2 = lexbuf.lex_start_pos in
      Float (float_of_string (Lexing.sub_lexeme lexbuf i1 i2) /. 100.0)
    end
  }
  | "true"     { Bool true }
  | "false"    { Bool false }
  | "unit"     { Unit }
(*   | Char of char *)
  | '"'        { string (Buffer.create 17) lexbuf }
(*   | DurationNs of int *)
(*   | DurationUs of int *)
(*   | DurationMs of int *)
(*   | DurationS of int *)
(*   | DurationM of int *)
(*   | DurationH of int *)
(*   | DurationD of int *)
(*   | DurationW of int *)
(*     LitDurationMo, *)
(*     LitDurationY, *)
(*   | Date of string *)
(*   | DateTime of string *)
(*   | DateTimeZone of string *)
  | '#'        { line_comment lexbuf; main lexbuf }
  | whitespace { main lexbuf }
  | newline    { next_line lexbuf; main lexbuf }
  | _          { raise (SyntaxError ("Unexpected char: '" ^ (Lexing.lexeme lexbuf) ^ "'")) }
  | eof        { Eof }

and line_comment =
  parse
  | newline { () }
  | _ { line_comment lexbuf }

and string buf =
  parse
  | '"'       { String (Buffer.contents buf) }
  | '\\' '/'  { Buffer.add_char buf '/'; string buf lexbuf }
  | '\\' '\\' { Buffer.add_char buf '\\'; string buf lexbuf }
  | '\\' 'b'  { Buffer.add_char buf '\b'; string buf lexbuf }
  | '\\' 'f'  { Buffer.add_char buf '\012'; string buf lexbuf }
  | '\\' 'n'  { Buffer.add_char buf '\n'; string buf lexbuf }
  | '\\' 'r'  { Buffer.add_char buf '\r'; string buf lexbuf }
  | '\\' 't'  { Buffer.add_char buf '\t'; string buf lexbuf }
  | [^ '"' '\\']+
    { Buffer.add_string buf (Lexing.lexeme lexbuf);
      string buf lexbuf
    }
  | _ { raise (SyntaxError ("Illegal string character: " ^ Lexing.lexeme lexbuf)) }
  | eof { raise (SyntaxError ("String is not terminated")) }