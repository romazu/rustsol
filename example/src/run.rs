#![allow(unused_imports)]

use std::str::FromStr;
use std::sync::Arc;
use alloy_primitives::{Address, U256};
use rustsol::types::{SlotsGetter, SlotsGetterSetter};

mod generated_contract;

// Example implementation for a struct
#[derive(Debug)]
struct DummySlotsGetter;
impl SlotsGetter for DummySlotsGetter {
    fn get_slots(&self, start: U256, n: usize) -> Result<Vec<U256>, String> {
        let mut start_ = start;
        if start > U256::from(1000) {
            start_ = U256::from(1);
        }
        let mut res = Vec::with_capacity(n); // U256 is 32 bytes
        for i in 0..n {
            res.push(start_ + U256::from(i));
        }
        Ok(res)
    }
}

fn main() {
    // // Example contract
    let contract = generated_contract::MyContract::new();
    // println!("{:?}", contract.myNestedMapping.at(0u64));
    // println!("{:?}", contract.myNestedMapping.at(0u64).at(1u64).position());
    // println!("myMapping2[\"some\"] {:?}", contract.myMapping2.at("some").position());
    // println!("myMapping2[vec]    {:?}", contract.myMapping2.at(vec![0x73, 0x6F, 0x6D, 0x65]).position());
    // println!("{:?}", contract.dynamicArray.at(10).position());
    // println!("{:?}", contract.dynamicArrayStruct.at(10).position());
    // println!("{:?}", contract.dynamicArrayStruct.at(11).position());
    // println!("{:?}", contract.dynamicArraySmall.at(10).position());
    // println!("{:?}", contract.dynamicArraySmall.at(11).position());
    // println!("{:?}", contract.staticArrayNestedSmall.position());
    // println!("{:?}", contract.staticArrayNestedSmall.at(0).position());
    // println!("{:?}", contract.staticArrayNestedSmall.at(1).position());
    // println!("{:?}", contract.staticArrayNestedSmall.at(0).at(0).position());
    // println!("{:?}", contract.staticArrayNestedSmall.at(0).at(31).position());
    // println!("{:?}", contract.staticArrayLarge.at(0).position());
    // println!("{:?}", contract.staticArrayLarge.at(1).position());
    // println!("{:?}", contract.staticArrayLarge.capacity());
    // println!("{:?}", contract.staticArrayLarge.capacity());
    // // println!("{:?}", contract.staticArrayLarge.at(2).position()); // panic
    // println!("myMappingBool[false] {:?}", contract.myMappingBool.at(false).position());
    // println!("myMappingBool[true] {:?}", contract.myMappingBool.at(true).position());
    //
    let address = Address::from_str("0x51A18333479472D1250Ee5063910079fc0B9b801").unwrap();
    // println!("myAddressMappingNested {:?}", contract.myAddressMappingNested.position());
    // println!("myAddressMappingNested[addr]: {:?}", contract.myAddressMappingNested.at(address).position());
    println!("myAddressMappingNested[addr][addr]: {:?}", contract.myAddressMappingNested.at(address).at(address).position());

    let mut contract = generated_contract::MyContract::new();
    let getter = Arc::new(DummySlotsGetter);
    contract.set_slots_getter(getter);
    println!("plainUint112.get_value()              {:?}", contract.plainUint112.get_value());
    println!("myNestedMapping[0][1].get_value()     {:?}", contract.myNestedMapping.at(0).at(1).get_value());
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
    println!("contract.get_value()                  {:?}", contract.get_value());

    // // // Uniswap V3
    // // let contract = generated_contract::UniswapV3Pool::new();
    // // println!("slot0 {:#?}", contract.slot0);
    // // println!("ticks {:#?}", contract.ticks);
    // // println!("ticks[42u64].initialized {:?}", contract.ticks.at(42u64).initialized.position());
    // // println!("slot0.observationIndex   {:?}", contract.slot0.observationIndex.position());
    // // println!("ticks[0]                 {:?}", contract.ticks.at(0).position());
    // // println!("ticks[149150]            {:?}", contract.ticks.at(149150).position());
    // // println!("ticks[887270]            {:?}", contract.ticks.at(887270).position());
    // // println!("ticks[-92110]            {:?}", contract.ticks.at(-92110).position());
    // // // println!("feeGrowthGlobal0X128.get_value() {:?}", contract.feeGrowthGlobal0X128.get_value()); // panic
    //
    // let contract = generated_contract::UniswapV3Pool::new();
    // let (slot, offset, size_bytes) = contract.observations.at(42).tickCumulative.position();
    // println!("slot={}, offset={}, size_bytes={}", slot, offset, size_bytes);
    //
    // let mut contract = generated_contract::UniswapV3Pool::new();
    // let getter = Arc::new(DummySlotsGetter);
    // contract.set_slots_getter(getter);
    // println!("feeGrowthGlobal0X128.get_value() {:?}", contract.feeGrowthGlobal0X128.get_value());
    // println!("tickBitmap[123].get_value() {:?}", contract.tickBitmap.at(123).get_value());
    // println!("ticks[123].get_value() {:?}", contract.ticks.at(123).get_value());
    // println!("ticks[123].get_value() {:?}", contract.ticks.at(123).get_value());
    
}
