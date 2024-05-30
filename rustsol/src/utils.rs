use sha3::{Digest, Keccak256};
use std::convert::TryInto;
use ethereum_types::{Address, U256};

/// Computes the keccak256 hash of the concatenation of two 32-byte arrays
pub fn keccak256<T: AsRef<[u8]>>(v: T) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(v);
    hasher.finalize().as_slice().try_into().expect("Wrong length")
}

pub fn keccak256_concat(a: [u8; 32], b: [u8; 32]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(a);
    hasher.update(b);
    hasher.finalize().as_slice().try_into().expect("Wrong length")
}

pub fn ceil_div(a: u64, b: u64) -> u64 {
    (a + b - 1) / b
}

pub fn u256_to_bytes32(num: U256) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    num.to_big_endian(&mut bytes);
    bytes
}

pub fn address_to_bytes32(address: Address) -> [u8; 32] {
    let bytes20 = address.to_fixed_bytes();
    let mut bytes32 = [0u8; 32];
    bytes32[12..32].copy_from_slice(&bytes20);
    bytes32
}

pub fn bytes32_to_u256(bytes: [u8; 32]) -> U256 {
    U256::from_big_endian(&bytes)
}

pub fn index_to_position(index: usize, packing_ratio_n: u64, packing_ratio_d: u64) -> (u64, u8) {
    let slot = index as u64 * packing_ratio_n / packing_ratio_d;
    let offset = (index as u64 * 32 * packing_ratio_n / packing_ratio_d % 32) as u8;  // guaranteed to fit in u8
    (slot, offset)
}