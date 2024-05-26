use primitive_types::U256;
use rustsol::types::Position;

mod generated_contract;



fn main() {
    let contract = generated_contract::Contract::new_from_position(U256::zero(), 0);

    println!("{:#?}", contract.slot0);
    println!("{:#?}", contract.ticks);
    println!("{:#?}", contract.ticks.get_item(42u64).initialized.slot());
    println!("{:#?}", contract.slot0.observationIndex.slot());
    println!("{:#?}", contract.slot0.observationIndex.offset());
    println!("{:#?}", contract.slot0.observationIndex.size());
    println!("{:#?}", contract.ticks.get_item(0).slot());
    println!("{:#?}", contract.ticks.get_item(149150).slot());
    println!("{:#?}", contract.ticks.get_item(887270).slot());
    println!("{:#?}", contract.ticks.get_item(-92110).slot());

    // println!("{:?}", contract.myNestedMapping.get_item(0u64));
    // println!("{:?}", contract.myNestedMapping.get_item(0u64).get_item(1u64));
    // println!("{:?}", contract.myMapping2.get_item("0u64"));
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
}
