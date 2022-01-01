use common::{
    clean_ledger_setup_validator, get_account_for_key, get_keypair, set_string_value,
    set_u64_value, setup_validator, USER1_ACCOUNT, USER2_ACCOUNT,
};
use solana_sdk::{commitment_config::CommitmentConfig, signer::Signer};

mod common;

#[test]
fn test_pre_data_change_load_pass() {
    let (test_validator, _initial_keypair) = clean_ledger_setup_validator().unwrap();
    let rpc_client = test_validator.get_rpc_client();
    let cc = CommitmentConfig::confirmed();
    let user1key = get_keypair(USER1_ACCOUNT).unwrap();
    let user2key = get_keypair(USER2_ACCOUNT).unwrap();
    let u1acc = get_account_for_key(&rpc_client, &user1key.pubkey(), cc).unwrap();
    let u2acc = get_account_for_key(&rpc_client, &user2key.pubkey(), cc).unwrap();
    assert_eq!(u1acc.data[0], 1);
    assert_eq!(u1acc.data[1], 0);
    assert_eq!(u1acc.data[2], 25u8);
    assert_eq!(u2acc.data[0], 1);
    assert_eq!(u2acc.data[1], 0);
    assert_eq!(u2acc.data[2], 50u8);

    // let u2acc = set_u64_value(&rpc_client, &initial_keypair, &user2key, 50u64, cc).unwrap();
    // assert_eq!(u2acc.data[2], 50u8);
    // println!("{:?}", u1acc.data)
}
#[test]
fn test_post_data_change_u2_pass() {
    let (test_validator, initial_keypair) = clean_ledger_setup_validator().unwrap();
    let rpc_client = test_validator.get_rpc_client();
    let cc = CommitmentConfig::confirmed();
    let u2keypair = get_keypair(USER2_ACCOUNT).unwrap();
    let u2acc = get_account_for_key(&rpc_client, &u2keypair.pubkey(), cc).unwrap();
    assert_eq!(u2acc.data[0], 1);
    assert_eq!(u2acc.data[1], 0);
    assert_eq!(u2acc.data[2], 50u8);
    // solana_logger::setup_with_default("solana=debug");
    let u2acc = set_string_value(
        &rpc_client,
        &initial_keypair,
        &u2keypair,
        String::from("Hello"),
        cc,
    )
    .unwrap();
    assert_eq!(u2acc.data[0], 1);
    assert_eq!(u2acc.data[1], 1);
    assert_eq!(u2acc.data[2], 50u8);
    println!("Data {:?}", u2acc.data);
}

#[test]
fn test_post_data_change_u1_pass() {
    let (test_validator, initial_keypair) = setup_validator().unwrap();
    let rpc_client = test_validator.get_rpc_client();
    let cc = CommitmentConfig::confirmed();
    let u1keypair = get_keypair(USER1_ACCOUNT).unwrap();
    let u1acc = get_account_for_key(&rpc_client, &u1keypair.pubkey(), cc).unwrap();
    assert_eq!(u1acc.data[0], 1);
    assert_eq!(u1acc.data[1], 0);
    assert_eq!(u1acc.data[2], 50u8);
    // solana_logger::setup_with_default("solana=debug");
    let u1acc = set_u64_value(&rpc_client, &initial_keypair, &u1keypair, 25u64, cc).unwrap();
    assert_eq!(u1acc.data[0], 1);
    assert_eq!(u1acc.data[1], 1);
    assert_eq!(u1acc.data[2], 25u8);
    println!("Data {:?}", u1acc.data);
}

#[test]
fn test_post_data_pass() {
    let (test_validator, _initial_keypair) = setup_validator().unwrap();
    let rpc_client = test_validator.get_rpc_client();
    let cc = CommitmentConfig::confirmed();
    let u1keypair = get_keypair(USER1_ACCOUNT).unwrap();
    let u1acc = get_account_for_key(&rpc_client, &u1keypair.pubkey(), cc).unwrap();
    assert_eq!(u1acc.data[0], 1);
    assert_eq!(u1acc.data[1], 0);
    assert_eq!(u1acc.data[2], 50u8);
    println!("Data {:?}", u1acc.data);
}
