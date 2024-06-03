# rustsol

## Description

`rustsol` is a tool designed to generate Rust storage bindings for Ethereum smart contracts.
These bindings can be used to retrieve storage slots, offsets, and sizes of all stored variables within a contract,
including objects stored in mappings and arrays.

## Bindings Example Usage

Get storage position of a contract variable:
```rust
let contract = generated_contract::UniswapV3Pool::new();
let (slot, offset, size_bytes) = contract.observations.at(42).tickCumulative.position();
println!("slot={}, offset={}, size_bytes={}", slot, offset, size_bytes);
// Output:
// slot=50, offset=4, size_bytes=7
```

Get storage value of a contract variable using provided slots getter:
```rust
let contract = generated_contract::UniswapV3Pool::new();
contract.set_slots_getter(my_slots_getter);
let value = contract.observations.at(42).tickCumulative.get_value();
println!("{:?}", value);
// Output (prettified):
// TickInfoValue {
//     liquidityGross:                 0x0000000000000000000000000000000000000000000000000000005cbbfb3715_U256
//     liquidityNet:                   0x0000000000000000000000000000000000000000000000000000005cbbfb3715_U256
//     feeGrowthOutside0X128:          0x0000000000000000000000000000000000000b73d798604f1b0cd4f1d544c646_U256
//     feeGrowthOutside1X128:          0x000000000000000000000000000000f2a960acbe8891e526c025b819077f15ae_U256
//     tickCumulativeOutside:          0x00000000000000000000000000000000000000000000000000000090f431361b_U256
//     secondsPerLiquidityOutsideX128: 0x0000000000000000000000000000000000000001e576ee66a9d9f002e36fad4c_U256
//     secondsOutside:                 0x0000000000000000000000000000000000000000000000000000000060c36c13_U256
//     initialized:                    0x0000000000000000000000000000000000000000000000000000000000000001_U256
// }
```

## Generated Structs

The generated structs from the bindings will look similar to the following:

```rust
pub struct UniswapV3Pool {
    __slot: U256,
    __slots_getter: Option<Arc<dyn SlotsGetter>>,
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
rustsol generate_storage_bindings your_solc_output.json contract_path contract_name generated_contract_uniswap3pool.rs
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


## Types Mapping
### Solidity -> Rust Mapping
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

In the future we plans to use generic `Primitive<NativeType>` for integers, addresses, bool, enum types
and `Bytes<NativeType>` for string and byte types.

### Value Types Mapping
Types of variable values obtained with get_value() methods are mapped as the following:

| Solidity Type | Generated Rust Type                    |
|---------------|----------------------------------------|
| Primitive     | `U256`                                 |
| Address       | `alloy_primitives::Address`            |
| Bytes         | `Vec[u8]`                              |
| StaticArray   | `Vec[value_type]`                      |
| DynamicArray  | `Vec[value_type]`                      |
| Mapping       | `Mapping<key_type, value_type>` getter |
| SomeStruct    | `SomeStructValue`                      |



## License

This project is licensed under the MIT License.
