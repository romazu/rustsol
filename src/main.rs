use std::marker::PhantomData;
use std::ops::Index;
use primitive_types::U256;

mod layout;

mod data;
mod keccak;

trait Location {
    fn slot(&self) -> U256;
    fn offset(&self) -> U256;
}

struct Primitive {
    __slot: U256,
    __offset: U256,
}

impl Location for Primitive {
    fn slot(&self) -> U256 {
        self.__slot
    }

    fn offset(&self) -> U256 {
        self.__offset
    }
}

struct Mapping<Key, Value> {
    __slot: U256,
    __marker: PhantomData<(Key, Value)>,
}

impl Mapping<Key, Value> {
    fn new(slot: U256) -> Self {
        Self {
            __slot: slot,
            __marker: PhantomData,
        }
    }

    fn get_slot_for_key(&self, key: &Key) -> U256 {
        keccak::compute_mapping_slot(key, self.__slot)
    }

    fn get_item(&self, key: &Key) -> Value
        where
            Value: From<U256>,
    {
        Value::from(self.__slot)
    }
}

impl<Key, Value> Index<Key> for Mapping<Key, Value>
    where
        Value: From<U256>,
{
    type Output = Value;

    fn index(&self, key: Key) -> &Self::Output {
        // Compute the value on the fly
        let value = self.get_item(&key);
        Box::leak(Box::new(value))
    }
}

struct MyStruct {
    __slot: U256,
    myUint0: Primitive,
    myUint1: Primitive,
}

struct MyContract {
    plainUint112: Primitive,
    plainUint32: Primitive,
    myMapping: Mapping<Primitive, Primitive>,
    myStruct: MyStruct,
}

fn main() {
    // let storage_layout: layout::StorageLayout = serde_json::from_str(data::STORAGE_STRING).expect("JSON was not well-formatted");
    // println!("{:#?}", storage_layout);
    // println!("{:#?}", storage_layout.types["t_uint32"]);
    // println!("{:#?}", storage_layout.types["t_array(t_uint112)10_storage"]);
    // println!("{:#?}", storage_layout.members[0]);

    // Initialize the fields for MyContract
    let contract = MyContract {
        plainUint112: Primitive {
            __slot: U256::from(0),
            __offset: U256::from(0),
        },
        plainUint32: Primitive {
            __slot: U256::from(0),
            __offset: U256::from(14),
        },
        myMapping: Mapping::new(U256::from(1)),
        myStruct: MyStruct {
            __slot: U256::from(2),
            myUint0: Primitive {
                __slot: U256::from(2),
                __offset: U256::from(0),
            },
            myUint1: Primitive {
                __slot: U256::from(3),
                __offset: U256::from(0),
            },
        },
    };

    println!("{:#?}", contract.plainUint32.slot());
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
