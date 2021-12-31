use common::{clean_ledger_setup_validator, get_account_for_key, get_accounts, set_u64_value};
use solana_sdk::{commitment_config::CommitmentConfig, signer::Signer};

mod common;

#[test]
fn test_load_pass() {
    let (test_validator, initial_keypair) = clean_ledger_setup_validator().unwrap();
    let rpc_client = test_validator.get_rpc_client();
    let cc = CommitmentConfig::confirmed();
    let acc = get_account_for_key(&rpc_client, &initial_keypair.pubkey(), cc).unwrap();
    // println!("acc kypair {:?}", initial_keypair);
    println!("acc kypair {:?}", initial_keypair.pubkey());
    println!("acc lamp {}", acc.lamports);
    println!("acc owner {:?}", acc.owner);
    // solana_logger::setup_with_default("solana=debug");
    let (u1keypair, u1acc, _, u2acc) = get_accounts(&rpc_client, &initial_keypair, cc).unwrap();
    assert_eq!(u1acc.data.len(), 1024);
    assert_eq!(u1acc.data[0], 1);
    assert_eq!(u1acc.data[1], 0);
    assert_eq!(u2acc.data.len(), 1024);
    assert_eq!(u2acc.data[0], 1);
    assert_eq!(u2acc.data[1], 0);
    let u1acc = set_u64_value(&rpc_client, &initial_keypair, &u1keypair, 50u64, cc).unwrap();
    assert_eq!(u1acc.data[2], 50u8);
    println!("{:?}", u1acc.data)
}
