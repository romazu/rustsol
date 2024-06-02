#![allow(unused_imports)]

use std::str::FromStr;
use std::sync::Arc;
use alloy_primitives::{Address, U256};
use rustsol::types::SlotsGetter;

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
    // let contract = generated_contract::MyContract::new();
    // println!("{:?}", contract.myNestedMapping.get(0u64));
    // println!("{:?}", contract.myNestedMapping.get(0u64).get(1u64).position());
    // println!("myMapping2[\"some\"] {:?}", contract.myMapping2.get("some").position());
    // println!("myMapping2[vec]    {:?}", contract.myMapping2.get(vec![0x73, 0x6F, 0x6D, 0x65]).position());
    // println!("{:?}", contract.dynamicArray.get(10).position());
    // println!("{:?}", contract.dynamicArrayStruct.get(10).position());
    // println!("{:?}", contract.dynamicArrayStruct.get(11).position());
    // println!("{:?}", contract.dynamicArraySmall.get(10).position());
    // println!("{:?}", contract.dynamicArraySmall.get(11).position());
    // println!("{:?}", contract.staticArrayNestedSmall.position());
    // println!("{:?}", contract.staticArrayNestedSmall.get(0).position());
    // println!("{:?}", contract.staticArrayNestedSmall.get(1).position());
    // println!("{:?}", contract.staticArrayNestedSmall.get(0).get(0).position());
    // println!("{:?}", contract.staticArrayNestedSmall.get(0).get(31).position());
    // println!("{:?}", contract.staticArrayLarge.get(0).position());
    // println!("{:?}", contract.staticArrayLarge.get(1).position());
    // println!("{:?}", contract.staticArrayLarge.capacity());
    // println!("{:?}", contract.staticArrayLarge.capacity());
    // // println!("{:?}", contract.staticArrayLarge.get(2).position()); // panic
    // println!("myMappingBool[false] {:?}", contract.myMappingBool.get(false).position());
    // println!("myMappingBool[true] {:?}", contract.myMappingBool.get(true).position());
    //
    // let address = Address::from_str("0x51A18333479472D1250Ee5063910079fc0B9b801").unwrap();
    // println!("myAddressMappingNested {:?}", contract.myAddressMappingNested.position());
    // println!("myAddressMappingNested[addr]: {:?}", contract.myAddressMappingNested.get(address).position());
    // println!("myAddressMappingNested[addr][addr]: {:?}", contract.myAddressMappingNested.get(address).get(address).position());

    let mut contract = generated_contract::MyContract::new();
    let getter = Arc::new(DummySlotsGetter);
    contract.set_slots_getter(getter);
    println!("plainUint112.value()              {:?}", contract.plainUint112.value());
    println!("myNestedMapping[0][1].value()     {:?}", contract.myNestedMapping.get(0).get(1).value());
    // println!("plainString.value()               {:?}", contract.plainString.value());
    println!("dynamicArray.position()           {:?}", contract.dynamicArray.position());
    println!("dynamicArray.value()              {:?}", contract.dynamicArray.value());
    println!("dynamicArrayNested.value()        {:?}", contract.dynamicArrayNested.value());
    println!("dynamicArrayNested[1].value()     {:?}", contract.dynamicArrayNested.get(1).value());
    println!("plainAddress.value()              {:?}", contract.plainAddress.value());
    println!("plainString.position()            {:?}", contract.plainString.position());
    println!("plainString.value()               {:?}", contract.plainString.value());
    println!("staticArray.position()            {:?}", contract.staticArray.position());
    println!("staticArray.value()               {:?}", contract.staticArray.value());
    println!("staticArrayNestedSmall.position() {:?}", contract.staticArrayNestedSmall.position());
    println!("staticArrayNestedSmall.value()    {:?}", contract.staticArrayNestedSmall.value());

    // // // Uniswap V3
    // // let contract = generated_contract::UniswapV3Pool::new();
    // // println!("slot0 {:#?}", contract.slot0);
    // // println!("ticks {:#?}", contract.ticks);
    // // println!("ticks[42u64].initialized {:?}", contract.ticks.get(42u64).initialized.position());
    // // println!("slot0.observationIndex   {:?}", contract.slot0.observationIndex.position());
    // // println!("ticks[0]                 {:?}", contract.ticks.get(0).position());
    // // println!("ticks[149150]            {:?}", contract.ticks.get(149150).position());
    // // println!("ticks[887270]            {:?}", contract.ticks.get(887270).position());
    // // println!("ticks[-92110]            {:?}", contract.ticks.get(-92110).position());
    // // // println!("feeGrowthGlobal0X128.value() {:?}", contract.feeGrowthGlobal0X128.value()); // panic
    //
    // let contract = generated_contract::UniswapV3Pool::new();
    // let (slot, offset, size_bytes) = contract.observations.get(42).tickCumulative.position();
    // println!("slot={}, offset={}, size_bytes={}", slot, offset, size_bytes);
    //
    // let mut contract = generated_contract::UniswapV3Pool::new();
    // let getter = Arc::new(DummySlotsGetter);
    // contract.set_slots_getter(getter);
    // println!("feeGrowthGlobal0X128.value() {:?}", contract.feeGrowthGlobal0X128.value());
    // println!("tickBitmap[123].value() {:?}", contract.tickBitmap.get(123).value());
    // println!("ticks[123].value() {:?}", contract.ticks.get(123).value());
    // println!("ticks[123].value() {:?}", contract.ticks.get(123).value());




    // // Uniswap V2
    // TODO: let contract = ...
    // println!("totalSupply {:?}", contract.totalSupply.position());
    // println!("balanceOf {:?}", contract.balanceOf.position());
    // println!("allowance {:?}", contract.allowance.position());
    // println!("DOMAIN_SEPARATOR {:?}", contract.DOMAIN_SEPARATOR.position());
    // println!("nonces {:?}", contract.nonces.position());
    // println!("factory {:?}", contract.factory.position());
    // println!("token0 {:?}", contract.token0.position());
    // println!("token1 {:?}", contract.token1.position());
    // println!("reserve0 {:?}", contract.reserve0.position());
    // println!("reserve1 {:?}", contract.reserve1.position());
    // println!("blockTimestampLast {:?}", contract.blockTimestampLast.position());
    // println!("price0CumulativeLast {:?}", contract.price0CumulativeLast.position());
    // println!("price1CumulativeLast {:?}", contract.price1CumulativeLast.position());
    // println!("kLast {:?}", contract.kLast.position());
    // println!("unlocked {:?}", contract.unlocked.position());

    // // AccessControlledOffchainAggregator ETH / USD
    // TODO: let contract = ...
    // println!("owner {:?}", contract.owner.position());
    // println!("s_description {:?}, {:?}", contract.s_description.position(), contract.s_description.storage());
    // println!("s_transmitters {:?}", contract.s_transmitters.position());
    // println!("s_transmitters[0] {:?}", contract.s_transmitters.get(0).position());
    // println!("s_transmitters[1] {:?}", contract.s_transmitters.get(1).position());
    // println!("s_signers[0] {:?}", contract.s_signers.get(0).position());

    // // Augur 2
    // TODO: let contract = ...
    // let address = U256::from_str("0x51A18333479472D1250Ee5063910079fc0B9b801").unwrap();
    // println!("{:#x}", address);
    // println!("marketCreationData {:?}", contract.marketCreationData.get(address).position());
    // println!("extraInfo {:?}, {:?}", contract.marketCreationData.get(address).extraInfo.position(), contract.marketCreationData.get(address).extraInfo.storage());


    // use std::time::Instant;
    //
    // struct ConcreteStruct {
    //     value: u32,
    // }
    //
    // let start = Instant::now();
    //
    // for _ in 0..1_000_000 {
    //     let _boxed = Box::new(ConcreteStruct { value: 42 });
    // }
    //
    // let duration = start.elapsed();
    // println!("Time elapsed in expensive_function() is: {:?}", duration / 1_000_000);
}
