use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::marker::PhantomData;
use std::ops::Index;
use prettyplease::unparse;
use primitive_types::U256;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use serde::Deserialize;
use sha3::digest::consts::U2;
use syn::{Item, parse_str};
use crate::keccak::{bytes32_to_u256, keccak256_concat, u256_to_bytes32};

mod layout;

mod data;
mod keccak;
mod types;
mod generate;

use types::{Primitive, Mapping};
use crate::layout::NestedType;

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
    //         },2
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

    let (member_defs, nested_types) = storage_layout.traverse();

    for member_def in &member_defs {
        println!("{:?}", member_def);
    }
    for nested_type in &nested_types {
        println!("{:?}", nested_type);
    }

    let main_struct_definition = generate::generate_struct_from_member_defs("MyContract", &member_defs);
    // println!("{}", main_struct_definition);

    // let mut nested_struct_definitions: Vec<TokenStream> = Vec::new();
    // for nested_type in nested_types {
    //     match nested_type {
    //         NestedType::Struct { label, members } => {
    //             let nested_struct_definition = generate::generate_struct_from_member_defs(&label, members);
    //             nested_struct_definitions.push(nested_struct_definition)
    //         }
    //         _ => {}
    //     }
    // }

    // let generated_tokens = main_struct_definition;

    // let predefined_structs_content = fs::read_to_string("src/types.rs").expect("Unable to read file");
    // let syntax_tree1 = syn::parse_file(&predefined_structs_content).expect("Unable to parse file");
    // let predefined_structs_tokens = syntax_tree1.to_token_stream();
    //
    // // Combine predefined structures with generated struct
    // let generated_tokens = quote! {
    //     #predefined_structs_tokens
    //     #main_struct_definition
    // };

    let imports_definition_items: Vec<Item> = vec![
        parse_str("use rustsol::types::{Primitive, Bytes, Mapping, PrimitiveKey, BytesKey};").expect("Failed to parse"),
    ];
    let imports_definition: TokenStream = imports_definition_items.into_iter().map(|item| item.into_token_stream()).collect();
    let generated_tokens = quote! {
        #imports_definition
        #main_struct_definition
        // #(#nested_struct_definitions)*
    };


    // Convert TokenStream to a pretty-printed string
    let syntax_tree = syn::parse_file(&generated_tokens.to_string()).expect("Failed to parse TokenStream");
    let formatted_code = prettyplease::unparse(&syntax_tree);

    let file_path = "generated/src/generated_contract.rs";
    let mut file = File::create(file_path).expect("Unable to create file");
    file.write_all(formatted_code.as_bytes()).expect("Unable to write data");
}
