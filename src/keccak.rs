use sha3::{Digest, Keccak256};
use std::convert::TryInto;
use primitive_types::U256;

/// Converts a u64 integer to a 32-byte array
fn to_bytes32(num: u64) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes[24..32].copy_from_slice(&num.to_be_bytes());
    bytes
}

/// Computes the keccak256 hash of the concatenation of two 32-byte arrays
fn keccak256_concat(a: [u8; 32], b: [u8; 32]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(a);
    hasher.update(b);
    hasher.finalize().as_slice().try_into().expect("Wrong length")
}

/// Computes the storage slot for a nested mapping
fn compute_mapping_slot(key: U256, mapping_slot: U256) -> U256 {
    let slot_bytes = keccak256_concat(u256_to_bytes32(key), u256_to_bytes32(mapping_slot));
    bytes32_to_u256(slot_bytes)
}

fn ceil_div(a: u64, b: u64) -> u64 {
    (a + b - 1) / b
}

/// Computes the storage slot and offset for an element in an array
fn compute_array_element_slot(base_slot: U256, index: u64, element_size: u64) -> (U256, u64) {
    if element_size > 32 {
        // Elements larger than 32 bytes, each element starts at a new slot
        let slots_per_element = ceil_div(element_size, 32);
        let slot = base_slot + U256::from(index * slots_per_element);
        (slot, 0)
    } else {
        // Elements smaller than or equal to 32 bytes, packed within slots
        let elements_per_slot = 32 / element_size; // Number of elements per 32-byte slot
        let slot_offset = U256::from(index / elements_per_slot);
        let slot = base_slot + slot_offset;
        let offset = (index % elements_per_slot) * element_size;
        (slot, offset)
    }
}

fn u256_to_bytes32(num: U256) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    num.to_big_endian(&mut bytes);
    bytes
}

fn bytes32_to_u256(bytes: [u8; 32]) -> U256 {
    U256::from_big_endian(&bytes)
}