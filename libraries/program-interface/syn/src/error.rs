//! Errors for the SPL interface instruction parser.

#[derive(Clone, Debug, Eq, thiserror::Error, PartialEq)]
pub enum SplProgramInterfaceError {
    #[error("Error parsing interface attribute")]
    ParseError,
    #[error("Invalid interface namespace")]
    InvalidInterfaceNamespace,
    #[error("Missing required instruction for interface")]
    InstructionMissing,
    #[error("Instruction not found")]
    InstructionNotFound,
    #[error("Missing argument(s) for instruction")]
    MissingArgument,
}
