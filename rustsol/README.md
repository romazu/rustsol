# rustsol

## Description

`rustsol` is a tool designed to generate Rust storage bindings for Ethereum smart contracts.
These bindings can be used to retrieve storage slots, offsets, and sizes of all stored variables within a contract,
including objects stored in mappings and arrays.

## Bindings Example Usage

```rust
let contract = generated_contract::UniswapV3Pool::new();
let (slot, offset, size_bytes) = contract.observations.get(42).tickCumulative.position();
println!("slot={}, offset={}, size_bytes={}", slot, offset, size_bytes);
// Output: slot=50, offset=4, size_bytes=7
```

## Generated Structs

The generated structs from the bindings will look similar to the following:

```rust
pub struct UniswapV3Pool {
    __slot: U256,
    pub slot0: UniswapV3PoolSlot0,
    pub feeGrowthGlobal0X128: Primitive<32>,
    pub feeGrowthGlobal1X128: Primitive<32>,
    pub protocolFees: UniswapV3PoolProtocolFees,
    pub liquidity: Primitive<16>,
    pub ticks: Mapping<PrimitiveKey, TickInfo>,
    pub tickBitmap: Mapping<PrimitiveKey, Primitive<32>>,
    pub positions: Mapping<PrimitiveKey, PositionInfo>,
    pub observations: StaticArray<2097120, OracleObservation>,
}
```

## Installation

```bash
cargo install rustsol
```

## Setup and Usage

### 1. Get Contract Storage Layout

To generate storage bindings for a contract, you first need the contract's storage layout in JSON format.
For Solidity contracts, this can be generated using the `solc` compiler.
`rustsol` provides a simple Python script to assist with this process.
See the original repository for instructions.


### 2. Generate Bindings

To generate bindings, use the following command:

```bash
rustsol generate_storage_bindings your_solc_output.json contract_path contract_name generated_contract.rs
```

To understand what `contract_path` and `contract_name` mean, let's look at an example storage layout JSON:

```json
{
  "contracts": {
    "UniswapV3Pool.sol": {
      "UniswapV3Pool": {
        "storageLayout": {
          ...
        }
      }
    }
  }
}
```

Here, `UniswapV3Pool.sol` is the contract path, and `UniswapV3Pool` is the contract name.

For the provided example, it would be:

```bash
rustsol generate_storage_bindings example/solc_output.json UniswapV3Pool.sol UniswapV3Pool example/src/generated_contract.rs
```

After running the above command, the generated structs will be in `generated_contract.rs` file.
See the original repository for an example.

## Type Bindings
Currently, Solidity -> Rust type mapping is as follows:

| Solidity Type                 | Generated Rust Type                        |
|-------------------------------|--------------------------------------------|
| all integer types, bool, enum | `Primitive<byte_size>`                     |
| address                       | `Address`                                  |
| string, bytes                 | `Bytes`                                    |
| static arrays                 | `StaticArray<byte_size, value_type>`       |
| dynamic arrays                | `DynamicArray<value_type>`                 |
| mapping                       | `Mapping<key_type, value_type>`            |
| struct                        | `CustomNamedStructWithCorrespondingFields` |

Plans include getting rid of the `Primitive` type and having separate types for integers, bool, and enum.
Also we plan to have a separate String type.


## License

This project is licensed under the MIT License.
