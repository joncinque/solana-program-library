//! Error types

use {
    num_derive::FromPrimitive,
    solana_program::{
        decode_error::DecodeError,
        msg,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};

/// Errors that may be returned by the interface.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum HookInterfaceError {
    /// Incorrect account provided
    #[error("Incorrect account provided")]
    IncorrectAccount,
    /// Mint has no mint authority
    #[error("Mint has no mint authority")]
    MintHasNoMintAuthority,
    /// Incorrect mint authority has signed the instruction
    #[error("Incorrect mint authority has signed the instruction")]
    IncorrectMintAuthority,
}
impl From<HookInterfaceError> for ProgramError {
    fn from(e: HookInterfaceError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for HookInterfaceError {
    fn type_of() -> &'static str {
        "HookInterfaceError"
    }
}

impl PrintProgramError for HookInterfaceError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            Self::IncorrectAccount => msg!("Incorrect account provided"),
            Self::MintHasNoMintAuthority => msg!("Mint has no mint authority"),
            Self::IncorrectMintAuthority => {
                msg!("Incorrect mint authority has signed the instruction")
            }
        }
    }
}
