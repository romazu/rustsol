use std::str::FromStr;
use std::sync::Arc;
use alloy_primitives::{Address, U256};
use rustsol::types::SlotsGetterSetter;

mod utils;

mod generated_contract_simple;

use crate::generated_contract_simple::MyContract;

fn main() {
    let contract = MyContract::new();
    // println!("{:?}", contract.myNestedMapping.at(0u64)); // Won't compile: incorrect key type.
    println!("{:?}", contract.myNestedMapping.at(U256::from(64)));
    println!("{:?}", contract.myNestedMapping.at(U256::from(64)).at(U256::from(1)).position());
    println!("myMapping2[\"some\"] {:?}", contract.myMapping2.at("some".to_string()).position()); // string key
    println!("myMapping2[vec]    {:?}", contract.myMapping3.at(vec![0x73, 0x6F, 0x6D, 0x65]).position()); // bytes key
    println!("{:?}", contract.dynamicArray.at(10).position());
    println!("{:?}", contract.dynamicArrayStruct.at(10).position());
    println!("{:?}", contract.dynamicArrayStruct.at(11).position());
    println!("{:?}", contract.dynamicArraySmall.at(10).position());
    println!("{:?}", contract.dynamicArraySmall.at(11).position());
    println!("{:?}", contract.staticArrayNestedSmall.position());
    println!("{:?}", contract.staticArrayNestedSmall.at(0).position());
    println!("{:?}", contract.staticArrayNestedSmall.at(1).position());
    println!("{:?}", contract.staticArrayNestedSmall.at(0).at(0).position());
    println!("{:?}", contract.staticArrayNestedSmall.at(0).at(31).position());
    println!("{:?}", contract.staticArrayLarge.at(0).position());
    println!("{:?}", contract.staticArrayLarge.at(1).position());
    println!("{:?}", contract.staticArrayLarge.capacity());
    println!("{:?}", contract.staticArrayLarge.capacity());
    // println!("{:?}", contract.staticArrayLarge.at(2).position()); // panic "Index is outside array capacity: 2 vs 2"
    println!("myMappingBool[false] {:?}", contract.myMappingBool.at(false).position());
    println!("myMappingBool[true]  {:?}", contract.myMappingBool.at(true).position());

    let address = Address::from_str("0x51A18333479472D1250Ee5063910079fc0B9b801").unwrap();
    println!("myAddressMappingNested                {:?}", contract.myAddressMappingNested.position());
    println!("myAddressMappingNested[addr]:         {:?}", contract.myAddressMappingNested.at(address).position());
    println!("myAddressMappingNested[addr][addr]:   {:?}", contract.myAddressMappingNested.at(address).at(address).position());

    let mut contract = MyContract::new();
    let getter = Arc::new(utils::DummySlotsGetter);
    contract.set_slots_getter(getter);

    println!("plainUint112.get_value()              {:?}", contract.plainUint112.get_value());
    println!("myNestedMapping[0][1].get_value()     {:?}", contract.myNestedMapping.at(U256::from(0)).at(U256::from(1)).get_value());
    println!("plainString.get_value()               {:?}", contract.plainString.get_value());
    println!("dynamicArray.position()               {:?}", contract.dynamicArray.position());
    println!("dynamicArray.get_value()              {:?}", contract.dynamicArray.get_value());
    println!("dynamicArrayNested.get_value()        {:?}", contract.dynamicArrayNested.get_value());
    println!("dynamicArrayNested[1].get_value()     {:?}", contract.dynamicArrayNested.at(1).get_value());
    println!("plainAddress.get_value()              {:?}", contract.plainAddress.get_value());
    println!("plainString.position()                {:?}", contract.plainString.position());
    println!("plainString.get_value()               {:?}", contract.plainString.get_value());
    println!("staticArray.position()                {:?}", contract.staticArray.position());
    println!("staticArray.get_value()               {:?}", contract.staticArray.get_value());
    println!("staticArrayNestedSmall.position()     {:?}", contract.staticArrayNestedSmall.position());
    println!("staticArrayNestedSmall.get_value()    {:?}", contract.staticArrayNestedSmall.get_value());
    // Get all the base storage of contract. Very verbose.
    // println!("contract.get_value()                  {:?}", contract.get_value());
}
