" Vim syntax file
" Language: arc

if version < 600
  syntax clear
elseif exists("b:current_syntax")
  finish
endif

runtime! syntax/rust.vim syntax/rust/*.vim

let b:current_syntax = "arc"

"=============================================================================
" Overrides
"=============================================================================
hi clear rustMacroVariable
syn clear rustMacroVariable
syn match rustSigil /&\s\+[&~@*$][^)= \t\r\n]/me=e-1,he=e-1  display
syn match rustSigil /[&~@*$][^)= \t\r\n]/me=e-1,he=e-1  display
"=============================================================================
" Operators
"=============================================================================
syn keyword arcOperator **
syn keyword arcOperator and
syn keyword arcOperator or
syn keyword arcOperator xor
syn keyword arcOperator band
syn keyword arcOperator bor
syn keyword arcOperator bxor
syn keyword arcOperator is
hi def link arcOperator Operator
"=============================================================================
" Statements
"=============================================================================
syn keyword arcStatement def
syn keyword arcStatement task
syn keyword arcStatement on
syn keyword arcStatement emit
syn keyword arcStatement val
syn keyword arcStatement var
hi def link arcStatement Statement
"=============================================================================
" Conditionals
"=============================================================================
hi def link arcConditional Conditional
"=============================================================================
" Reserved Keywords
"=============================================================================
syn keyword arcStatement not
syn keyword arcStatement in
syn keyword arcStatement is
syn keyword arcStatement fun
hi def link arcKeyword Keyword
"=============================================================================
" Primitive Types
"=============================================================================
syn keyword arcType fun
syn keyword arcType unit
hi def link arcType Type
"=============================================================================
" Floats
"=============================================================================
hi def link arcFloat Float
"=============================================================================
" Constants
"=============================================================================
syn match arcConstant "[[:digit:]]\+ns"
syn match arcConstant "[[:digit:]]\+us"
syn match arcConstant "[[:digit:]]\+ms"
syn match arcConstant "[[:digit:]]\+s"
syn match arcConstant "[[:digit:]]\+m"
syn match arcConstant "[[:digit:]]\+h"
syn match arcConstant "[[:digit:]]\+d"
syn match arcConstant "[[:digit:]]\+w"
hi def link arcConstant Constant
"=============================================================================
" Comments
"=============================================================================
syn match arcComment "#.*"
hi def link arcComment Comment
