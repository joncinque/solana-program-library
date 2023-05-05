//! The program's instructions

use solana_program::program_error::ProgramError;
use spl_transfer_hook_interface::instruction::{
    init::InitializeExtraAccountMetas, mint_to::MintTo, transfer::Transfer,
    unpack_with_discriminator_checked, HookInterfaceInstruction,
};
use spl_type_length_value::discriminator::{Discriminator, TlvDiscriminator};
use std::convert::TryInto;

/// The program's instructions
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum MyInstruction {
    /// Initializes the extra account metas on an account, writing into
    /// the first open TLV space.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[w]` Account with extra account metas
    ///   1. `[]` Mint
    ///   2. `[s]` Mint authority
    ///   3. `[]` System program
    ///   4..4+M `[]` `M` additional accounts, to be written to validation data
    ///
    Init,
    /// Runs additional mint logic.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[]` Token mint
    ///   1. `[]` Destination account
    ///   2. `[]` Mint authority
    ///   3. `[]` Validation account
    ///   4..4+M `[]` `M` additional accounts, written in validation account data
    ///
    MintTo {
        /// Amount of tokens to mint
        amount: u64,
    },
    /// Runs additional transfer logic.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[]` Source account
    ///   1. `[]` Token mint
    ///   2. `[]` Destination account
    ///   3. `[]` Source account's owner/delegate
    ///   4. `[]` Validation account
    ///   5..5+M `[]` `M` additional accounts, written in validation account data
    ///
    Transfer {
        /// Amount of tokens to transfer
        amount: u64,
    },
    /// Arbitrary additional custom instruction for demonstration.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[]` Token mint
    ///   1. `[]` Mint authority
    ///
    Arbitrary {
        /// Arbitrary argument for demonstration.
        arg: u8,
    },
}

/// Intruction for initializing extra account metas in the validation account
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Arbitrary {}
impl TlvDiscriminator for Arbitrary {
    const TLV_DISCRIMINATOR: Discriminator = Discriminator::new(ARBITRARY_DISCRIMINATOR);
}
impl<'a> HookInterfaceInstruction<'a> for Arbitrary {
    const DISCRIMINATOR_SLICE: &'a [u8] = &ARBITRARY_DISCRIMINATOR;
}
/// First 8 bytes of `hash::hashv(&["spl-hook-interface-example:arbitrary"])`
const ARBITRARY_DISCRIMINATOR: [u8; 8] = [159, 188, 87, 78, 151, 190, 23, 195];

impl MyInstruction {
    /// Unpacks a byte buffer into a [MyInstruction](enum.MyInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (discriminator, rest) = unpack_with_discriminator_checked(input)?;
        Ok(match discriminator {
            InitializeExtraAccountMetas::DISCRIMINATOR_SLICE => Self::Init,
            MintTo::DISCRIMINATOR_SLICE => {
                let amount = rest
                    .get(..8)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(ProgramError::InvalidInstructionData)?;
                Self::Transfer { amount }
            }
            Transfer::DISCRIMINATOR_SLICE => {
                let amount = rest
                    .get(..8)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(ProgramError::InvalidInstructionData)?;
                Self::Transfer { amount }
            }
            Arbitrary::DISCRIMINATOR_SLICE => {
                let arg = rest
                    .get(..8)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u8::from_le_bytes)
                    .ok_or(ProgramError::InvalidInstructionData)?;
                Self::Arbitrary { arg }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    /// Packs a [TokenInstruction](enum.TokenInstruction.html) into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = vec![];
        match self {
            Self::Init => {
                buf.extend_from_slice(InitializeExtraAccountMetas::DISCRIMINATOR_SLICE);
            }
            Self::MintTo { amount } => {
                buf.extend_from_slice(MintTo::DISCRIMINATOR_SLICE);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::Transfer { amount } => {
                buf.extend_from_slice(Transfer::DISCRIMINATOR_SLICE);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::Arbitrary { arg } => {
                buf.extend_from_slice(Transfer::DISCRIMINATOR_SLICE);
                buf.extend_from_slice(&arg.to_le_bytes());
            }
        };
        buf
    }
}
