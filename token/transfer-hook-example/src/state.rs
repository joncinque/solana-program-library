//! State helpers for working with the example program

use solana_program::{instruction::AccountMeta, program_error::ProgramError};
use spl_tlv_account_resolution::state::ExtraAccountMetas;
use spl_transfer_hook_interface::instruction::{mint_to::MintTo, transfer::Transfer};

/// Generate example data to be used directly in an account for testing
pub fn example_data(account_metas: &[AccountMeta]) -> Result<Vec<u8>, ProgramError> {
    let account_size = ExtraAccountMetas::size_of(account_metas.len())?;
    let mut data = vec![0; account_size];
    ExtraAccountMetas::init_with_account_metas::<MintTo>(&mut data, &account_metas[..3])?;
    ExtraAccountMetas::init_with_account_metas::<Transfer>(&mut data, &account_metas[3..])?;
    Ok(data)
}
