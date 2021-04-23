//! Module for converting errors emitted by `LALRPOP` into compiler diagnostics.

use crate::compiler::ast::lower::source::lexer::sem_tokens::Token;
use crate::compiler::info::diags::{Diagnostic, Error};
use crate::compiler::info::files::{ByteIndex, FileId, Loc};

/// Dropped tokens + errors produced while parsing with LALRPOP.
pub(crate) type ErrorRecovery = lalrpop_util::ErrorRecovery<ByteIndex, Token, ()>;

/// Errors produced while parsing with LALRPOP.
pub(crate) type ParseError = lalrpop_util::ParseError<ByteIndex, Token, ()>;

impl Diagnostic {
    /// Converts an LALRPOP `ErrorRecovery` into a `Diagnostic`.
    pub(crate) fn from(recovery: ErrorRecovery, file: FileId) -> impl Into<Self> {
        match recovery.error {
            /// User errors (lexer errors) are handled by the lexer.
            ParseError::User { error: () } => unreachable!(),
            ParseError::ExtraToken { token: (l, t, r) } => Error::ExtraToken {
                found: t,
                loc: Loc::from_range(file, l..r),
            },
            ParseError::InvalidToken { location: l } => Error::InvalidToken {
                loc: Loc::from_range(file, l..l),
            },
            ParseError::UnrecognizedEOF {
                location: l,
                expected,
            } => Error::UnrecognizedEOF {
                loc: Loc::from_range(file, l..l),
                expected,
            },
            ParseError::UnrecognizedToken {
                token: (l, t, r),
                expected,
            } => Error::UnrecognizedToken {
                found: t,
                loc: Loc::from_range(file, l..r),
                expected,
            },
        }
    }
}