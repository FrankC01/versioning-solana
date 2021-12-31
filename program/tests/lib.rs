//! test processor framework

use solana_data_versioning::{
    account_state::ACCOUNT_STATE_SPACE, entry_point::process_instruction,
    instruction::ProgramInstruction,
};
use solana_program::{
    hash::Hash,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use solana_program_test::{
    processor,
    tokio::{self},
    BanksClient, ProgramTest,
};
use solana_sdk::{
    account::Account, pubkey, signature::Keypair, signer::Signer, transaction::Transaction,
    transport::TransportError,
};
use std::time::Duration;

const PROGRAM_ID: Pubkey = pubkey!("PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc");

/// Sets up the Program test and initializes 'n' program_accounts
async fn setup(program_accounts: &[Pubkey]) -> (BanksClient, Keypair, Hash) {
    // std::env::set_var("BPF_OUT_DIR", "target/deploy/");
    let mut program_test = ProgramTest::new(
        "solana_data_versioning", // Run the BPF version with `cargo test-bpf`
        PROGRAM_ID,
        processor!(process_instruction), // Run the native version with `cargo test`
    );
    solana_logger::setup_with("solana_program_test=debug");
    for account in program_accounts {
        program_test.add_account(
            *account,
            Account {
                lamports: 5,
                data: vec![0_u8; ACCOUNT_STATE_SPACE],
                owner: PROGRAM_ID,
                ..Account::default()
            },
        );
    }
    program_test.start().await
}

/// Submit transaction with relevant instruction data
#[allow(clippy::ptr_arg)]
async fn submit_txn(
    instruction_data: &ProgramInstruction,
    accounts: &[AccountMeta],
    payer: &dyn Signer,
    recent_blockhash: Hash,
    banks_client: &mut BanksClient,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_borsh(
            PROGRAM_ID,
            instruction_data,
            accounts.to_vec(),
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer], recent_blockhash);
    banks_client.process_transaction(transaction).await
}

#[tokio::test]
/// Validates initialization processing
async fn test_initialize_prechange_pass() {
    // Setup runtime testing and accounts
    let account_pubkey = Pubkey::new_unique();
    let (mut banks_client, payer, recent_blockhash) = setup(&[account_pubkey]).await;

    // Verify account is not yet initialized
    let is_initialized = match banks_client.get_account(account_pubkey).await.unwrap() {
        Some(account) => {
            if account.data[0] == 0 {
                false
            } else {
                true
            }
        }
        None => true,
    };
    assert!(is_initialized == false);

    // Initialize account
    println!("Recent BH {:?}", recent_blockhash);
    let macc = [AccountMeta::new(account_pubkey, false)];
    let result = submit_txn(
        &ProgramInstruction::InitializeAccount,
        &macc,
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_ok());

    // Verify initialized
    let acc = banks_client
        .get_account(account_pubkey)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(acc.data[0], 1);
    assert_eq!(acc.data[1], 0);
    assert_eq!(acc.data[2], 1);

    // Wait for new blockhash
    tokio::time::sleep(Duration::from_millis(500)).await;
    let new_blockhash = banks_client.get_latest_blockhash().await.unwrap();
    println!("New BH {:?}", new_blockhash);
    // Initialize account twice fail
    let bad_result = submit_txn(
        &ProgramInstruction::InitializeAccount,
        &macc,
        &payer,
        new_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(bad_result.is_err());

    // Wait for new blockhash
    tokio::time::sleep(Duration::from_millis(500)).await;
    let new2_blockhash = banks_client.get_latest_blockhash().await.unwrap();
    println!("New2 BH {:?}", new2_blockhash);
    // Initialize account twice fail
    let set_result = submit_txn(
        &ProgramInstruction::SetU64Value(50u64),
        &macc,
        &payer,
        new2_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(set_result.is_ok());
    // Verify set
    let acc = banks_client
        .get_account(account_pubkey)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(acc.data[0], 1);
    assert_eq!(acc.data[1], 0);
    assert_eq!(acc.data[2], 50u8);
}

#[tokio::test]
/// Validates unknown instruction processing
async fn test_unknown_instruction_error_pass() {
    // Setup runtime testing and accounts
    let account_pubkey = Pubkey::new_unique();
    let (mut banks_client, payer, recent_blockhash) = setup(&[account_pubkey]).await;
    // Initialize account
    let result = submit_txn(
        &ProgramInstruction::FailInstruction,
        &[AccountMeta::new(account_pubkey, false)],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_err());
}
