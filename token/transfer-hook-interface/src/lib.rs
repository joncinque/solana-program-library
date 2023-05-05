//! Crate defining an interface for performing a hook on transfer, where the
//! token program calls into a separate program with additional accounts after
//! all other logic, to be sure that a transfer has accomplished all required
//! preconditions.

#![allow(clippy::integer_arithmetic)]
#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

pub mod error;
pub mod inline_spl_token;
pub mod instruction;
pub mod invoke;
pub mod offchain;

use error::HookInterfaceError;
// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke_signed,
    program_error::ProgramError, pubkey::Pubkey, system_instruction,
};
use spl_tlv_account_resolution::state::ExtraAccountMetas;

/// Namespace for all programs implementing transfer-hook
pub const NAMESPACE: &str = "spl-transfer-hook-interface";

/// Seed for the state
const EXTRA_ACCOUNT_METAS_SEED: &[u8] = b"extra-account-metas";

/// Get the state address PDA
pub fn get_extra_account_metas_address(mint: &Pubkey, program_id: &Pubkey) -> Pubkey {
    get_extra_account_metas_address_and_bump_seed(mint, program_id).0
}

/// Function used by programs implementing the interface, when creating the PDA,
/// to also get the bump seed
pub fn get_extra_account_metas_address_and_bump_seed(
    mint: &Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(&collect_extra_account_metas_seeds(mint), program_id)
}

/// Function used by programs implementing the interface, when creating the PDA,
/// to get all of the PDA seeds
pub fn collect_extra_account_metas_seeds(mint: &Pubkey) -> [&[u8]; 2] {
    [EXTRA_ACCOUNT_METAS_SEED, mint.as_ref()]
}

/// Function used by programs implementing the interface, when creating the PDA,
/// to sign for the PDA
pub fn collect_extra_account_metas_signer_seeds<'a>(
    mint: &'a Pubkey,
    bump_seed: &'a [u8],
) -> [&'a [u8]; 3] {
    [EXTRA_ACCOUNT_METAS_SEED, mint.as_ref(), bump_seed]
}

/// Function used by programs implementing the interface to create
/// a validation account to hold extra account metas
pub fn create_validation_account_checked(
    program_id: &Pubkey,
    mint_info: &AccountInfo<'_>,
    validation_account: &AccountInfo<'_>,
    authority_info: &AccountInfo<'_>,
    length: usize,
) -> ProgramResult {
    // check that the mint authority is valid without fully deserializing
    let mint_authority = inline_spl_token::get_mint_authority(&mint_info.try_borrow_data()?)?;
    let mint_authority = mint_authority.ok_or(HookInterfaceError::MintHasNoMintAuthority)?;

    // Check signers
    if !authority_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if *authority_info.key != mint_authority {
        return Err(HookInterfaceError::IncorrectMintAuthority.into());
    }

    // Check validation account
    let (expected_validation_address, bump_seed) =
        get_extra_account_metas_address_and_bump_seed(mint_info.key, program_id);
    if expected_validation_address != *validation_account.key {
        return Err(ProgramError::InvalidSeeds);
    }

    // Create the account
    let bump_seed = [bump_seed];
    let signer_seeds = collect_extra_account_metas_signer_seeds(mint_info.key, &bump_seed);
    let account_size = ExtraAccountMetas::size_of(length)?;
    invoke_signed(
        &system_instruction::allocate(validation_account.key, account_size as u64),
        &[validation_account.clone()],
        &[&signer_seeds],
    )?;
    invoke_signed(
        &system_instruction::assign(validation_account.key, program_id),
        &[validation_account.clone()],
        &[&signer_seeds],
    )?;
    Ok(())
}
