use sha3::{Digest, Keccak256};
use std::convert::TryInto;
use std::{u64, usize};
use alloy_primitives::{Address, U256};

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

pub fn bool_to_bytes32(value: bool) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    if value {
        bytes[31] = 1;
    }
    bytes
}

pub fn u256_to_bytes32(num: U256) -> [u8; 32] {
    num.to_be_bytes()
}

pub fn address_to_bytes32(address: Address) -> [u8; 32] {
    let bytes20 = address.as_slice();
    let mut bytes32 = [0u8; 32];
    bytes32[12..32].copy_from_slice(&bytes20);
    bytes32
}

pub fn bytes32_to_u256(bytes: [u8; 32]) -> U256 {
    U256::from_be_bytes(bytes)
}

pub fn u256_to_u64(value: U256) -> u64 {
    let bytes = value.to_be_bytes::<{ U256::BYTES }>(); // Assuming it returns [u8; 32]

    // Check if the upper 24 bytes are all zeros, which means it fits into u64
    // TODO: Use the correct byte size of usize on a machine.
    if bytes[..24] == [0u8; 24] {
        let lower_u64 = u64::from_be_bytes(bytes[24..32].try_into().expect("slice with incorrect length"));
        lower_u64
    } else {
        panic!("Value does not fit in u64")
    }
}

pub fn vec_u256_to_vec_bytes(vec_u256: &Vec<U256>, start: usize, end: usize) -> Vec<u8> {
    let mut vec_bytes = Vec::with_capacity((end - start) * 32); // U256 is 32 bytes
    for u256 in &vec_u256[start..end] {
        vec_bytes.extend_from_slice(&u256.to_be_bytes::<{ U256::BYTES }>());
    }
    vec_bytes
}

pub fn index_to_position(index: usize, packing_ratio_n: u64, packing_ratio_d: u64) -> (u64, u8) {
    let slot = index as u64 * packing_ratio_n / packing_ratio_d;
    let offset = (index as u64 * 32 * packing_ratio_n / packing_ratio_d % 32) as u8;  // guaranteed to fit in u8
    (slot, offset)
}
