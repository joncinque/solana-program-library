use solana_program::pubkey::Pubkey;
use spl_program_interface::*;
use spl_token_metadata_interface::instruction::Field;

#[derive(SplProgramInterface)]
pub enum SampleToken {
    #[interface(spl_token_metadata_interface::metadata_initialize)]
    Initialize {
        name: String,
        symbol: String,
        uri: String,
    },
    #[interface(spl_token_metadata_interface::update_field)]
    UpdateField { field: Field, value: String },
    #[interface(spl_token_metadata_interface::remove_a_key)]
    RemoveKey { key: String },
    #[interface(spl_token_metadata_interface::update_authority)]
    UpdateAuthority { new_authority: Option<Pubkey> },
    #[interface(spl_token_metadata_interface::emitting)]
    Emit {
        start: Option<u64>,
        end: Option<u64>,
    },
}

#[test]
fn test_compiles() {}
