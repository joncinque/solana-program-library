//! Instruction for initializing extra account metas in the validation account

use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};
use spl_type_length_value::discriminator::{Discriminator, TlvDiscriminator};

use super::HookInterfaceInstruction;

/// Instruction for initializing extra account metas in the validation account
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeExtraAccountMetas {}
impl TlvDiscriminator for InitializeExtraAccountMetas {
    const TLV_DISCRIMINATOR: Discriminator =
        Discriminator::new(INITIALIZE_EXTRA_ACCOUNT_METAS_DISCRIMINATOR);
}
impl<'a> HookInterfaceInstruction<'a> for InitializeExtraAccountMetas {
    const DISCRIMINATOR_SLICE: &'a [u8] = &INITIALIZE_EXTRA_ACCOUNT_METAS_DISCRIMINATOR;
}
/// First 8 bytes of `hash::hashv(&["spl-hook-interface:initialize-extra-account-metas"])`
const INITIALIZE_EXTRA_ACCOUNT_METAS_DISCRIMINATOR: [u8; 8] =
    [233, 153, 239, 113, 226, 61, 67, 134];

impl InitializeExtraAccountMetas {
    /// Packs the instruction into a buffer
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = vec![];
        buf.extend_from_slice(Self::DISCRIMINATOR_SLICE);
        buf
    }

    /// Creates a `InitializeExtraAccountMetas` instruction.
    pub fn initialize_extra_account_metas(
        program_id: &Pubkey,
        extra_account_metas_pubkey: &Pubkey,
        mint_pubkey: &Pubkey,
        authority_pubkey: &Pubkey,
        additional_accounts: &[AccountMeta],
    ) -> Instruction {
        let data = InitializeExtraAccountMetas {}.pack();

        let mut accounts = vec![
            AccountMeta::new(*extra_account_metas_pubkey, false),
            AccountMeta::new_readonly(*mint_pubkey, false),
            AccountMeta::new_readonly(*authority_pubkey, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ];
        accounts.extend_from_slice(additional_accounts);

        Instruction {
            program_id: *program_id,
            accounts,
            data,
        }
    }
}
