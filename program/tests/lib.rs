//! test processor framework

use borsh::{BorshDeserialize, BorshSerialize};
use solana_data_versioning::{
    account_state::{ProgramAccountState, ACCOUNT_STATE_SPACE},
    entry_point::process_instruction,
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

#[cfg(test)]
mod tests {
    use borsh::BorshSerialize;
    use solana_data_versioning::account_state::{
        AccountContentCurrent, ProgramAccountState, ACCOUNT_STATE_SPACE, CURRENT_VERSION_DATA_SIZE,
        INTERMMEDIATE_SIZE,
    };
    use solana_program::pubkey::Pubkey;

    #[test]
    fn test_size() {
        println!("Intermmediate size {}", INTERMMEDIATE_SIZE);
        println!("Content size {}", CURRENT_VERSION_DATA_SIZE);
        println!("Total space size {}", ACCOUNT_STATE_SPACE);
        let x = ProgramAccountState {
            is_initialized: true,
            data_version: 0,
            account_data: AccountContentCurrent {
                somekey: Pubkey::new_unique(),
            },
        };
        let x_ser = x.account_data.try_to_vec().unwrap();
        println!("{:?}", x_ser);
    }
}

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

#[derive(BorshSerialize, Debug)]
enum InstructionPayload {
    Initialize,
    FailInstruction,
}

/// Submit transaction with relevant instruction data
#[allow(clippy::ptr_arg)]
async fn submit_txn(
    instruction_data: &InstructionPayload,
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
async fn test_initialize_pass() {
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
        &InstructionPayload::Initialize,
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
            assert!(acc_deser.is_initialized);
            assert_eq!(acc_deser.get_content().somekey, account_pubkey);
            assert_eq!(acc_deser.data_version, 0);
        }
        None => panic!(),
    }
}

#[tokio::test]
/// Validates unknown instruction processing
async fn test_unknown_instruction_pass() {
    // Setup runtime testing and accounts
    let account_pubkey = Pubkey::new_unique();
    let (mut banks_client, payer, recent_blockhash) = setup(&[account_pubkey]).await;
    // Initialize account
    let result = submit_txn(
        &InstructionPayload::FailInstruction,
        &[AccountMeta::new(account_pubkey, false)],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_err());
}
