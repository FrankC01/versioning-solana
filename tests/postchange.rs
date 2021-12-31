use common::{
    get_account_for_key, get_keypair, set_u64_value, setup_validator, USER1_ACCOUNT, USER2_ACCOUNT,
};
use solana_sdk::{commitment_config::CommitmentConfig, signer::Signer};

mod common;

#[test]
fn test_pre_data_change_load_pass() {
    let (test_validator, initial_keypair) = setup_validator().unwrap();
    let rpc_client = test_validator.get_rpc_client();
    let cc = CommitmentConfig::confirmed();
    let user1key = get_keypair(USER1_ACCOUNT).unwrap();
    let user2key = get_keypair(USER2_ACCOUNT).unwrap();
    let u1acc = get_account_for_key(&rpc_client, &user1key.pubkey(), cc).unwrap();
    let u2acc = get_account_for_key(&rpc_client, &user2key.pubkey(), cc).unwrap();
    assert_eq!(u1acc.data[0], 1);
    assert_eq!(u1acc.data[1], 0);
    assert_eq!(u1acc.data[2], 50u8);
    assert_eq!(u2acc.data[0], 1);
    assert_eq!(u2acc.data[1], 0);
    assert_eq!(u2acc.data[2], 50u8);

    let u2acc = set_u64_value(&rpc_client, &initial_keypair, &user2key, 50u64, cc).unwrap();
    assert_eq!(u2acc.data[2], 50u8);
    println!("{:?}", u1acc.data)
}

#[test]
fn test_post_data_change_pass() {
    let (test_validator, initial_keypair) = setup_validator().unwrap();
    let rpc_client = test_validator.get_rpc_client();
    let cc = CommitmentConfig::confirmed();
    let u1keypair = get_keypair(USER1_ACCOUNT).unwrap();
    let u1acc = get_account_for_key(&rpc_client, &u1keypair.pubkey(), cc).unwrap();
    println!("Account Lamports {:?}", u1acc.lamports);
    // println!("{:?}", u1acc.data());
    assert_eq!(u1acc.data[0], 1);
    assert_eq!(u1acc.data[1], 1);
    assert_eq!(u1acc.data[2], 50u8);
    let acc = get_account_for_key(&rpc_client, &initial_keypair.pubkey(), cc).unwrap();
    println!("Wallet Lamports {:?}", acc.lamports);
    // let u1acc = set_u64_value(&rpc_client, &initial_keypair, &u1keypair, 15u64, cc).unwrap();
    // assert_eq!(u1acc.data[2], 15u8);

    // let u1acc = set_string_value(
    //     &rpc_client,
    //     &initial_keypair,
    //     &u1keypair,
    //     String::from("newwords"),
    //     cc,
    // )
    // .unwrap();
    // assert_eq!(u1acc.data[0], 1);
    // assert_eq!(u1acc.data[1], 1);
    // assert_eq!(u1acc.data[2], 50u8);
}
