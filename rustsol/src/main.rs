use std::fs::File;
use std::io::{Read, Write};
use std::ops::Index;
use quote::ToTokens;
use serde::Deserialize;

mod layout;

mod data;
mod utils;
mod types;
mod generate;


fn main() {
    let mut file = File::open("scratch_output.json").expect("Cannot open storage layout json file");
    let mut storage_layout_json_string = String::new();
    file.read_to_string(&mut storage_layout_json_string).expect("Cannot read storage layout json file");
    let storage_layout: layout::StorageLayout = serde_json::from_str(&storage_layout_json_string).expect("JSON was not well-formatted");
    // println!("{:#?}", storage_layout);
    // println!("{:#?}", storage_layout.types["t_uint32"]);
    // println!("{:#?}", storage_layout.types["t_array(t_uint112)10_storage"]);
    // println!("{:#?}", storage_layout.members[0]);

    // let member = &storage_layout.members[6];
    // let ftype = &storage_layout.types[&member.type_name];

    // // let ket_ftype_name = &storage_layout.types[]
    // println!("{:#?}", member);
    // println!("{:#?}", ftype);

    let nested_types = storage_layout.traverse();
    // for nested_type in &nested_types {
    //     println!("{:?}", nested_type);
    // }

    let generated_tokens = generate::generate_structs(nested_types);
    // println!("{}", generated_tokens);

    // Convert TokenStream to a pretty-printed string
    let syntax_tree = syn::parse_file(&generated_tokens.to_string()).expect("Failed to parse TokenStream");
    let formatted_code = prettyplease::unparse(&syntax_tree);

    let file_path = "generated/src/generated_contract.rs";
    let mut file = File::create(file_path).expect("Unable to create file");
    file.write_all(formatted_code.as_bytes()).expect("Unable to write data");
}
