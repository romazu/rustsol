use proc_macro2::{Ident, TokenStream};
use quote::quote;
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

                let struct_definition = quote! {
                    #[derive(Default)]
                    #[allow(non_snake_case)]
                    pub struct #struct_name {
                        __slot: [u8; 32],
                        #(#fields),*
                    }
                };
                nested_struct_definitions.push(struct_definition)
            }
            _ => {}
        }
    }

    let generated_tokens = quote! {
        #(#nested_struct_definitions)*
    };

    generated_tokens
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
