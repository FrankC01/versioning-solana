//! Common references

use solana_client::rpc_client::RpcClient;
use solana_data_versioning::{
    account_state::ACCOUNT_STATE_SPACE, instruction::VersionProgramInstruction,
};

use solana_program::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    system_instruction,
};
use solana_sdk::{
    account::Account,
    commitment_config::CommitmentConfig,
    pubkey,
    signature::{read_keypair_file, Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};
use solana_streamer::socket::SocketAddrSpace;
use solana_validator::test_validator::{TestValidator, TestValidatorGenesis};
use std::{
    error,
    path::{Path, PathBuf},
    str::FromStr,
};

/// Test validator information
const LEDGER_PATH: &str = "./.ledger";
const PROG_PATH: &str = "target/deploy/";
const PROG_NAME: &str = "solana_data_versioning";
pub const PROG_KEY: Pubkey = pubkey!("PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc");

/// Sample key information
const KEY_ACCOUNTS_BASE_PATH: &str = "./keys/accounts";
pub const USER1_ACCOUNT: &str = "user1_account.json";
pub const USER2_ACCOUNT: &str = "user2_account.json";
const WALLET_ACCOUNT: &str = "version_wallet.json";

/// Loads a keypair from path provided
pub fn get_keypair(keyname: &str) -> Result<Keypair, Box<dyn error::Error>> {
    let path = Path::new(KEY_ACCOUNTS_BASE_PATH).join(keyname);
    match read_keypair_file(&path) {
        Err(e) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("could not read keypair file \"{}\". Run \"solana-keygen new\" to create a keypair file: {}",
                path.display(), e),
        )
        .into()),
        Ok(kp) => Ok(kp),
    }
}

/// Setup the test validator with predefined properties
pub fn setup_validator() -> Result<(TestValidator, Keypair), Box<dyn error::Error>> {
    let vwallet = get_keypair(WALLET_ACCOUNT).unwrap();
    std::env::set_var("BPF_OUT_DIR", PROG_PATH);
    let mut test_validator = TestValidatorGenesis::default();
    test_validator.ledger_path(LEDGER_PATH);
    test_validator.add_program(PROG_NAME, PROG_KEY);
    // solana_logger::setup_with_default("solana=error");
    let test_validator =
        test_validator.start_with_mint_address(vwallet.pubkey(), SocketAddrSpace::new(true))?;
    Ok((test_validator, vwallet))
}

/// Ensures an empty ledger before setting up the validator
pub fn clean_ledger_setup_validator() -> Result<(TestValidator, Keypair), Box<dyn error::Error>> {
    if PathBuf::from_str(LEDGER_PATH).unwrap().exists() {
        std::fs::remove_dir_all(LEDGER_PATH).unwrap();
    }
    setup_validator()
}

/// Checks for existence of account
pub fn get_account_for_key(
    rpc_client: &RpcClient,
    key: &Pubkey,
    commitment_config: CommitmentConfig,
) -> Option<Account> {
    rpc_client
        .get_account_with_commitment(key, commitment_config)
        .unwrap()
        .value
}

/// Submits the program instruction as per the
/// instruction definition
fn submit_transaction(
    rpc_client: &RpcClient,
    wallet_signer: &dyn Signer,
    instruction: Instruction,
    commitment_config: CommitmentConfig,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let mut transaction =
        Transaction::new_unsigned(Message::new(&[instruction], Some(&wallet_signer.pubkey())));
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .map_err(|err| format!("error: unable to get recent blockhash: {}", err))?;
    transaction
        .try_sign(&vec![wallet_signer], recent_blockhash)
        .map_err(|err| format!("error: failed to sign transaction: {}", err))?;
    let signature = rpc_client
        .send_and_confirm_transaction_with_spinner_and_commitment(&transaction, commitment_config)
        .map_err(|err| format!("error: send transaction: {}", err))?;
    Ok(signature)
}

/// Set a well know field on the account
pub fn set_u64_value(
    rpc_client: &RpcClient,
    wallet_signer: &dyn Signer,
    account_pair: &dyn Signer,
    value: u64,
    cc: CommitmentConfig,
) -> Result<Account, Box<dyn std::error::Error>> {
    let accounts = &[AccountMeta::new(account_pair.pubkey(), false)];

    let instruction = Instruction::new_with_borsh(
        PROG_KEY,
        &VersionProgramInstruction::SetU64Value(value),
        accounts.to_vec(),
    );
    submit_transaction(rpc_client, wallet_signer, instruction, cc)?;

    Ok(rpc_client
        .get_account_with_commitment(&account_pair.pubkey(), cc)
        .map_err(|err| format!("error: getting account after initialization: {}", err))
        .unwrap()
        .value
        .unwrap())
}

/// Set a well know field on the account
pub fn set_string_value(
    rpc_client: &RpcClient,
    wallet_signer: &dyn Signer,
    account_pair: &dyn Signer,
    value: String,
    cc: CommitmentConfig,
) -> Result<Account, Box<dyn std::error::Error>> {
    let accounts = &[AccountMeta::new(account_pair.pubkey(), false)];

    let instruction = Instruction::new_with_borsh(
        PROG_KEY,
        &VersionProgramInstruction::SetString(value),
        accounts.to_vec(),
    );
    submit_transaction(rpc_client, wallet_signer, instruction, cc)?;

    Ok(rpc_client
        .get_account_with_commitment(&account_pair.pubkey(), cc)
        .map_err(|err| format!("error: getting account after initialization: {}", err))
        .unwrap()
        .value
        .unwrap())
}

/// Create a new program account with account state data allocation
fn new_account(
    rpc_client: &RpcClient,
    wallet_signer: &dyn Signer,
    account_pair: &dyn Signer,
    program_owner: &Pubkey,
    state_space: u64,
    commitment_config: CommitmentConfig,
) -> Result<Account, Box<dyn std::error::Error>> {
    let account_lamports = rpc_client
        .get_minimum_balance_for_rent_exemption(state_space as usize)
        .unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &wallet_signer.pubkey(),
                &account_pair.pubkey(),
                account_lamports,
                state_space,
                program_owner,
            ),
            Instruction::new_with_borsh(
                *program_owner,
                &VersionProgramInstruction::InitializeAccount,
                vec![
                    AccountMeta::new(account_pair.pubkey(), false),
                    AccountMeta::new(wallet_signer.pubkey(), true),
                ],
            ),
        ],
        Some(&wallet_signer.pubkey()),
    );

    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .map_err(|err| format!("error: unable to get recent blockhash: {}", err))
        .unwrap();
    transaction
        .try_sign(&vec![wallet_signer, account_pair], recent_blockhash)
        .map_err(|err| format!("error: failed to sign transaction: {}", err))
        .unwrap();
    let _signature = rpc_client
        .send_and_confirm_transaction_with_spinner_and_commitment(&transaction, commitment_config)
        .map_err(|err| format!("error: send transaction: {}", err))
        .unwrap();
    let account = rpc_client
        .get_account_with_commitment(&account_pair.pubkey(), commitment_config)
        .map_err(|err| format!("error: getting account after initialization: {}", err))
        .unwrap()
        .value
        .unwrap();
    Ok(account)
}

pub fn get_accounts(
    rpc_client: &RpcClient,
    wallet_signer: &dyn Signer,
    cc: CommitmentConfig,
) -> Result<(Keypair, Account, Keypair, Account), Box<dyn error::Error>> {
    let user1 = get_keypair(USER1_ACCOUNT)?;
    let user2 = get_keypair(USER2_ACCOUNT)?;
    println!("acc size {:?}", ACCOUNT_STATE_SPACE);
    let u1acc = match get_account_for_key(rpc_client, &user1.pubkey(), cc) {
        Some(acc) => acc,
        None => new_account(
            rpc_client,
            wallet_signer,
            &user1,
            &PROG_KEY,
            ACCOUNT_STATE_SPACE as u64,
            cc,
        )?,
    };
    let u2acc = match get_account_for_key(rpc_client, &user2.pubkey(), cc) {
        Some(acc) => acc,
        None => new_account(
            rpc_client,
            wallet_signer,
            &user2,
            &PROG_KEY,
            ACCOUNT_STATE_SPACE as u64,
            cc,
        )?,
    };

    Ok((user1, u1acc, user2, u2acc))
}
