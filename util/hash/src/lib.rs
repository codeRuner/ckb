pub use blake2_rfc::blake2b::Blake2b;
pub use tiny_keccak::Keccak as Sha3;

pub fn sha3_256<T: AsRef<[u8]>>(s: T) -> [u8; 32] {
    tiny_keccak::sha3_256(s.as_ref())
}

pub fn blake2b<T: AsRef<[u8]>>(s: T) -> [u8; 32] {
    let mut result = [0u8; 32];
    let hash = blake2_rfc::blake2b::blake2b(32, &[], s.as_ref());
    result.copy_from_slice(hash.as_bytes());
    result
}

#[test]
fn empty_sha3_256() {
    let actual = sha3_256([]);

    let expected = vec![
        0xa7, 0xff, 0xc6, 0xf8, 0xbf, 0x1e, 0xd7, 0x66, 0x51, 0xc1, 0x47, 0x56, 0xa0, 0x61, 0xd6,
        0x62, 0xf5, 0x80, 0xff, 0x4d, 0xe4, 0x3b, 0x49, 0xfa, 0x82, 0xd8, 0x0a, 0x4b, 0x80, 0xf8,
        0x43, 0x4a,
    ];

    let ref_ex: &[u8] = &expected;
    assert_eq!(&actual, ref_ex);
}

#[test]
fn string_sha3_256() {
    let actual = sha3_256(b"hello");

    let expected = vec![
        0x33, 0x38, 0xbe, 0x69, 0x4f, 0x50, 0xc5, 0xf3, 0x38, 0x81, 0x49, 0x86, 0xcd, 0xf0, 0x68,
        0x64, 0x53, 0xa8, 0x88, 0xb8, 0x4f, 0x42, 0x4d, 0x79, 0x2a, 0xf4, 0xb9, 0x20, 0x23, 0x98,
        0xf3, 0x92,
    ];

    let ref_ex: &[u8] = &expected;
    assert_eq!(&actual, ref_ex);
}
