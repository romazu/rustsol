use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub ast_id: u64,
    pub contract: String,
    pub label: String,
    pub offset: u64,
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
    StaticArray {
        base: String,
        label: String,
        number_of_bytes: u64,
    },
    DynamicArray {
        base: String,
        label: String,
        number_of_bytes: u64,
    },
    Mapping {
        key: String,
        value: String,
        label: String,
        number_of_bytes: u64,
    },
}

#[derive(Debug, Deserialize)]
pub struct StorageLayout {
    pub members: Vec<Member>,
    pub types: HashMap<String, MemberType>,
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