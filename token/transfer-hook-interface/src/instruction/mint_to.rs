//! Instruction for minting new tokens

use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use spl_type_length_value::discriminator::{Discriminator, TlvDiscriminator};

use super::HookInterfaceInstruction;

/// Instruction for minting new tokens
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct MintTo {
    amount: u64,
}
impl TlvDiscriminator for MintTo {
    const TLV_DISCRIMINATOR: Discriminator = Discriminator::new(MINT_TO_DISCRIMINATOR);
}
impl<'a> HookInterfaceInstruction<'a> for MintTo {
    const DISCRIMINATOR_SLICE: &'a [u8] = &MINT_TO_DISCRIMINATOR;
}
/// First 8 bytes of `hash::hashv(&["spl-hook-interface:mint-to"])`
const MINT_TO_DISCRIMINATOR: [u8; 8] = [143, 74, 223, 72, 254, 24, 18, 53];

impl MintTo {
    /// Packs the instruction into a buffer
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = vec![];
        buf.extend_from_slice(Self::DISCRIMINATOR_SLICE);
        buf.extend_from_slice(&self.amount.to_le_bytes());
        buf
    }

    /// Creates an `Execute` instruction, provided all of the additional required
    /// account metas
    #[allow(clippy::too_many_arguments)]
    pub fn execute_with_extra_account_metas(
        program_id: &Pubkey,
        mint_pubkey: &Pubkey,
        destination_pubkey: &Pubkey,
        authority_pubkey: &Pubkey,
        validate_state_pubkey: &Pubkey,
        additional_accounts: &[AccountMeta],
        amount: u64,
    ) -> Instruction {
        let mut instruction = Self::execute(
            program_id,
            mint_pubkey,
            destination_pubkey,
            authority_pubkey,
            validate_state_pubkey,
            amount,
        );
        instruction.accounts.extend_from_slice(additional_accounts);
        instruction
    }

    /// Creates an `Execute` instruction, without the additional accounts
    #[allow(clippy::too_many_arguments)]
    pub fn execute(
        program_id: &Pubkey,
        mint_pubkey: &Pubkey,
        destination_pubkey: &Pubkey,
        authority_pubkey: &Pubkey,
        validate_state_pubkey: &Pubkey,
        amount: u64,
    ) -> Instruction {
        let data = MintTo { amount }.pack();
        let accounts = vec![
            AccountMeta::new_readonly(*mint_pubkey, false),
            AccountMeta::new_readonly(*destination_pubkey, false),
            AccountMeta::new_readonly(*authority_pubkey, false),
            AccountMeta::new_readonly(*validate_state_pubkey, false),
        ];
        Instruction {
            program_id: *program_id,
            accounts,
            data,
        }
    }
}
