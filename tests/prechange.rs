use common::{clean_ledger_setup_validator, get_account_for_key, get_accounts, PROG_KEY};
use solana_sdk::{commitment_config::CommitmentConfig, signer::Signer};

mod common;

#[test]
fn test_load_pass() {
    let (test_validator, initial_keypair) = clean_ledger_setup_validator().start();
    let rpc_client = test_validator.get_rpc_client();
    let cc = CommitmentConfig::confirmed();
    let acc = get_account_for_key(&rpc_client, &PROG_KEY, cc);
    assert!(acc.is_some());
    // solana_logger::setup_with_default("solana=debug");
    let (_, u1acc, _, u2acc) = get_accounts(&rpc_client, &initial_keypair, cc).unwrap();
    assert_eq!(u1acc.data[0], 1);
    assert_eq!(u1acc.data[1], 0);
    assert_eq!(u2acc.data[0], 1);
    assert_eq!(u2acc.data[1], 0);
}