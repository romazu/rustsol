use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;
use alloy_primitives::U256;
use derivative::Derivative;
use crate::types::{DynamicArray, Mapping, Position, SlotsGetter, SlotsGetterSetter, Value};
use crate::utils::{index_to_position, ceil_div, u256_to_u64, vec_u256_to_vec_bytes, u256_to_bytes32};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct StaticArray<const SIZE: usize, ElementType>
    where ElementType: Debug + Value + Position
{
    __slot: U256,
    __marker: PhantomData<ElementType>,
    #[derivative(Debug = "ignore")]
    __slot_getter: Option<Arc<dyn SlotsGetter>>,
}

impl<const SIZE: usize, ElementType: Debug + Position + Value + SlotsGetterSetter> StaticArray<SIZE, ElementType> {
    pub fn slot(&self) -> U256 {
        self.__slot
    }

    pub fn position(&self) -> (U256, usize, usize) {
        (self.__slot, 0, SIZE)
    }

    pub fn storage(&self) -> U256 {
        self.__slot
    }

    // Return the packing ratio: (n, d).
    // This means that packing is "n slot per d elements"
    // In the current solidity implementation one element of the pair is always one.
    pub fn packing_ratio(&self) -> (usize, usize) {
        let element_size = ElementType::size();
        if element_size > 32 {
            (ceil_div(element_size, 32), 1)
        } else {
            (1, 32 / element_size)
        }
    }

    pub fn capacity(&self) -> usize {
        let (packing_n, packing_d) = self.packing_ratio();
        let capacity = SIZE / 32 * packing_d / packing_n;
        capacity as usize
    }

    pub fn value(&self) -> Result<Vec<<ElementType as Value>::ValueType>, String> {
        let getter = self.__slot_getter.as_ref().expect("No slots getter");
        let array_size_slots = SIZE / 32;
        let slot_values = getter.get_slots(self.__slot, array_size_slots as usize)
            .map_err(|err| format!("Failed to get slot values: {}", err))?;
        self.value_from_slots(slot_values)
    }
}

impl<const SIZE: usize, ElementType: Debug + Value + Position> Position for StaticArray<SIZE, ElementType> {
    fn from_position(slot: U256, _: usize) -> Self {
        StaticArray::<SIZE, ElementType> { __slot: slot, __marker: PhantomData, __slot_getter: None }
    }

    fn size() -> usize {
        SIZE
    }
}

impl<const SIZE: usize, ElementType: Debug + Value + Position + SlotsGetterSetter> StaticArray<SIZE, ElementType> {
    fn new_element(&self, slot: U256, offset: usize) -> ElementType
        where ElementType: Debug + Position + Value + SlotsGetterSetter,
    {
        let mut element = ElementType::from_position(slot, offset);
        match &self.__slot_getter {
            None => {
                // No slots getter to pass to children.
            }
            Some(getter) => {
                // Set child's slots getter.
                element.set_slots_getter(getter.clone());
            }
        }
        element
    }

    pub fn get(&self, index: usize) -> ElementType
        where
            ElementType: Position,
    {
        let (packing_n, packing_d) = self.packing_ratio();
        let capacity = SIZE / 32 * packing_d / packing_n;
        if index >= capacity as usize {
            panic!("Index is outside array capacity: {} vs {}", index, capacity)
        }
        let (index_slot, index_offset) = index_to_position(index, packing_n, packing_d);
        self.new_element(self.storage() + index_slot, index_offset)
    }
}

impl<const SIZE: usize, ElementType: Debug + Value + Position> SlotsGetterSetter for StaticArray<SIZE, ElementType> {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter);
    }
}

impl<const SIZE: usize, ElementType: Debug + Position + Value + SlotsGetterSetter> Value for StaticArray<SIZE, ElementType> {
    type ValueType = Vec<<ElementType as Value>::ValueType>;

    fn value_from_slots(&self, slot_values: Vec<U256>) -> Result<Self::ValueType, String> {
        let (packing_n, packing_d) = self.packing_ratio();
        let capacity = SIZE / 32 * packing_d / packing_n; // >= array_len
        let mut values = Vec::new();
        let storage_slot = self.storage();
        for i in 0..capacity as usize {
            // Simple assumption (holds in Solidity) that element occupying several slots cannot have offset.
            // TODO: Write general element_size_slots estimator.
            let element_size_slots = packing_n;
            let (element_slot, element_offset) = index_to_position(i, packing_n, packing_d);
            let element = self.new_element(storage_slot + element_slot, element_offset);
            let start = u256_to_u64(element_slot) as usize;
            let end = start + element_size_slots;
            let element_slot_values = slot_values[start..end].to_vec();
            let value = element.value_from_slots(element_slot_values)?;
            values.push(value);
        }
        Ok(values)
    }
}
