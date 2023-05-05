//! Supported interface instructions

pub mod init;
pub mod mint_to;
pub mod transfer;

use solana_program::program_error::ProgramError;
use spl_type_length_value::discriminator::{Discriminator, TlvDiscriminator};

/// Trait for programs implementing the interface to add
/// a discriminator to any custom instructions
pub trait HookInterfaceInstruction<'a>: TlvDiscriminator {
    /// Slice representation of the TLV Discriminator for matching
    const DISCRIMINATOR_SLICE: &'a [u8];
}

/// Helper method for programs implementing the interface to build a processor
pub fn unpack_with_discriminator_checked(input: &[u8]) -> Result<(&[u8], &[u8]), ProgramError> {
    if input.len() < Discriminator::LENGTH {
        return Err(ProgramError::InvalidInstructionData);
    }
    Ok(input.split_at(Discriminator::LENGTH))
}
