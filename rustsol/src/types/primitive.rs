use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;
use alloy_primitives::{I256, U256};
use crate::types::{Mapping, SlotsGetterSetter};
use crate::types::traits::{Position, SlotsGetter, Value};
use derivative::Derivative;
use crate::utils::vec_u256_to_vec_bytes;

#[derive(Derivative, Default)]
#[derivative(Debug)]
pub struct Primitive<const SIZE: usize, NativeType> {
    __slot: U256,
    __offset: usize,
    #[derivative(Debug = "ignore")]
    __slots_getter: Option<Arc<dyn SlotsGetter>>,
    __marker: PhantomData<NativeType>,
}

impl<const SIZE: usize, NativeType: FromLESlice> Primitive<SIZE, NativeType> {
    pub fn slot(&self) -> U256 {
        self.__slot
    }

    pub fn offset(&self) -> usize {
        self.__offset
    }

    pub fn position(&self) -> (U256, usize, usize) {
        (self.__slot, self.__offset, SIZE)
    }

    pub fn get_value(&self) -> Result<<Self as Value>::ValueType, String> {
        let getter = self.__slots_getter.as_ref().expect("No slots getter");
        let slot_values = getter.get_slots(self.__slot, 1)
            .map_err(|err| format!("Failed to get slot values: {}", err))?;
        self.get_value_from_slots_content(slot_values)
    }
}

impl<const SIZE: usize, NativeType> Position for Primitive<SIZE, NativeType> {
    fn from_position(slot: U256, offset: usize) -> Self {
        Primitive { __slot: slot, __offset: offset, __slots_getter: None, __marker: PhantomData }
    }

    fn size() -> usize {
        SIZE
    }
}

impl<const SIZE: usize, NativeType> SlotsGetterSetter for Primitive<SIZE, NativeType> {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slots_getter = Some(getter);
    }
}

impl<const SIZE: usize, NativeType: FromLESlice> Value for Primitive<SIZE, NativeType> {
    type ValueType = NativeType;

    fn get_value_from_slots_content(&self, slot_values: Vec<U256>) -> Result<Self::ValueType, String> {
        let bytes = &slot_values[0].to_le_bytes::<{ U256::BYTES }>()[self.__offset..self.__offset + SIZE];
        Ok(NativeType::from(bytes))
    }
}

pub trait FromLESlice {
    fn from(bytes: &[u8]) -> Self;
}

macro_rules! from_le_slice_impl_signed {
    ($($t:ty),+) => {
        $(
            impl FromLESlice for $t {
                fn from(bytes: &[u8]) -> Self {
                    // Check MSB.
                    let is_negative = bytes[bytes.len() - 1] & (1 << 7) != 0;
                    const TYPE_SIZE: usize = (<$t>::BITS / 8) as usize;
                    let mut bytes_sized = if is_negative {
                        [0xFF; TYPE_SIZE]
                    } else {
                        [0u8; TYPE_SIZE]
                    };
                    bytes_sized[0..bytes.len()].copy_from_slice(bytes);
                    Self::from_le_bytes(bytes_sized)
                }
            }
        )+
    };
}

macro_rules! from_le_slice_impl_unsigned {
    ($($t:ty),+) => {
        $(
            impl FromLESlice for $t {
                fn from(bytes: &[u8]) -> Self {
                    const TYPE_SIZE: usize = (<$t>::BITS / 8) as usize;
                    let mut bytes_sized = [0u8; TYPE_SIZE];
                    bytes_sized[0..bytes.len()].copy_from_slice(bytes);
                    Self::from_le_bytes(bytes_sized.try_into().unwrap())
                }
            }
        )+
    };
}

from_le_slice_impl_signed!(i8, i16, i32, i64, i128, I256);
from_le_slice_impl_unsigned!(u8, u16, u32, u64, u128, U256);
impl FromLESlice for bool {
    fn from(bytes: &[u8]) -> Self {
        bytes[0] == 1
    }
}
