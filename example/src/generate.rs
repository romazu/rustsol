fn main() {
    let storage_layout_path = "example/solc_output.json";
    let generated_file_path = "example/src/generated_contract.rs";

    // let (contract_path, contract_name) = ("contract.sol", "MyContract");
    let (contract_path, contract_name) = ("UniswapV3Pool.sol", "UniswapV3Pool");

    rustsol::generate(
        storage_layout_path.into(),
        generated_file_path.into(),
        contract_path.into(),
        contract_name.into(),
    );
}
