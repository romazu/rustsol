#![allow(non_snake_case)]
use std::sync::Arc;
use ethereum_types::Address;
use rustsol::types::SlotsGetterSetter;
use rustsol::utils::u256_to_u64;

mod generated_contract;
mod utils;

use crate::generated_contract::UniswapV3Pool;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // revm project's instance, don't abuse.
    let provider_url = "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27";

    // UniswapV3Pool | USDC WETH 500
    let contract_address: Address = "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640".parse().unwrap();

    let slots_getter = Arc::new(
        utils::EthereumSlotsGetter::new(
            provider_url,
            utils::SlotsGetterContext { contract: contract_address, block: None },
        )?
    );

    let mut contract = UniswapV3Pool::new();
    contract.set_slots_getter(slots_getter);
    let observationIndex = contract.slot0.observationIndex.get_value().unwrap();
    let observation = contract.observations.at(u256_to_u64(observationIndex) as usize);
    println!("slot0.observationIndex.get_value()        {}", observationIndex.to_string());
    println!("slot0.observationCardinality.get_value()  {}", contract.slot0.observationCardinality.get_value().unwrap().to_string());
    println!("observations[].blockTimestamp.get_value() {}", observation.blockTimestamp.get_value().unwrap().to_string());
    println!("observations[].tickCumulative.position()  {:?}", observation.tickCumulative.position());
    println!("observations[].tickCumulative.get_value() {}", observation.tickCumulative.get_value().unwrap().to_string());
    println!("liquidity.get_value()                     {}", contract.liquidity.get_value().unwrap().to_string());
    println!("slot0.get_value()                         {:?}", contract.slot0.get_value().unwrap());
    // Get all base storage of contract, take ~infinity time for UniswapV3 because of static array of 65535
    // println!("contract.get_value()                      {:?}", contract.get_value());

    Ok(())
}
