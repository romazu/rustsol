#![allow(unused_imports)]

use std::str::FromStr;
use ethereum_types::{Address, H160, U256};
mod generated_contract;

fn main() {
    // Example contract
    // let contract = generated_contract::MyContract::new());
    // println!("{:?}", contract.myNestedMapping.get_item(0u64));
    // println!("{:?}", contract.myNestedMapping.get_item(0u64).get_item(1u64).position());
    // println!("myMapping2[\"some\"] {:?}", contract.myMapping2.get_item("some").position());
    // println!("myMapping2[vec]    {:?}", contract.myMapping2.get_item(vec![0x73, 0x6F, 0x6D, 0x65]).position());
    // println!("{:?}", contract.dynamicArray.get_item(10).position());
    // println!("{:?}", contract.dynamicArrayStruct.get_item(10).position());
    // println!("{:?}", contract.dynamicArrayStruct.get_item(11).position());
    // println!("{:?}", contract.dynamicArraySmall.get_item(10).position());
    // println!("{:?}", contract.dynamicArraySmall.get_item(11).position());
    // println!("{:?}", contract.staticArrayNestedSmall.position());
    // println!("{:?}", contract.staticArrayNestedSmall.get_item(0).position());
    // println!("{:?}", contract.staticArrayNestedSmall.get_item(1).position());
    // println!("{:?}", contract.staticArrayNestedSmall.get_item(0).get_item(0).position());
    // println!("{:?}", contract.staticArrayNestedSmall.get_item(0).get_item(31).position());
    // println!("{:?}", contract.staticArrayLarge.get_item(0).position());
    // println!("{:?}", contract.staticArrayLarge.get_item(1).position());
    // println!("{:?}", contract.staticArrayLarge.capacity());
    // println!("{:?}", contract.staticArrayLarge.capacity());
    // // println!("{:?}", contract.staticArrayLarge.get_item(2).position()); // panic

    // Uniswap V3
    let contract = generated_contract::UniswapV3Pool::new();
    println!("{:?}", contract.slot0);
    println!("{:?}", contract.ticks);
    println!("{:?}", contract.ticks.get_item(42u64).initialized.position());
    println!("{:?}", contract.slot0.observationIndex.position());
    println!("{:?}", contract.slot0.observationIndex.offset());
    println!("{:?}", contract.slot0.observationIndex.size());
    println!("{:?}", contract.ticks.get_item(0).position());
    println!("{:?}", contract.ticks.get_item(149150).position());
    println!("{:?}", contract.ticks.get_item(887270).position());
    println!("{:?}", contract.ticks.get_item(-92110).position());

    // Check that PrimitiveKey accepts Address type input.
    let address = Address::from_str("0x51A18333479472D1250Ee5063910079fc0B9b801").unwrap();
    println!("AddressKey check (Address): {:?}", contract.ticks.get_item(address).position());
    println!("AddressKey check (H160): {:?}", contract.ticks.get_item(H160::zero()).position());

    let contract = generated_contract::UniswapV3Pool::new();
    let (slot, offset, size_bytes) = contract.observations.get_item(42).tickCumulative.position();
    println!("slot={}, offset={}, size_bytes={}", slot, offset, size_bytes);

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
    // println!("s_transmitters[0] {:?}", contract.s_transmitters.get_item(0).position());
    // println!("s_transmitters[1] {:?}", contract.s_transmitters.get_item(1).position());
    // println!("s_signers[0] {:?}", contract.s_signers.get_item(0).position());

    // // Augur 2
    // TODO: let contract = ...
    // let address = U256::from_str("0x51A18333479472D1250Ee5063910079fc0B9b801").unwrap();
    // println!("{:#x}", address);
    // println!("marketCreationData {:?}", contract.marketCreationData.get_item(address).position());
    // println!("extraInfo {:?}, {:?}", contract.marketCreationData.get_item(address).extraInfo.position(), contract.marketCreationData.get_item(address).extraInfo.storage());
}
