# rustsol

## Description

`rustsol` is a tool designed to generate Rust storage bindings for Ethereum smart contracts.
These bindings can be used to retrieve storage slots, offsets, and sizes of all stored variables within a contract, including objects stored in mappings and arrays.
When initialized with a `slots_getter`, the generated structures can be used to conveniently access the values of these storage variables.

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
let tick_value = contract.ticks.at(-92110).get_value().unwrap();
println!("{:?}", tick_value);
// Output (prettified):
// TickInfoValue {
//     liquidityGross:                 398290794261,
//     liquidityNet:                   398290794261,
//     feeGrowthOutside0X128:          0x0000000000000000000000000000000000000b73d798604f1b0cd4f1d544c646_U256,
//     feeGrowthOutside1X128:          0x000000000000000000000000000000f2a960acbe8891e526c025b819077f15ae_U256,
//     tickCumulativeOutside:          622572156443,
//     secondsPerLiquidityOutsideX128: 0x0000000000000000000000000000000000000001e576ee66a9d9f002e36fad4c_U256,
//     secondsOutside:                 1623419923,
//     initialized:                    true
// }
```

## Generated Structs

The generated structs from the bindings will look similar to the following:

```rust
pub struct UniswapV3Pool {
    __slot: U256,
    __slots_getter: Option<Arc<dyn SlotsGetter>>,
    pub slot0: UniswapV3PoolSlot0,
    pub feeGrowthGlobal0X128: Primitive<32, U256>,
    pub feeGrowthGlobal1X128: Primitive<32, U256>,
    pub protocolFees: UniswapV3PoolProtocolFees,
    pub liquidity: Primitive<16, u128>,
    pub ticks: Mapping<i32, TickInfo>,
    pub tickBitmap: Mapping<i16, Primitive<32, U256>>,
    pub positions: Mapping<U256, PositionInfo>,
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
rustsol generate_storage_bindings example/solc_output_uniswap3pool.json UniswapV3Pool.sol UniswapV3Pool example/src/generated_contract_uniswap3pool.rs
```

After running the above command, the generated structs will be in `generated_contract.rs` file.
See the original repository for an example.


## Types Mapping
### Solidity -> Rust Mapping
Solidity variable types are mapped to `rustsol` binding types.
This mapping is shown in the table below.
Here, `value_type` corresponds to a (possibly nested) `rustsol` binding type, such as `Primitive` or `Mapping`.
The `native_type` is a type to which the actual slot values are converted, such as `u64`, `bool` and `U256`.
Note that the keys of `Mappings` are also native types.

| Solidity Type                                                           | Generated Rust Type                        |
|-------------------------------------------------------------------------|--------------------------------------------|
| all integer types, bool, enum,<br/> address, contract, small bytes1..32 | `Primitive<byte_size, native_type>`        |
| string, bytes                                                           | `Bytes<String>`, `Bytes<Vec<u8>>`          |
| static arrays                                                           | `StaticArray<byte_size, value_type>`       |
| dynamic arrays                                                          | `DynamicArray<value_type>`                 |
| mapping                                                                 | `Mapping<key_native_type, value_type>`     |
| struct                                                                  | `CustomNamedStructWithCorrespondingFields` |

Note that all enums are mapped to `Primitive<byte_size, U256>`.
It would be better to use native Rust enums as the `native_type` instead of the generic `U256`,
but this requires deeper Solidity contract analysis,
as information about enum field names is not available from the storage layout.


### Value Types Mapping
Variable values obtained with `get_value()` are converted to native types according to the table below.

| `rustsol` Type              | Generated Rust Type                           |
|-----------------------------|-----------------------------------------------|
| `Primitive<_, native_type>` | `native_type`                                 |
| `Bytes<String>`             | `String`                                      |
| `Bytes<Vec<u8>>`            | `Vec<u8>`                                     |
| `StaticArray<value_type>`   | `Vec<recursive native type of value_type>`    |
| `DynamicArray`              | `Vec<recursive native type of value_type>`    |
| `Mapping`                   | `Mapping<key_native_type, value_type>` getter |
| `SomeStruct`                | `SomeStructValue`                             |

Note that `Mapping` types are actually mapped to the same `Mapping`.
This is because there is no way to get all elements of a Solidity mapping.
Therefore, we return a value getter instead of a concrete value.


## License

This project is licensed under the MIT License.
