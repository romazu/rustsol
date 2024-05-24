use std::fs::File;
use std::io::Read;
use std::marker::PhantomData;
use std::ops::Index;
use primitive_types::U256;
use serde::Deserialize;
use sha3::digest::consts::U2;
use crate::keccak::{bytes32_to_u256, keccak256_concat, u256_to_bytes32};

mod layout;

mod data;
mod keccak;
mod types;
use types::{Primitive, Mapping};

//
// #[derive(Debug)]
// struct MyStruct {
//     __slot: U256,
//     myUint0: Primitive,
//     myUint1: Primitive,
// }
//
// impl From<U256> for MyStruct {
//     fn from(u: U256) -> Self {
//         MyStruct{__slot: u, myUint0: Primitive::from(u), myUint1: Primitive::from(u)}  // Use the conversion from U256 to u64
//     }
// }
//
// struct MyContract {
//     plainUint112: Primitive,
//     plainUint32: Primitive,
//     myMapping: Mapping<Primitive>,
//     myStruct: MyStruct,
// }


fn main() {
    // let contract = MyContract {
    //     plainUint112: Primitive {
    //         __slot: U256::from(0),
    //         __offset: U256::from(0),
    //     },
    //     plainUint32: Primitive {
    //         __slot: U256::from(0),
    //         __offset: U256::from(14),
    //     },
    //     myMapping: Mapping::new(U256::from(1)),
    //     myStruct: MyStruct {
    //         __slot: U256::from(2),
    //         myUint0: Primitive {
    //             __slot: U256::from(2),
    //             __offset: U256::from(0),
    //         },
    //         myUint1: Primitive {
    //             __slot: U256::from(3),
    //             __offset: U256::from(0),
    //         },
    //     },
    // };
    // println!("{:#?}", contract.plainUint32.slot());

    // let mapping = Mapping::<MyStruct>::from(U256::from(137));
    // println!("Value: {:?}", mapping[10u64]);  // Outputs: Value: MyStruct(137)

    let mut file = File::open("scratch_output.json").expect("Cannot open storage layout json file");
    let mut storage_layout_json_string = String::new();
    file.read_to_string(&mut storage_layout_json_string).expect("Cannot read storage layout json file");
    let storage_layout: layout::StorageLayout = serde_json::from_str(&storage_layout_json_string).expect("JSON was not well-formatted");
    println!("{:#?}", storage_layout);
    // println!("{:#?}", storage_layout.types["t_uint32"]);
    // println!("{:#?}", storage_layout.types["t_array(t_uint112)10_storage"]);
    // println!("{:#?}", storage_layout.members[0]);

    // let member = &storage_layout.members[6];
    // let ftype = &storage_layout.types[&member.type_name];

    // // let ket_ftype_name = &storage_layout.types[]
    // println!("{:#?}", member);
    // println!("{:#?}", ftype);

    let (member_defs, nested_types) = storage_layout.traverse_mappings();

    for member_def in member_defs {
        println!("{:?}", member_def);
    }
    for nested_type in nested_types {
        println!("{:?}", nested_type);
    }


}
