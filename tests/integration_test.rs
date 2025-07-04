use serde_json::Value;
use std::fs;

#[test]
fn test_legacy() {
    let raw_transaction_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
    // let json = transaction_decoder::decode(raw_transaction_hex.to_string()).unwrap();
    // let expected = fs::read_to_string("tests/test_legacy_transaction.json").unwrap();
    // assert_eq!(expected, json);
    let json_str = transaction_decoder::decode(raw_transaction_hex.to_string()).unwrap();
    let expected_str = fs::read_to_string("tests/test_legacy_transaction.json").unwrap();

    let json: Value = serde_json::from_str(&json_str).unwrap();
    let expected: Value = serde_json::from_str(&expected_str).unwrap();

    assert_eq!(json, expected);
}

#[test]
fn test_segwit() {
    let raw_transaction_hex = "02000000000101d2467ec855e99689ec0ac5978708c30cf4206e49e30dd81a2377c411cce40f0c0100000000feffffff028f0b1f00000000001600146f048d1381aa546a3e89e87f7549efc45f150b7fa9ce0f0000000000160014d850c02b89821f0f189ca7e81756c102241f7f4002473044022036c03ad8796f865c9348403fb705d5b984a4ef9565e8b0c81a1069f0f36bbeeb022034e9d5679e9783a441586fae034c78c60854ed71b7b53e6ef169e4f58153356101210355dd8af3cbfe5c3d3424b441069455a59ce0c8d5fe628da0913dae55037ef928bff62400";
    // let json = transaction_decoder::decode(raw_transaction_hex.to_string()).unwrap();
    // let expected = fs::read_to_string("tests/test_segwit_transaction.json").unwrap();
    // assert_eq!(expected, json);
    let json_str = transaction_decoder::decode(raw_transaction_hex.to_string()).unwrap();
    let expected_str = fs::read_to_string("tests/test_segwit_transaction.json").unwrap();

    let json: Value = serde_json::from_str(&json_str).unwrap();
    let expected: Value = serde_json::from_str(&expected_str).unwrap();

    assert_eq!(json, expected);
}

