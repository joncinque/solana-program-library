// Mark this test as BPF-only due to current `ProgramTest` limitations when
// CPIing into the system program
#![cfg(feature = "test-sbf")]

mod program_test;

#[allow(deprecated)]
use spl_associated_token_account::create_associated_token_account as deprecated_create_associated_token_account;
use {
    program_test::program_test,
    solana_program::pubkey::Pubkey,
    solana_program_test::*,
    solana_sdk::{
        program_pack::Pack,
        signature::{Keypair, Signer},
        transaction::Transaction,
    },
    spl_associated_token_account::instruction::create_associated_token_account,
    spl_associated_token_account_client::address::{
        get_associated_token_address, get_associated_token_address_with_program_id,
    },
    spl_token::state::Account,
};

#[tokio::test]
async fn success_create() {
    let wallet_address = Pubkey::new_unique();
    let token_mint_address = Pubkey::new_unique();
    let associated_token_address =
        get_associated_token_address(&wallet_address, &token_mint_address);

    let (mut banks_client, payer, recent_blockhash) =
        program_test(token_mint_address, true).start().await;
    let rent = banks_client.get_rent().await.unwrap();
    let expected_token_account_len = Account::LEN;
    let expected_token_account_balance = rent.minimum_balance(expected_token_account_len);

    // Associated account does not exist
    assert_eq!(
        banks_client
            .get_account(associated_token_address)
            .await
            .expect("get_account"),
        None,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[create_associated_token_account(
            &payer.pubkey(),
            &wallet_address,
            &token_mint_address,
            &spl_token::id(),
        )],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await.unwrap();

    // Associated account now exists
    let associated_account = banks_client
        .get_account(associated_token_address)
        .await
        .expect("get_account")
        .expect("associated_account not none");
    assert_eq!(associated_account.data.len(), expected_token_account_len);
    assert_eq!(associated_account.owner, spl_token::id());
    assert_eq!(associated_account.lamports, expected_token_account_balance);
}

#[tokio::test]
async fn success_using_deprecated_instruction_creator() {
    let wallet_address = Pubkey::new_unique();
    let token_mint_address = Pubkey::new_unique();
    let associated_token_address =
        get_associated_token_address(&wallet_address, &token_mint_address);

    let (mut banks_client, payer, recent_blockhash) =
        program_test(token_mint_address, true).start().await;
    let rent = banks_client.get_rent().await.unwrap();
    let expected_token_account_len = Account::LEN;
    let expected_token_account_balance = rent.minimum_balance(expected_token_account_len);

    // Associated account does not exist
    assert_eq!(
        banks_client
            .get_account(associated_token_address)
            .await
            .expect("get_account"),
        None,
    );

    // Use legacy instruction creator
    #[allow(deprecated)]
    let create_associated_token_account_ix = deprecated_create_associated_token_account(
        &payer.pubkey(),
        &wallet_address,
        &token_mint_address,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[create_associated_token_account_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await.unwrap();

    // Associated account now exists
    let associated_account = banks_client
        .get_account(associated_token_address)
        .await
        .expect("get_account")
        .expect("associated_account not none");
    assert_eq!(associated_account.data.len(), expected_token_account_len);
    assert_eq!(associated_account.owner, spl_token::id());
    assert_eq!(associated_account.lamports, expected_token_account_balance);
}

#[tokio::test]
async fn test_basic() {
    let mint_auth = Keypair::new();
    let minter = Keypair::new();

    // minter will transfer tokens to recipient's account
    let alice = Keypair::new();
    let alice_ata = get_associated_token_address_with_program_id(
        &alice.pubkey(),
        &minter.pubkey(),
        &spl_token_2022::id(),
    );

    let pc = ProgramTest::default();
    let (mut banks_client, payer, recent_blockhash) = pc.start().await;

    // Create the mint account
    let create_mint = solana_sdk::system_instruction::create_account(
        &payer.pubkey(),
        &minter.pubkey(),
        100_000_000,
        spl_token_2022::state::Mint::LEN as u64, // Mint account size
        &spl_token_2022::id(),                   // account owner
    );

    // Initialize the mint account
    let initialize_mint = spl_token_2022::instruction::initialize_mint(
        &spl_token_2022::id(),
        &minter.pubkey(),
        &mint_auth.pubkey(),
        None, // No freeze authority
        6,    // Number of decimals (e.g., 6 decimals for fungible tokens)
    )
    .unwrap();

    let create_account = create_associated_token_account(
        &payer.pubkey(),
        &alice.pubkey(),
        &minter.pubkey(),
        &spl_token_2022::id(),
    );

    let initialize_account = spl_token_2022::instruction::initialize_account(
        &spl_token_2022::id(),
        &alice_ata,
        &minter.pubkey(),
        &alice.pubkey(),
    )
    .unwrap();

    // Setup: Create the mint account and initialize it with SPL Token.
    // Mint tokens to the recipient's token account
    let mint_to = spl_token_2022::instruction::mint_to(
        &spl_token_2022::id(),
        &minter.pubkey(),
        &alice_ata,
        &alice.pubkey(),
        &[&mint_auth.pubkey()],
        10_000_000, // Amount to mint (10 tokens, given 6 decimals)
    )
    .unwrap();

    // Combine the create account, initialize mint, and mint_to instructions into one transaction
    let transaction = Transaction::new_signed_with_payer(
        &[
            create_mint,
            initialize_mint,
            create_account,
            // initialize_account,
            // mint_to,
        ],
        Some(&payer.pubkey()),
        &[
            &payer, // &mint_auth,
            &minter,
        ],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await.unwrap();
}
