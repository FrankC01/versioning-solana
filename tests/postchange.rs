use common::{
    get_account_for_key, get_keypair, setup_validator, PROG_KEY, USER1_ACCOUNT, USER2_ACCOUNT,
};
use solana_sdk::{commitment_config::CommitmentConfig, signer::Signer};

mod common;

#[test]
fn test_pre_change_load_pass() {
    let (test_validator, initial_keypair) = setup_validator().start();
    let rpc_client = test_validator.get_rpc_client();
    let cc = CommitmentConfig::confirmed();
    let acc = get_account_for_key(&rpc_client, &PROG_KEY, cc);
    assert!(acc.is_some());
    println!("Default {:?}", initial_keypair.pubkey());
    let user1key = get_keypair(USER1_ACCOUNT).unwrap();
    let user2key = get_keypair(USER2_ACCOUNT).unwrap();
    let u1acc = get_account_for_key(&rpc_client, &user1key.pubkey(), cc).unwrap();
    let u2acc = get_account_for_key(&rpc_client, &user2key.pubkey(), cc).unwrap();
    assert_eq!(u1acc.data[0], 1);
    assert_eq!(u1acc.data[1], 0);
    assert_eq!(u2acc.data[0], 1);
    assert_eq!(u2acc.data[1], 0);
}
