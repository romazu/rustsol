use std::str::FromStr;
use primitive_types::U256;
use rustsol::keccak::u256_to_bytes32;
use rustsol::types::Position;

mod generated_contract;



fn main() {
    let contract = generated_contract::Contract::new_from_position(U256::zero(), 0);

    // // Example contract
    // println!("{:?}", contract.myNestedMapping.get_item(0u64));
    // println!("{:?}", contract.myNestedMapping.get_item(0u64).get_item(1u64));
    // // println!("{:?}", contract.myMapping2.get_item("0u64")); // panic
    // println!("{:?}", contract.dynamicArray.get_item(10).slot());
    // println!("{:?}", contract.dynamicArrayStruct.get_item(10).slot());
    // println!("{:?}", contract.dynamicArrayStruct.get_item(11).slot());
    // println!("{:?}", contract.dynamicArraySmall.get_item(10).slot());
    // println!("{:?}", contract.dynamicArraySmall.get_item(11).slot());
    // println!("{:?}", contract.staticArrayNestedSmall.slot());
    // println!("{:?}", contract.staticArrayNestedSmall.get_item(0).slot());
    // println!("{:?}", contract.staticArrayNestedSmall.get_item(1).slot());
    // println!("{:?}", contract.staticArrayNestedSmall.get_item(0).get_item(0).position());
    // println!("{:?}", contract.staticArrayNestedSmall.get_item(0).get_item(31).position());
    // println!("{:?}", contract.staticArrayLarge.get_item(0).slot());
    // println!("{:?}", contract.staticArrayLarge.get_item(1).slot());
    // println!("{:?}", contract.staticArrayLarge.capacity());
    // println!("{:?}", contract.staticArrayLarge.capacity());
    // // println!("{:?}", contract.staticArrayLarge.get_item(2).slot()); // panic

    // // Uniswap V3
    // println!("{:?}", contract.slot0);
    // println!("{:?}", contract.ticks);
    // println!("{:?}", contract.ticks.get_item(42u64).initialized.slot());
    // println!("{:?}", contract.slot0.observationIndex.slot());
    // println!("{:?}", contract.slot0.observationIndex.offset());
    // println!("{:?}", contract.slot0.observationIndex.size());
    // println!("{:?}", contract.ticks.get_item(0).slot());
    // println!("{:?}", contract.ticks.get_item(149150).slot());
    // println!("{:?}", contract.ticks.get_item(887270).slot());
    // println!("{:?}", contract.ticks.get_item(-92110).slot());

    // // Uniswap V2
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
    // println!("owner {:?}", contract.owner.position());
    // println!("s_description {:?}, {:?}", contract.s_description.position(), contract.s_description.storage());
    // println!("s_transmitters {:?}", contract.s_transmitters.position());
    // println!("s_transmitters[0] {:?}", contract.s_transmitters.get_item(0).position());
    // println!("s_transmitters[1] {:?}", contract.s_transmitters.get_item(1).position());
    // println!("s_signers[0] {:?}", contract.s_signers.get_item(0).position());

    // Augur 2
    let address = U256::from_str("0x51A18333479472D1250Ee5063910079fc0B9b801").unwrap();
    println!("{:#x}", address);
    println!("{:#?}", address);
    println!("{:?}", u256_to_bytes32(address));
    println!("marketCreationData {:?}", contract.marketCreationData.get_item(address).position());
    println!("extraInfo {:?}, {:?}", contract.marketCreationData.get_item(address).extraInfo.position(), contract.marketCreationData.get_item(address).extraInfo.storage());
}
