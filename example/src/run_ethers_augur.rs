#![allow(non_snake_case)]

use std::sync::Arc;
use alloy_primitives::address;
use ethereum_types::Address;
use rustsol::types::SlotsGetterSetter;

mod utils;

mod generated_contract_augur;

use crate::generated_contract_augur::Augur;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // revm project's instance, don't abuse.
    let provider_url = "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27";

    // Augur
    let contract_address: Address = "0x23916a8F5C3846e3100e5f587FF14F3098722F5d".parse().unwrap();

    let slots_getter = Arc::new(
        utils::EthersSlotsGetter::new(
            provider_url,
            utils::EthersSlotsGetterContext { contract: contract_address, block: None },
        )?
    );

    // with getter
    let mut contract = Augur::new();
    contract.set_slots_getter(slots_getter);
    let market_address = address!("51A18333479472D1250Ee5063910079fc0B9b801");
    let bytes = contract.marketCreationData.at(market_address).extraInfo.get_value().unwrap();
    println!("marketCreationData[].extraInfo.get_value()     {}", String::from_utf8_lossy(&bytes));
    println!("cash.get_value()                               {}", contract.cash.get_value().unwrap());
    println!("marketCreationData[].marketCreator.get_value() {}", contract.marketCreationData.at(market_address).marketCreator.get_value().unwrap());
    println!("marketCreationData.get_value_at()              {:?}", contract.marketCreationData.get_value_at(market_address).unwrap());
    println!("marketCreationData[].get_value()               {:?}", contract.marketCreationData.at(market_address).get_value().unwrap()); // equivalent to above
    // Get all the base storage of contract. Long and verbose.
    // println!("contract.get_value()                           {:?}", contract.get_value());

    Ok(())

}
