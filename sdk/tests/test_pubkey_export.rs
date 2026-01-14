// Simple test to make sure we haven't broken the re-export of the pubkey macro in trezoa_sdk
#[test]
fn test_sdk_pubkey_export() {
    assert_eq!(
        trezoa_sdk::pubkey!("ZkTokenProof1111111111111111111111111111111"),
        trezoa_pubkey::pubkey!("ZkTokenProof1111111111111111111111111111111")
    );
}
