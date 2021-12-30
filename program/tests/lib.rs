//! test processor framework

use borsh::BorshDeserialize;
use solana_data_versioning::{
    account_state::{ProgramAccountState, ACCOUNT_STATE_SPACE},
    entry_point::process_instruction,
    instruction::ProgramInstruction,
};
use solana_program::{
    hash::Hash,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use solana_program_test::*;
use solana_sdk::{
    account::Account, pubkey, signature::Keypair, signer::Signer, transaction::Transaction,
    transport::TransportError,
};

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
    let result = submit_txn(
        &ProgramInstruction::InitializeAccount,
        &[AccountMeta::new(account_pubkey, false)],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_ok());

    // Verify initialized
    match banks_client.get_account(account_pubkey).await.unwrap() {
        Some(acc) => {
            let acc_deser = ProgramAccountState::try_from_slice(&acc.data).unwrap();
            assert!(acc_deser.initialized());
            assert_eq!(acc_deser.content().somevalue, 1);
            assert_eq!(acc_deser.version(), 0);
        }
        None => panic!(),
    }
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
