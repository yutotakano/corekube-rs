use super::*;

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn test_plmn_identity() {
    let plmn_identity = build_plmn_identity(208, 93).0;
    let plmn_identity_expected_bytes: [u8; 3] = [0x02, 0xf8, 0x39];
    assert_eq!(plmn_identity, plmn_identity_expected_bytes.to_vec());
}
