use miette::Diagnostic;
use std::io;
use thiserror::Error;

/**
 * The errors that could occur while assembling a file.
 */

#[derive(Debug, Diagnostic, Error)]
pub enum AssemblerError {
    /// An error due to IO operations.
    #[error(transparent)]
    #[diagnostic(code(assembler::errors::io))]
    IoError {
        /// The source of the error.
        source: io::Error,
    },

    /// An error occured during parsing.
    #[error(transparent)]
    #[diagnostic(code(assembler::errors::parse))]
    ParseError {
        /// The source of the error.
        source: nom::error::ParseError<dyn Span<'a>>,
    },
}

/**
 * IO errors are automatically converted into AssemblerErrors.
 */
impl From<io::Error> for AssemblerError {
    fn from(source: io::Error) -> Self {
        Self::IoError { source }
    }
}
