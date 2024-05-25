use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{Item, parse_str};
use crate::generate;
use crate::layout::{MemberDef, NestedType};


pub fn generate_structs(nested_types: Vec<NestedType>) -> TokenStream {
    let mut nested_struct_definitions: Vec<TokenStream> = Vec::new();

    for nested_type in nested_types {
        match nested_type {
            NestedType::Struct { label, members } => {
                let struct_name = Ident::new(&label, proc_macro2::Span::call_site());

                let fields: Vec<TokenStream> = members.iter().map(|member_def| {
                    let field_name = Ident::new(&member_def.member_info.label, proc_macro2::Span::call_site());
                    let field_type = get_nested_type(&member_def.type_def);
                    quote! {
                        pub #field_name: #field_type
                    }
                }).collect();

                let default_fields: Vec<TokenStream> = members.iter().map(|member_def| {
                    let field_name = Ident::new(&member_def.member_info.label, proc_macro2::Span::call_site());
                    let field_type = get_type_name(&member_def.type_def);
                    let member_slot = member_def.member_info.slot;
                    let member_offset = member_def.member_info.offset;
                    quote! {
                        // #field_name: #field_type
                        #field_name: #field_type::from_position(slot + #member_slot, U256::from(#member_offset))
                        // #field_name: Default::default()
                    }
                }).collect();

                println!("{}", default_fields[0].to_string());

                let struct_definition = quote! {
                    #[derive(Debug)]
                    #[allow(non_snake_case)]
                    pub struct #struct_name {
                        __slot: U256,
                        #(#fields),*
                    }
                    impl #struct_name {
                        pub fn new_from_position(slot: U256, offset: U256) -> Self {
                            Self {
                                __slot: slot,
                                #(#default_fields),*
                            }
                        }
                        pub fn slot(&self) -> U256 {
                            self.__slot
                        }
                    }
                    impl FromPosition for #struct_name {
                        fn from_position(slot: U256, offset: U256) -> Self {
                            Self::new_from_position(slot, offset)
                        }
                    }
                };
                nested_struct_definitions.push(struct_definition)
            }
            // All other types are general and are predefined in separate files, see imports_definition.
            _ => {}
        }
    }

    let imports_definition_items: Vec<Item> = vec![
        parse_str("use rustsol::types::{Primitive, Bytes, Mapping, PrimitiveKey, BytesKey, FromPosition};").expect("Failed to parse"),
        parse_str("use primitive_types::{U256};").expect("Failed to parse"),
    ];
    let imports_definition: TokenStream = imports_definition_items.into_iter().map(|item| item.into_token_stream()).collect();

    let generated_tokens = quote! {
        #imports_definition
        #(#nested_struct_definitions)*
    };

    generated_tokens
}

fn get_type_name(nested_type: &NestedType) -> TokenStream {
    let type_name = match nested_type {
        NestedType::Bytes => {"Bytes"}
        NestedType::Primitive => {"Primitive"}
        NestedType::Mapping { .. } => {"Mapping"}
        NestedType::Struct { label, .. } => {label}
    };
    let ident = syn::Ident::new(type_name, proc_macro2::Span::call_site());
    quote! { #ident }
}

fn get_nested_type(nested_type: &NestedType) -> TokenStream {
    match nested_type {
        NestedType::Primitive => quote! { Primitive },
        NestedType::Bytes => quote! { Bytes },
        NestedType::Mapping { key, value } => {
            let key_type = get_nested_type(key);
            let value_type = get_nested_type(value);
            let key_type_for_mapping = match key.as_ref() {
                NestedType::Primitive => quote! { PrimitiveKey },
                NestedType::Bytes => quote! { BytesKey },
                _ => panic!("Bad key type")
            };

            quote! { Mapping<#key_type_for_mapping, #value_type> }
        }
        NestedType::Struct { label, members } => {
            let label_ident = syn::Ident::new(label, proc_macro2::Span::call_site());
            quote! { #label_ident }
        }
    }
}


// fn get_nested_type(nested_type: &NestedType) -> TokenStream {
//     let type_name = get_nested_type_string(nested_type);
//     let ident = syn::Ident::new(type_name, proc_macro2::Span::call_site());
//     quote! {#ident}
// }
//
// fn get_nested_type_string(nested_type: &NestedType) -> &str {
//     match nested_type {
//         NestedType::Primitive => "Primitive",
//         NestedType::Bytes => "Bytes",
//         NestedType::Mapping { key, value } => {
//             let key_type = get_nested_type_string(key);
//             let value_type = get_nested_type_string(value);
//             let key_type_for_mapping = match key.as_ref() {
//                 NestedType::Primitive => "PrimitiveKey",
//                 NestedType::Bytes => "BytesKey",
//                 _ => panic!("Bad key type")
//             };
//
//             // quote! { Mapping<#key_type_for_mapping, #value_type> }
//             quote! { Mapping<#key_type_for_mapping, #value_type> }
//         }
//         NestedType::Struct { label, members } => {
//             label
//         }
//     }
// }
