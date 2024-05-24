use std::marker::PhantomData;
use std::ops::Index;
use primitive_types::U256;
use serde::Deserialize;
use sha3::digest::consts::U2;
use crate::keccak::{bytes32_to_u256, keccak256_concat, u256_to_bytes32};

mod layout;

mod data;
mod keccak;
mod types;
use types::Primitive;


// Define the Mapping struct with generic types for Key and Value
struct Mapping<Value> {
    __slot: U256,
    __marker: PhantomData<Value>,
}

// Implement the Mapping struct
impl<Value> Mapping<Value> {
    fn new(slot: U256) -> Self {
        Self {
            __slot: slot,
            __marker: PhantomData,
        }
    }

    fn get_item<Key>(&self, key: Key) -> Value
        where
            Key: Into<CommonKey>,
            Value: From<U256>,
    {
        // Convert the key into a common representation [u8; 32]
        let key_bytes = key.into().to_key_bytes();
        let value_slot_bytes = keccak256_concat(key_bytes, u256_to_bytes32(self.__slot));
        let value_slot = bytes32_to_u256(value_slot_bytes);
        Value::from(value_slot)
    }
}

impl<Key, Value> Index<Key> for Mapping<Value>
    where
        Key: Into<CommonKey>,
        Value: From<U256>,
{
    type Output = Value;

    fn index(&self, key: Key) -> &Self::Output {
        // Compute the value on the fly and return a reference to it
        let value = self.get_item(key);
        Box::leak(Box::new(value))
    }
}

impl From<u64> for CommonKey {
    fn from(key: u64) -> Self {
        let mut bytes = [0u8; 32];
        bytes[0..std::mem::size_of::<u64>()].copy_from_slice(&key.to_be_bytes());
        CommonKey(bytes)
    }
}


#[derive(Debug)]
struct CommonKey([u8; 32]);

// Implement methods for CommonKey
impl CommonKey {
    fn to_key_bytes(&self) -> [u8; 32] {
        self.0
    }
}


#[derive(Debug)]
struct MyStruct {
    __slot: U256,
    myUint0: Primitive,
    myUint1: Primitive,
}


impl From<U256> for MyStruct {
    fn from(u: U256) -> Self {
        MyStruct{__slot: u, myUint0: Primitive::from(u), myUint1: Primitive::from(u)}  // Use the conversion from U256 to u64
    }
}

struct MyContract {
    plainUint112: Primitive,
    plainUint32: Primitive,
    myMapping: Mapping<Primitive>,
    myStruct: MyStruct,
}

fn main() {
    // let storage_layout: layout::StorageLayout = serde_json::from_str(data::STORAGE_STRING).expect("JSON was not well-formatted");
    // println!("{:#?}", storage_layout);
    // println!("{:#?}", storage_layout.types["t_uint32"]);
    // println!("{:#?}", storage_layout.types["t_array(t_uint112)10_storage"]);
    // println!("{:#?}", storage_layout.members[0]);

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
    //         },
    //     },
    // };
    // println!("{:#?}", contract.plainUint32.slot());

    let mapping = Mapping::<MyStruct>::new(U256::from(137));
    println!("Value: {:?}", mapping[10u64]);  // Outputs: Value: MyStruct(137)
}


//
// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// struct MemberType {
//     encoding: String,
//     label: String,
//     #[serde(deserialize_with = "string_to_u64")]
//     number_of_bytes: u64,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     base: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     key: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     value: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     members: Option<Vec<Member>>,
// }
