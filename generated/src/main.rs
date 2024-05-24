mod generated_contract;



fn main() {
    let contract = generated_contract::MyContract::default();
    println!("{:#?}", contract.myNestedMapping);
    println!("{:#?}", contract.myNestedMapping.get_item(0u64));
    println!("{:#?}", contract.myNestedMapping.get_item(0u64).get_item(1u64));
    println!("{:#?}", contract.myMapping2.get_item("0u64"));
}
