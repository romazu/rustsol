#![allow(non_snake_case)]

use std::sync::Arc;
use std::time::Instant;
use ethereum_types::Address;
use rustsol::types::SlotsGetterSetter;

mod utils;

mod generated_contract_uniswap3pool;

use crate::generated_contract_uniswap3pool::UniswapV3Pool;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // revm project's instance, don't abuse.
    let provider_url = "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27";

    // UniswapV3Pool | USDC WETH 500
    let contract_address: Address = "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640".parse().unwrap();

    let slots_getter = Arc::new(
        utils::EthersSlotsGetter::new(
            provider_url,
            utils::EthersSlotsGetterContext { contract: contract_address, block: None },
        )?
    );

    // no getter
    let contract = UniswapV3Pool::new();
    println!("slot0 {:#?}", contract.slot0);
    println!("ticks {:#?}", contract.ticks);
    // println!("ticks[42u64].initialized {:?}", contract.ticks.at(42u64).initialized.position()); // Won't compile: incorrect key type.
    println!("ticks[42u64].initialized {:?}", contract.ticks.at(42).initialized.position());
    println!("slot0.observationIndex   {:?}", contract.slot0.observationIndex.position());
    println!("ticks[0]                 {:?}", contract.ticks.at(0).position());
    println!("ticks[149150]            {:?}", contract.ticks.at(149150).position());
    println!("ticks[887270]            {:?}", contract.ticks.at(887270).position());
    println!("ticks[-92110]            {:?}", contract.ticks.at(-92110).position());
    // println!("feeGrowthGlobal0X128.get_value() {:?}", contract.feeGrowthGlobal0X128.get_value()); // panic "No slots getter"

    let (slot, offset, size_bytes) = contract.observations.at(42).tickCumulative.position();
    println!("observations.at(42).tickCumulative.position(): slot={}, offset={}, size_bytes={}", slot, offset, size_bytes);


    // with getter
    let mut contract = UniswapV3Pool::new();
    contract.set_slots_getter(slots_getter);
    let observationIndex = contract.slot0.observationIndex.get_value().unwrap();
    let observation = contract.observations.at(observationIndex as usize);
    println!("slot0.observationIndex.get_value()        {}", observationIndex.to_string());
    println!("slot0.observationCardinality.get_value()  {}", contract.slot0.observationCardinality.get_value().unwrap().to_string());
    println!("observations[].blockTimestamp.get_value() {}", observation.blockTimestamp.get_value().unwrap().to_string());
    println!("observations[].tickCumulative.position()  {:?}", observation.tickCumulative.position());
    println!("observations[].tickCumulative.get_value() {}", observation.tickCumulative.get_value().unwrap().to_string());
    println!("liquidity.get_value()                     {}", contract.liquidity.get_value().unwrap().to_string());
    println!("slot0.get_value()                         {:?}", contract.slot0.get_value().unwrap());
    // Get all the base storage of contract. Take ~infinity time for UniswapV3Pool because of static array of length 65535.
    // println!("contract.get_value()                      {:?}", contract.get_value());

    let tick_value = contract.ticks.at(-92110).get_value().unwrap();
    println!("ticks.at(-92110).get_value() {:?}", tick_value);

    // Change contract address.
    // TODO: Create a method to change context conveniently.
    let slots_getter = Arc::new(
        utils::EthersSlotsGetter::new(
            provider_url,
            utils::EthersSlotsGetterContext {
                contract: "0x92560c178ce069cc014138ed3c2f5221ba71f58a".parse().unwrap(),
                block: None },
        )?
    );
    // Set slots getter only for this variable.
    let mut tick_info = contract.ticks.at(39120);
    tick_info.set_slots_getter(slots_getter);
    let value = tick_info.get_value().unwrap().tickCumulativeOutside;
    println!("ticks.at(39120).get_value().tickCumulativeOutside {}", value); // negative value

    // benchmark
    let n = 100_000;
    let start = Instant::now();
    for _ in 0..n {
        contract.ticks.at(-92110).position();
    }
    let duration = start.elapsed();
    println!("Call duration in benchmark is: {:?}", duration / n);
    // debug:   19.564µs @ 100_000
    // release:    565ns @ 100_000
    // release:    284ns @ 10_000_000

    Ok(())

}
