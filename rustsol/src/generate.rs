use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{Item, parse_str};
use crate::layout::NestedType;
use crate::utils::ceil_div;


pub fn generate_structs(nested_types: Vec<NestedType>) -> TokenStream {
    let mut nested_struct_definitions: Vec<TokenStream> = Vec::new();
    let mut nested_struct_implementations: Vec<TokenStream> = Vec::new();

    for nested_type in nested_types {
        match &nested_type {
            NestedType::Struct { label, members, number_of_bytes } => {
                let struct_name = Ident::new(&label, proc_macro2::Span::call_site());
                let number_of_bytes_literal = syn::LitInt::new(&number_of_bytes.to_string(), proc_macro2::Span::call_site());

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
                    let member_slot_literal = syn::LitInt::new(&member_def.member_info.slot.to_string(), proc_macro2::Span::call_site());
                    let member_offset_literal = syn::LitInt::new(&member_def.member_info.offset.to_string(), proc_macro2::Span::call_site());
                    quote! {
                        #field_name: #field_type::from_position(slot + U256::from(#member_slot_literal), #member_offset_literal)
                    }
                }).collect();

                let set_slots_getter_fields: Vec<TokenStream> = members.iter().map(|member_def| {
                    let field_name = Ident::new(&member_def.member_info.label, proc_macro2::Span::call_site());
                    quote! {
                        self.#field_name.set_slots_getter(getter.clone())
                    }
                }).collect();

                let value_struct_ident = Ident::new(&format!("{}Value", struct_name), proc_macro2::Span::call_site());

                let value_fields: Vec<TokenStream> = members.iter().map(|member_def| {
                    let field_name = Ident::new(&member_def.member_info.label, proc_macro2::Span::call_site());
                    let value_type = get_value_type_name(&member_def.type_def);
                    quote! {
                        pub #field_name: #value_type
                    }
                }).collect();

                let struct_size_slots = ceil_div(nested_type.size() as usize, 32) as u64;
                let struct_size_slots_literal = syn::LitInt::new(&struct_size_slots.to_string(), proc_macro2::Span::call_site());

                let get_value_from_slots_content_fields: Vec<TokenStream> = members.iter().map(|member_def| {
                    let field_name = Ident::new(&member_def.member_info.label, proc_macro2::Span::call_site());
                    let size_slots = ceil_div(member_def.type_def.size() as usize, 32) as u64;
                    let member_slot_start_literal = syn::LitInt::new(&member_def.member_info.slot.to_string(), proc_macro2::Span::call_site());
                    let member_slot_end_literal = syn::LitInt::new(&(member_def.member_info.slot + size_slots).to_string(), proc_macro2::Span::call_site());
                    quote! {
                        #field_name: self.#field_name.get_value_from_slots_content(slot_values[#member_slot_start_literal..#member_slot_end_literal].to_vec())?
                    }
                }).collect();

                let struct_definition = quote! {
                    #[derive(Derivative)]
                    #[derivative(Debug)]
                    pub struct #struct_name {
                        __slot: U256,
                        #[derivative(Debug = "ignore")]
                        __slots_getter: Option<Arc<dyn SlotsGetter>>,
                        #(#fields),*
                    }
                };
                nested_struct_definitions.push(struct_definition);

                let struct_implementation = quote! {
                    impl #struct_name {
                        pub fn new() -> Self {
                            Self::from_position(U256::ZERO, 0)
                        }
                        pub fn from_position(slot: U256, offset: usize) -> Self {
                            Self {
                                __slot: slot,
                                __slots_getter: None,
                                #(#default_fields),*
                            }
                        }
                        pub fn slot(&self) -> U256 {
                            self.__slot
                        }
                        pub fn position(&self) -> (U256, usize, usize) {
                            (self.__slot, 0, #number_of_bytes_literal)
                        }
                        pub fn get_value(&self) -> Result<<Self as Value>::ValueType, String> {
                            let getter = self.__slots_getter.as_ref().expect("No slots getter");
                            let slot_values = getter.get_slots(self.__slot, #struct_size_slots_literal)
                                .map_err(|err| format!("Failed to get slot values: {}", err))?;
                            self.get_value_from_slots_content(slot_values)
                        }
                    }
                    impl Position for #struct_name {
                        fn from_position(slot: U256, offset: usize) -> Self {
                            Self::from_position(slot, offset)
                        }
                        fn size() -> usize {
                            #number_of_bytes_literal
                        }
                    }
                    impl SlotsGetterSetter for #struct_name {
                        fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
                            self.__slots_getter = Some(getter.clone());
                            #(#set_slots_getter_fields);*
                        }
                    }
                    #[derive(Debug)]
                    pub struct #value_struct_ident {
                        #(#value_fields),*
                    }
                    impl Value for #struct_name {
                        type ValueType = #value_struct_ident;
                        fn get_value_from_slots_content(&self, slot_values: Vec<U256>) -> Result<Self::ValueType, String> {
                            let getter = self.__slots_getter.as_ref().expect("No slots getter");
                            Ok(#value_struct_ident {
                                #(#get_value_from_slots_content_fields),*
                            })
                        }
                    }
                };
                nested_struct_implementations.push(struct_implementation);
            }
            // All other types are general and are predefined in separate files, see imports_definition.
            _ => {}
        }
    }

    let global_attribites = quote! {
        #![allow(unused_imports, non_snake_case, unused, dead_code)]
    };
    let imports_definition_items: Vec<Item> = vec![
        parse_str("use std::sync::Arc;").expect("Failed to parse"),
        parse_str("use rustsol::types::Derivative;").expect("Failed to parse"),
        parse_str("use rustsol::types::{Position, SlotsGetter, SlotsGetterSetter, Value};").expect("Failed to parse"),
        parse_str("use rustsol::types::{Primitive, Bytes, Address, Mapping, DynamicArray, StaticArray};").expect("Failed to parse"),
        parse_str("use rustsol::types::{PrimitiveKey, BytesKey, AddressKey};").expect("Failed to parse"),
        parse_str("use alloy_primitives;").expect("Failed to parse"),
        parse_str("use alloy_primitives::U256;").expect("Failed to parse"),
    ];
    let imports_definition: TokenStream = imports_definition_items.into_iter().map(|item| item.into_token_stream()).collect();

    let generated_tokens = quote! {
        #global_attribites
        #imports_definition
        #(#nested_struct_definitions)*
        #(#nested_struct_implementations)*
    };

    generated_tokens
}

fn get_type_name(nested_type: &NestedType) -> TokenStream {
    let type_name = match nested_type {
        NestedType::Primitive { .. } => "Primitive",
        NestedType::Bytes => "Bytes",
        NestedType::Address => "Address",
        NestedType::Mapping { .. } => "Mapping",
        NestedType::Struct { label, .. } => label,
        NestedType::DynamicArray { .. } => "DynamicArray",
        NestedType::StaticArray { .. } => "StaticArray",
    };
    let ident = syn::Ident::new(type_name, proc_macro2::Span::call_site());
    quote! { #ident }
}

fn get_value_type_name(nested_type: &NestedType) -> TokenStream {
    let type_name = match nested_type {
        NestedType::Primitive { number_of_bytes: _, native_type } => {
            let native_type_ident = syn::Ident::new(native_type, proc_macro2::Span::call_site());
            quote! {#native_type_ident}
        }
        NestedType::Bytes => quote! {Vec<u8>},
        NestedType::Address => quote! {alloy_primitives::Address},
        NestedType::Mapping { .. } => {
            // Mapping value type is Mapping itself.
            get_nested_type(nested_type)
        }
        NestedType::Struct { label, .. } => {
            let name = format!("{}Value", label);
            let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
            quote! { #ident }
        }
        NestedType::DynamicArray { value } => {
            let element_value_type_name = get_value_type_name(value);
            quote! {Vec<#element_value_type_name>}
        }
        NestedType::StaticArray { value, .. } => {
            let element_value_type_name = get_value_type_name(value);
            quote! {Vec<#element_value_type_name>}
        }
    };
    type_name
    // let ident = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
}

fn get_nested_type(nested_type: &NestedType) -> TokenStream {
    match nested_type {
        NestedType::Primitive { number_of_bytes, native_type } => {
            // Avoid verbose definitions like Primitive<32u64>.
            let number_of_bytes_literal = syn::LitInt::new(&number_of_bytes.to_string(), proc_macro2::Span::call_site());
            let native_type_ident = syn::Ident::new(native_type, proc_macro2::Span::call_site());
            quote! { Primitive<#number_of_bytes_literal, #native_type_ident> }
        }
        NestedType::Bytes => quote! { Bytes },
        NestedType::Address => quote! { Address },
        NestedType::Mapping { key, value } => {
            let value_type = get_nested_type(value);
            let key_type_for_mapping = match key.as_ref() {
                NestedType::Primitive { .. } => quote! { PrimitiveKey },
                NestedType::Bytes => quote! { BytesKey },
                NestedType::Address => quote! { AddressKey },
                _ => panic!("Bad key type")
            };

            quote! { Mapping<#key_type_for_mapping, #value_type> }
        }
        NestedType::Struct { label, .. } => {
            let label_ident = syn::Ident::new(label, proc_macro2::Span::call_site());
            quote! { #label_ident }
        }
        NestedType::DynamicArray { value } => {
            let value_type = get_nested_type(value);
            quote! { DynamicArray<#value_type> }
        }
        NestedType::StaticArray { value, number_of_bytes } => {
            let value_type = get_nested_type(value);
            let number_of_bytes_literal = syn::LitInt::new(&number_of_bytes.to_string(), proc_macro2::Span::call_site());
            quote! { StaticArray<#number_of_bytes_literal, #value_type> }
        }
    }
}
