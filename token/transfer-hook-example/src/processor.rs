//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_tlv_account_resolution::state::ExtraAccountMetas;
use spl_transfer_hook_interface::{
    create_validation_account_checked,
    error::HookInterfaceError,
    get_extra_account_metas_address,
    instruction::{mint_to::MintTo, transfer::Transfer},
};
use spl_type_length_value::state::TlvStateBorrowed;

use crate::instruction::MyInstruction;

/// Processes a HookInterface::InitializeExtraAccountMetas instruction
pub fn process_init(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let extra_account_metas_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;
    let _system_program_info = next_account_info(account_info_iter)?;

    let accounts_for_minting = [
        next_account_info(account_info_iter)?.to_owned(),
        next_account_info(account_info_iter)?.to_owned(),
        next_account_info(account_info_iter)?.to_owned(),
    ];
    let accounts_for_transferring = [
        next_account_info(account_info_iter)?.to_owned(),
        next_account_info(account_info_iter)?.to_owned(),
        next_account_info(account_info_iter)?.to_owned(),
    ];

    let extra_account_metas_len = accounts_for_minting.len() + accounts_for_transferring.len();

    // Create the validation account with checks
    create_validation_account_checked(
        program_id,
        mint_info,
        extra_account_metas_info,
        authority_info,
        extra_account_metas_len,
    )?;

    // Write the data
    let mut data = extra_account_metas_info.try_borrow_mut_data()?;
    // Write the accounts required for minting
    ExtraAccountMetas::init_with_account_infos::<MintTo>(
        &mut data,
        accounts_for_minting.as_slice(),
    )?;
    // Write the accounts required for transferring
    ExtraAccountMetas::init_with_account_infos::<Transfer>(
        &mut data,
        accounts_for_transferring.as_slice(),
    )?;

    Ok(())
}

/// Processes a HookInterface::MintTo instruction
pub fn process_mint_to(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let mint_info = next_account_info(account_info_iter)?;
    let _destination_account_info = next_account_info(account_info_iter)?;
    let _authority_info = next_account_info(account_info_iter)?;
    let extra_account_metas_info = next_account_info(account_info_iter)?;

    // For the example program, we just check that the correct pda and validation
    // pubkeys are provided
    let expected_validation_address = get_extra_account_metas_address(mint_info.key, program_id);
    if expected_validation_address != *extra_account_metas_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    let data = extra_account_metas_info.try_borrow_data()?;
    let state = TlvStateBorrowed::unpack(&data).unwrap();
    let extra_account_metas = ExtraAccountMetas::unpack_with_tlv_state::<MintTo>(&state)?;

    // if incorrect number of are provided, error
    let extra_account_infos = account_info_iter.as_slice();
    let account_metas = extra_account_metas.data();
    if extra_account_infos.len() != account_metas.len() {
        return Err(HookInterfaceError::IncorrectAccount.into());
    }

    // Let's assume that they're provided in the correct order
    for (i, account_info) in extra_account_infos.iter().enumerate() {
        if &account_metas[i] != account_info {
            return Err(HookInterfaceError::IncorrectAccount.into());
        }
    }

    // * MintTo logic *

    Ok(())
}

/// Processes a HookInterface::Transfer instruction
pub fn process_transfer(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let _source_account_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let _destination_account_info = next_account_info(account_info_iter)?;
    let _authority_info = next_account_info(account_info_iter)?;
    let extra_account_metas_info = next_account_info(account_info_iter)?;

    // For the example program, we just check that the correct pda and validation
    // pubkeys are provided
    let expected_validation_address = get_extra_account_metas_address(mint_info.key, program_id);
    if expected_validation_address != *extra_account_metas_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    let data = extra_account_metas_info.try_borrow_data()?;
    let state = TlvStateBorrowed::unpack(&data).unwrap();
    let extra_account_metas = ExtraAccountMetas::unpack_with_tlv_state::<Transfer>(&state)?;

    // if incorrect number of are provided, error
    let extra_account_infos = account_info_iter.as_slice();
    let account_metas = extra_account_metas.data();
    if extra_account_infos.len() != account_metas.len() {
        return Err(HookInterfaceError::IncorrectAccount.into());
    }

    // Let's assume that they're provided in the correct order
    for (i, account_info) in extra_account_infos.iter().enumerate() {
        if &account_metas[i] != account_info {
            return Err(HookInterfaceError::IncorrectAccount.into());
        }
    }

    // * Transfer logic *

    Ok(())
}

/// Processes an Arbitrary instruction (specific to this program)
pub fn process_arbitrary(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _arg: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let _mint_info = next_account_info(account_info_iter)?;
    let _authority_info = next_account_info(account_info_iter)?;

    // * Arbitrary program logic *

    Ok(())
}

/// Processes an [Instruction](enum.Instruction.html).
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    let instruction = MyInstruction::unpack(input)?;

    match instruction {
        MyInstruction::Init => {
            msg!("Instruction: Init");
            process_init(program_id, accounts)
        }
        MyInstruction::MintTo { amount } => {
            msg!("Instruction: Mint To");
            process_mint_to(program_id, accounts, amount)
        }
        MyInstruction::Transfer { amount } => {
            msg!("Instruction: Transfer");
            process_transfer(program_id, accounts, amount)
        }
        MyInstruction::Arbitrary { arg } => {
            msg!("Instruction: Arbitrary");
            process_arbitrary(program_id, accounts, arg)
        }
    }
}
