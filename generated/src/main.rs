use primitive_types::U256;

mod generated_contract;



fn main() {
    let contract = generated_contract::Contract::new_from_position(U256::zero(), U256::zero());

    // println!("{:#?}", contract.slot0);
    // println!("{:#?}", contract.ticks);
    // println!("{:#?}", contract.ticks.get_item(42u64).initialized.slot());
    println!("{:#?}", contract.ticks.get_item(42).slot());
    println!("{:#?}", contract.ticks.get_item(42u8).slot());
    println!("{:#?}", contract.ticks.get_item(42i64).slot());

    // println!("{:#?}", contract.myNestedMapping.get_item(0u64));
    // println!("{:#?}", contract.myNestedMapping.get_item(0u64).get_item(1u64));
    // println!("{:#?}", contract.myMapping2.get_item("0u64"));
}
