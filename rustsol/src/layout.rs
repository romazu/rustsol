use serde::{Deserialize, Deserializer};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Deserialize, Default)]
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
pub struct StorageLayout {
    #[serde(rename = "storage")]
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
    Bytes,
    Primitive,
    Mapping {
        // Box is needed to avoid problems with recursive definition of NestedType
        key: Box<NestedType>,
        value: Box<NestedType>,
    },
    Struct {
        label: String,
        members: Vec<MemberDef>,
    },
}

impl NestedType {
    fn to_string(&self) -> String {
        match self {
            NestedType::Bytes => "Bytes".to_string(),
            NestedType::Primitive => "Primitive".to_string(),
            NestedType::Mapping{key, value } => {
                format!("Mapping<{}, {}>", key.to_string(), value.to_string())
            }
            NestedType::Struct { label, members } => {
                format!("Struct {}", label)
            }
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
    pub fn traverse(&self) -> Vec<NestedType> {
        self.traverse_struct("Contract".into(), &self.members)
    }

    fn traverse_struct(&self, label: String, members: &Vec<Member>) -> Vec<NestedType> {
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
        let main_struct = NestedType::Struct {
            label: label,
            members: member_defs,
        };
        nested_types.insert(0, main_struct);
        nested_types
    }

    fn traverse_type(&self, type_name: &str) -> Option<NestedType> {
        if let Some(ty) = self.types.get(type_name) {
            match ty {
                MemberType::Bytes { .. } => Some(NestedType::Bytes),
                MemberType::Primitive { .. } => Some(NestedType::Primitive),
                MemberType::Mapping { key, value, .. } => {
                    let key_type = match self.traverse_type(key) {
                        Some(NestedType::Primitive) => Some(NestedType::Primitive),
                        Some(NestedType::Bytes) => Some(NestedType::Bytes),
                        _ => panic!("Key type must be Primitive or Bytes"),
                    };

                    let value_type = self.traverse_type(value);

                    if let Some(valid_key_type) = key_type {
                        if let Some(valid_value_type) = value_type {
                            Some(NestedType::Mapping{
                                key: Box::new(valid_key_type),
                                value: Box::new(valid_value_type),
                            })
                        } else {
                            // panic!("Value type could not be resolved for type: {}", value);
                            println!("Value type could not be resolved for type: {}", value);
                            None
                        }
                    } else {
                        None
                    }
                }
                MemberType::Struct { label, members, .. } => {
                    let struct_name = get_struct_name(label);
                    let nested_types = self.traverse_struct(struct_name, members);
                    Some(nested_types[0].clone())
                }
                _ => None,
            }
        } else {
            None
        }
    }

    fn collect_unique_types(&self, nested_type: &NestedType, nested_types: &mut Vec<NestedType>, unique_representations: &mut HashSet<String>) {
        let repr = nested_type.to_string();
        if unique_representations.insert(repr.clone()) {
            nested_types.push(nested_type.clone());
        }
        if let NestedType::Mapping{key, value} = nested_type {
            self.collect_unique_types(key, nested_types, unique_representations);
            self.collect_unique_types(value, nested_types, unique_representations);
        }
        // if let NestedType::Struct{label, members} = nested_type {
        //     self.collect_unique_types(key, nested_types, unique_representations);
        //     self.collect_unique_types(value, nested_types, unique_representations);
        // }
    }
}
