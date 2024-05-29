use rustsol;
use clap::Parser;

/// Command line arguments
#[derive(Parser)]
struct Args {
    /// Path to the storage layout JSON file
    storage_layout_path: String,

    /// Contract path string in storage layout JSON
    contract_path: String,

    /// Name of the contract in storage layout JSON
    contract_name: String,

    /// Path to the generated contract file
    output_path: String,
}

fn main() {
    let args = Args::parse();

    rustsol::generate_storage_bindings(
        args.storage_layout_path.into(),
        args.contract_path.into(),
        args.contract_name.into(),
        args.output_path.into(),
    );
}
