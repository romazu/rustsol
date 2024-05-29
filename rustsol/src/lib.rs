pub mod types;
pub mod layout;
pub mod utils;
pub mod generate;

use std::fs::File;
use std::io::{Read, Write};

pub fn generate_storage_bindings(storage_layout_path: String, contract_path: String, contract_name: String, output_path: String) {
    let mut file = File::open(storage_layout_path).expect("Cannot open storage layout json file");
    let mut storage_layout_json_string = String::new();
    file.read_to_string(&mut storage_layout_json_string).expect("Cannot read storage layout json file");
    let solc_output: layout::SolcOutput = serde_json::from_str(&storage_layout_json_string).expect("JSON was not well-formatted");
    let storage_layout = &solc_output.contracts[&contract_path][&contract_name].storage_layout;

    let nested_types = storage_layout.traverse(contract_name);
    let generated_tokens = generate::generate_structs(nested_types);

    let syntax_tree = syn::parse_file(&generated_tokens.to_string()).expect("Failed to parse TokenStream");
    let pretty_formatted_code = prettyplease::unparse(&syntax_tree);

    let mut file = File::create(output_path).expect("Unable to create file");
    file.write_all(pretty_formatted_code.as_bytes()).expect("Unable to write data");
}