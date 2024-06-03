fn main() {
    let storage_layout_path = "example/solc_output_simple.json";
    let contract_path = "contract.sol";
    let contract_name = "MyContract";
    let output_path = "example/src/generated_contract_simple.rs";
    rustsol::generate_storage_bindings(
        storage_layout_path.into(),
        contract_path.into(),
        contract_name.into(),
        output_path.into(),
    );

    let storage_layout_path = "example/solc_output_uniswap3pool.json";
    let contract_path = "UniswapV3Pool.sol";
    let contract_name = "UniswapV3Pool";
    let output_path = "example/src/generated_contract_uniswap3pool.rs";
    rustsol::generate_storage_bindings(
        storage_layout_path.into(),
        contract_path.into(),
        contract_name.into(),
        output_path.into(),
    );

    let storage_layout_path = "example/solc_output_augur.json";
    let contract_path = "Augur.sol";
    let contract_name = "Augur";
    let output_path = "example/src/generated_contract_augur.rs";
    rustsol::generate_storage_bindings(
        storage_layout_path.into(),
        contract_path.into(),
        contract_name.into(),
        output_path.into(),
    );

}
