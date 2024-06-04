use serde::{Deserialize, Deserializer};
use std::collections::{HashMap, HashSet};
use crate::utils::ceil_div;

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub ast_id: u64,
    pub contract: String,
    pub label: String,
    pub offset: usize,
    #[serde(deserialize_with = "string_to_u64")]
    pub slot: u64,
    #[serde(rename = "type")]
    pub type_name: String,
}

#[derive(Debug)]
pub enum MemberType {
    Primitive {
        label: String,
        number_of_bytes: u64,
    },
    Bytes {
        label: String,
        number_of_bytes: u64,
    },
    Address,
    Mapping {
        key: String,
        value: String,
        label: String,
        number_of_bytes: u64,
    },
    DynamicArray {
        base: String,
        label: String,
        number_of_bytes: u64,
    },
    StaticArray {
        base: String,
        label: String,
        number_of_bytes: u64,
    },
    Struct {
        label: String,
        members: Vec<Member>,
        number_of_bytes: u64,
    },
}

#[derive(Debug, Deserialize)]
pub struct SolcOutput {
    // contracts: file_path: contract_name: storageLayout -> StorageLayout
    pub contracts: HashMap<String, HashMap<String, SolcOutputContract>>,

    // other fields
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolcOutputContract {
    pub storage_layout: StorageLayout,
}

#[derive(Debug, Deserialize)]
pub struct StorageLayout {
    #[serde(rename = "storage")]
    pub members: Vec<Member>,
    pub types: Option<HashMap<String, MemberType>>,
}

impl<'de> Deserialize<'de> for MemberType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct IntermediateMemberType {
            encoding: String,
            label: String,
            #[serde(deserialize_with = "string_to_u64")]
            number_of_bytes: u64,
            base: Option<String>,
            key: Option<String>,
            value: Option<String>,
            members: Option<Vec<Member>>,
        }

        let intermediate = IntermediateMemberType::deserialize(deserializer)?;
        match intermediate.encoding.as_str() {
            "inplace" => {
                if let Some(base) = intermediate.base {
                    Ok(MemberType::StaticArray {
                        base,
                        label: intermediate.label,
                        number_of_bytes: intermediate.number_of_bytes,
                    })
                } else if let Some(members) = intermediate.members {
                    Ok(MemberType::Struct {
                        label: intermediate.label,
                        members: members,
                        number_of_bytes: intermediate.number_of_bytes,
                    })
                } else if intermediate.label == "address" {
                    Ok(MemberType::Address)
                } else if intermediate.label.starts_with("contract ") {
                    Ok(MemberType::Address)
                } else {
                    Ok(MemberType::Primitive {
                        label: intermediate.label,
                        number_of_bytes: intermediate.number_of_bytes,
                    })
                }
            }
            "bytes" => Ok(MemberType::Bytes {
                label: intermediate.label,
                number_of_bytes: intermediate.number_of_bytes,
            }),
            "dynamic_array" => Ok(MemberType::DynamicArray {
                base: intermediate.base.unwrap(),
                label: intermediate.label,
                number_of_bytes: intermediate.number_of_bytes,
            }),
            "mapping" => Ok(MemberType::Mapping {
                key: intermediate.key.unwrap(),
                value: intermediate.value.unwrap(),
                label: intermediate.label,
                number_of_bytes: intermediate.number_of_bytes,
            }),
            _ => Err(serde::de::Error::custom("unknown encoding")),
        }
    }
}

fn string_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
{
    let slot_str = String::deserialize(deserializer)?;
    slot_str.parse::<u64>().map_err(serde::de::Error::custom)
}


#[derive(Debug, Clone)]
pub enum NestedType {
    Primitive {
        number_of_bytes: u64,
        native_type: String,
    },
    Bytes,
    Address,
    Mapping {
        // Box is needed to avoid problems with recursive definition of NestedType
        key: Box<NestedType>,
        value: Box<NestedType>,
    },
    Struct {
        label: String,
        members: Vec<MemberDef>,
        number_of_bytes: u64,
    },
    DynamicArray {
        value: Box<NestedType>,
    },
    StaticArray {
        value: Box<NestedType>,
        number_of_bytes: u64,
    },
}

impl NestedType {
    fn to_string(&self) -> String {
        match self {
            NestedType::Primitive { number_of_bytes, native_type } => {
                format!("Primitive<{}, {}>", number_of_bytes.to_string(), native_type)
            }
            NestedType::Bytes => "Bytes".to_string(),
            NestedType::Address => "Address".to_string(),
            NestedType::Mapping { key, value } => {
                format!("Mapping<{}, {}>", key.to_string(), value.to_string())
            }
            NestedType::Struct { label, .. } => {
                format!("Struct<{}>", label)
            }
            NestedType::DynamicArray { value } => {
                format!("DynamicArray<{}>", value.to_string())
            }
            NestedType::StaticArray { value, number_of_bytes } => {
                format!("StaticArray<{}, {}>", number_of_bytes.to_string(), value.to_string())
            }
        }
    }

    pub fn size(&self) -> usize {
        match self {
            NestedType::Primitive { number_of_bytes, .. } => { *number_of_bytes as usize }
            NestedType::Bytes => { 32 }
            NestedType::Address => { 20 }
            NestedType::Mapping { .. } => { 32 }
            NestedType::Struct { label: _, members: _, number_of_bytes } => { *number_of_bytes as usize }
            NestedType::DynamicArray { .. } => { 32 }
            NestedType::StaticArray { value: _, number_of_bytes } => { *number_of_bytes as usize }
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemberDef {
    pub type_def: NestedType,
    pub member_info: Member,
}

fn get_struct_name(s: &str) -> String {
    let result = s.replace("struct ", "");
    let result = result.replace(".", "");
    result
}

impl StorageLayout {
    pub fn traverse(&self, name: String) -> Vec<NestedType> {
        self.traverse_struct(name, &self.members, 0)
    }

    fn traverse_struct(&self, label: String, members: &Vec<Member>, size: u64) -> Vec<NestedType> {
        let mut member_defs = Vec::new();
        let mut nested_types = Vec::new();
        let mut unique_representations = HashSet::new();

        for member in members {
            if let Some(nested_type) = self.traverse_type(&member.type_name) {
                member_defs.push(MemberDef {
                    type_def: nested_type.clone(),
                    member_info: member.clone(),
                });
                self.collect_unique_types(&nested_type, &mut nested_types, &mut unique_representations);
            }
        }
        let last_member = &member_defs[member_defs.len() - 1];
        let last_member_size_slots = ceil_div(last_member.type_def.size(), 32);
        let estimated_struct_size = (last_member.member_info.slot + last_member_size_slots as u64) * 32;
        if size != 0 {
            assert_eq!(estimated_struct_size, size, "struct size differs from total size of its members")
        }
        let main_struct = NestedType::Struct {
            label: label,
            members: member_defs,
            number_of_bytes: estimated_struct_size,
        };
        nested_types.insert(0, main_struct);
        nested_types
    }

    fn traverse_type(&self, type_name: &str) -> Option<NestedType> {
        let type_def = self.types.as_ref().expect("Types map is None")
            .get(type_name).expect("No type definition found for {}");
        match type_def {
            MemberType::Primitive { label, number_of_bytes } => {
                let native_type = primitive_label_to_native_type(label).into();
                Some(NestedType::Primitive { number_of_bytes: *number_of_bytes, native_type: native_type })
            }
            MemberType::Bytes { .. } => Some(NestedType::Bytes),
            MemberType::Address => Some(NestedType::Address),
            MemberType::Mapping { key, value, .. } => {
                let key_type = match self.traverse_type(key) {
                    Some(NestedType::Primitive { number_of_bytes, native_type }) => Some(NestedType::Primitive { number_of_bytes, native_type }),
                    Some(NestedType::Bytes) => Some(NestedType::Bytes),
                    Some(NestedType::Address) => Some(NestedType::Address),
                    _ => panic!("Key type must be Primitive or Bytes"),
                };

                let value_type = self.traverse_type(value);

                if let Some(valid_key_type) = key_type {
                    if let Some(valid_value_type) = value_type {
                        Some(NestedType::Mapping {
                            key: Box::new(valid_key_type),
                            value: Box::new(valid_value_type),
                        })
                    } else {
                        panic!("Mapping value type could not be resolved for type: {}", value);
                    }
                } else {
                    panic!("Key type could not be resolved for type: {}", value);
                }
            }
            MemberType::Struct { label, members, number_of_bytes } => {
                let struct_name = get_struct_name(label);
                let nested_types = self.traverse_struct(struct_name, members, *number_of_bytes);
                Some(nested_types[0].clone())
            }
            MemberType::DynamicArray { base, .. } => {
                let value_type = self.traverse_type(base);
                if let Some(valid_value_type) = value_type {
                    Some(NestedType::DynamicArray {
                        value: Box::new(valid_value_type),
                    })
                } else {
                    // panic!("Value type could not be resolved for type: {}", value);
                    println!("Value type could not be resolved for type: {}", base);
                    None
                }
            }
            MemberType::StaticArray { base, label: _, number_of_bytes } => {
                let value_type = self.traverse_type(base);
                if let Some(valid_value_type) = value_type {
                    Some(NestedType::StaticArray {
                        value: Box::new(valid_value_type),
                        number_of_bytes: *number_of_bytes,
                    })
                } else {
                    // panic!("Value type could not be resolved for type: {}", value);
                    println!("Value type could not be resolved for type: {}", base);
                    None
                }
            }
        }
    }

    fn collect_unique_types(&self, nested_type: &NestedType, nested_types: &mut Vec<NestedType>, unique_representations: &mut HashSet<String>) {
        let repr = nested_type.to_string();
        if unique_representations.insert(repr.clone()) {
            nested_types.push(nested_type.clone());
        } else {
            return;
        }
        match nested_type {
            NestedType::Primitive { .. } => {
                // This is a leaf type. Do nothing.
            }
            NestedType::Bytes => {
                // This is a leaf type. Do nothing.
            }
            NestedType::Address => {
                // This is a leaf type. Do nothing.
            }
            NestedType::Mapping { key, value } => {
                self.collect_unique_types(key, nested_types, unique_representations);
                self.collect_unique_types(value, nested_types, unique_representations);
            }
            NestedType::Struct { label: _, members, .. } => {
                for member in members {
                    self.collect_unique_types(&member.type_def, nested_types, unique_representations);
                }
            }
            NestedType::DynamicArray { value } => {
                self.collect_unique_types(value, nested_types, unique_representations);
            }
            NestedType::StaticArray { value, .. } => {
                self.collect_unique_types(value, nested_types, unique_representations);
            }
        }
    }
}

fn primitive_label_to_native_type(label: &str) -> &str {
    // TODO: Use programmatic conversion instead of direct mapping.
    // TODO: Check for invalid type for Primitive.
    match label {
        "bool" => "bool",
        "int" => "I256",
        "int8" => "i8",
        "int16" => "i16",
        "int24" => "i32",
        "int32" => "i32",
        "int40" => "i64",
        "int48" => "i64",
        "int56" => "i64",
        "int64" => "i64",
        "int72" => "i128",
        "int80" => "i128",
        "int88" => "i128",
        "int96" => "i128",
        "int104" => "i128",
        "int112" => "i128",
        "int120" => "i128",
        "int128" => "i128",
        "int136" => "I256",
        "int144" => "I256",
        "int152" => "I256",
        "int160" => "I256",
        "int168" => "I256",
        "int176" => "I256",
        "int184" => "I256",
        "int192" => "I256",
        "int200" => "I256",
        "int208" => "I256",
        "int216" => "I256",
        "int224" => "I256",
        "int232" => "I256",
        "int240" => "I256",
        "int248" => "I256",
        "int256" => "I256",
        "uint" => "U256",
        "uint8" => "u8",
        "uint16" => "u16",
        "uint24" => "u32",
        "uint32" => "u32",
        "uint40" => "u64",
        "uint48" => "u64",
        "uint56" => "u64",
        "uint64" => "u64",
        "uint72" => "u128",
        "uint80" => "u128",
        "uint88" => "u128",
        "uint96" => "u128",
        "uint104" => "u128",
        "uint112" => "u128",
        "uint120" => "u128",
        "uint128" => "u128",
        "uint136" => "U256",
        "uint144" => "U256",
        "uint152" => "U256",
        "uint160" => "U256",
        "uint168" => "U256",
        "uint176" => "U256",
        "uint184" => "U256",
        "uint192" => "U256",
        "uint200" => "U256",
        "uint208" => "U256",
        "uint216" => "U256",
        "uint224" => "U256",
        "uint232" => "U256",
        "uint240" => "U256",
        "uint248" => "U256",
        "uint256" => "U256",
        // "address" => "U256",
        // "bytes" => "U256",
        // "bytes1" => "U256",
        // "bytes2" => "U256",
        // "bytes3" => "U256",
        // "bytes4" => "U256",
        // "bytes5" => "U256",
        // "bytes6" => "U256",
        // "bytes7" => "U256",
        // "bytes8" => "U256",
        // "bytes9" => "U256",
        // "bytes10" => "U256",
        // "bytes11" => "U256",
        // "bytes12" => "U256",
        // "bytes13" => "U256",
        // "bytes14" => "U256",
        // "bytes15" => "U256",
        // "bytes16" => "U256",
        // "bytes17" => "U256",
        // "bytes18" => "U256",
        // "bytes19" => "U256",
        // "bytes20" => "U256",
        // "bytes21" => "U256",
        // "bytes22" => "U256",
        // "bytes23" => "U256",
        // "bytes24" => "U256",
        // "bytes25" => "U256",
        // "bytes26" => "U256",
        // "bytes27" => "U256",
        // "bytes28" => "U256",
        // "bytes29" => "U256",
        // "bytes30" => "U256",
        // "bytes31" => "U256",
        // "bytes32" => "U256",
        // "string" => "U256",
        _ => "U256",
        // _ => panic!("unsupported native primitive type: {}", label),
    }


    // match label {
    //     "bool" => "bool",
    //     "int8" => "i8",
    //     "int16" => "i16",
    //     "int24" => "i32",
    //     "int32" => "i32",
    //     "int40" => "i64",
    //     "int48" => "i64",
    //     "int56" => "i64",
    //     "int64" => "i64",
    //     "uint8" => "u8",
    //     "uint16" => "u16",
    //     "uint24" => "u32",
    //     "uint32" => "u32",
    //     "uint40" => "u64",
    //     "uint48" => "u64",
    //     "uint56" => "u64",
    //     "uint64" => "u64",
    //     _ => "U256",
    //     // _ => panic!("unsupported native primitive type: {}", label),
    // }

    // match label {
    //     "bool" => "bool",
    //     "int8" => "i64",
    //     "int16" => "i64",
    //     "int24" => "i64",
    //     "int32" => "i64",
    //     "int40" => "i64",
    //     "int48" => "i64",
    //     "int56" => "i64",
    //     "int64" => "i64",
    //     "uint8" => "u64",
    //     "uint16" => "u64",
    //     "uint24" => "u64",
    //     "uint32" => "u64",
    //     "uint40" => "u64",
    //     "uint48" => "u64",
    //     "uint56" => "u64",
    //     "uint64" => "u64",
    //     _ => "U256",
    //     // _ => panic!("unsupported native primitive type: {}", label),
    // }
}

