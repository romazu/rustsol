fn main() {
    let storage_layout_path = "example/solc_output.json";
    let output_path = "example/src/generated_contract.rs";

    // let (contract_path, contract_name) = ("contract.sol", "MyContract");
    let (contract_path, contract_name) = ("UniswapV3Pool.sol", "UniswapV3Pool");
    // let (contract_path, contract_name) = ("Augur.sol", "Augur");

    rustsol::generate_storage_bindings(
        storage_layout_path.into(),
        contract_path.into(),
        contract_name.into(),
        output_path.into(),
    );
}
