use std::sync::Arc;
use alloy_primitives::{FixedBytes, U256};
use derivative::Derivative;
use crate::utils::{bytes32_to_u256, ceil_div, keccak256, u256_to_bytes32, vec_u256_to_vec_bytes};
use crate::types::{Address, Primitive, Value};
use crate::types::{Position, SlotsGetter, SlotsGetterSetter};

// In particular: if the data is at most 31 bytes long, the elements are stored in the higher-order
// bytes (left aligned) and the lowest-order byte stores the value length * 2. For byte arrays that
// store data which is 32 or more bytes long, the main slot p stores length * 2 + 1 and the data is
// stored as usual in keccak256(p). This means that you can distinguish a short array from a long
// array by checking if the lowest bit is set: short (not set) and long (set).
#[derive(Derivative, Default)]
#[derivative(Debug)]
pub struct Bytes {
    __slot: U256,
    #[derivative(Debug = "ignore")]
    __slot_getter: Option<Arc<dyn SlotsGetter>>,
}

impl Bytes {
    pub fn slot(&self) -> U256 {
        self.__slot
    }

    pub fn position(&self) -> (U256, usize, usize) {
        (self.__slot, 0, 32)
    }

    pub fn storage(&self) -> U256 {
        bytes32_to_u256(keccak256(u256_to_bytes32(self.__slot)))
    }

    pub fn value(self) -> Result<Vec<u8>, String> {
        let getter = self.__slot_getter.as_ref().expect("No slots getter");
        let base_slot_value = getter.get_slots(self.__slot, 1)
            .map_err(|err| format!("Failed to get slot values: {}", err))?[0];
        self.value_from_base_bytes(&base_slot_value.to_be_bytes::<{ U256::BYTES }>())
    }
}

impl Position for Bytes {
    fn from_position(slot: U256, _: usize) -> Self {
        Bytes { __slot: slot, __slot_getter: None }
    }

    fn size() -> usize {
        32
    }
}

impl SlotsGetterSetter for Bytes {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter);
    }
}

impl Value for Bytes {
    type ValueType = Vec<u8>;

    fn value_from_base_bytes(&self, bytes: &[u8]) -> Result<Self::ValueType, String> {
        let getter = self.__slot_getter.as_ref().expect("No slots getter");
        let base_slot_value = U256::from_be_slice(bytes);
        let is_long = base_slot_value.bit(0);
        if is_long {
            let string_len_bytes = (base_slot_value.byte(0) - 1) / 2;
            let string_len_slots = ceil_div(string_len_bytes as usize, 32);
            let slots = getter.get_slots(self.storage(), string_len_slots)
                .map_err(|err| format!("Failed to get slot values: {}", err))?;
            let bytes = vec_u256_to_vec_bytes(&slots, 0, string_len_slots);
            Ok(bytes[0..string_len_bytes as usize].to_vec())
        } else {
            let string_len = base_slot_value.byte(0) / 2;
            // let bytes = Vec::from(&bytes[0..string_len as usize]);
            let bytes = bytes[0..string_len as usize].to_vec();
            Ok(bytes)
        }
    }
}
