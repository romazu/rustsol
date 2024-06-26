use std::marker::PhantomData;
use std::sync::Arc;
use alloy_primitives::U256;
use derivative::Derivative;
use crate::utils::{bytes32_to_u256, ceil_div, keccak256, u256_to_bytes32, u256_to_u64, vec_u256_to_vec_bytes};
use crate::types::{FromLESlice, Value, Position, SlotsGetter, SlotsGetterSetter};

// In particular: if the data is at most 31 bytes long, the elements are stored in the higher-order
// bytes (left aligned) and the lowest-order byte stores the value length * 2. For byte arrays that
// store data which is 32 or more bytes long, the main slot p stores length * 2 + 1 and the data is
// stored as usual in keccak256(p). This means that you can distinguish a short array from a long
// array by checking if the lowest bit is set: short (not set) and long (set).
#[derive(Derivative, Default)]
#[derivative(Debug)]
pub struct Bytes<NativeType> {
    __slot: U256,
    #[derivative(Debug = "ignore")]
    __slots_getter: Option<Arc<dyn SlotsGetter>>,
    __marker: PhantomData<NativeType>,
}

impl<NativeType: FromLESlice> Bytes<NativeType> {
    pub fn slot(&self) -> U256 {
        self.__slot
    }

    pub fn position(&self) -> (U256, usize, usize) {
        (self.__slot, 0, 32)
    }

    pub fn storage(&self) -> U256 {
        bytes32_to_u256(keccak256(u256_to_bytes32(self.__slot)))
    }

    pub fn get_value(&self) -> Result<<Self as Value>::ValueType, String> {
        let getter = self.__slots_getter.as_ref().expect("No slots getter");
        let slot_values = getter.get_slots(self.__slot, 1)
            .map_err(|err| format!("Failed to get slot values: {}", err))?;
        self.get_value_from_slots_content(slot_values)
    }
}

impl<NativeType> Position for Bytes<NativeType> {
    fn from_position(slot: U256, _: usize) -> Self {
        Bytes { __slot: slot, __slots_getter: None, __marker: PhantomData }
    }

    fn size() -> usize {
        32
    }
}

impl<NativeType> SlotsGetterSetter for Bytes<NativeType> {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slots_getter = Some(getter);
    }
}

impl<NativeType: FromLESlice> Value for Bytes<NativeType> {
    type ValueType = NativeType;

    fn get_value_from_slots_content(&self, slot_values: Vec<U256>) -> Result<Self::ValueType, String> {
        let getter = self.__slots_getter.as_ref().expect("No slots getter");
        let base_slot_value = slot_values[0];
        let is_long = base_slot_value.bit(0);
        if is_long {
            let string_len_bytes = (u256_to_u64(base_slot_value) - 1) / 2;
            let string_len_slots = ceil_div(string_len_bytes as usize, 32);
            let element_slot_values = getter.get_slots(self.storage(), string_len_slots)
                .map_err(|err| format!("Failed to get slot values: {}", err))?;
            let bytes = vec_u256_to_vec_bytes(&element_slot_values, 0, string_len_slots);
            Ok(NativeType::from(&bytes[0..string_len_bytes as usize]))
        } else {
            let string_len = base_slot_value.byte(0) / 2;
            // let bytes = Vec::from(&bytes[0..string_len as usize]);
            let bytes = base_slot_value.to_be_bytes::<{ U256::BYTES }>();
            Ok(NativeType::from(&bytes[0..string_len as usize]))
        }
    }
}
