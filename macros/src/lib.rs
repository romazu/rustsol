extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro]
pub fn generate_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let members = if let syn::Data::Struct(data) = input.data {
        data.fields
    } else {
        panic!("Expected a struct");
    };

    let expanded = quote! {
        pub struct #name {
            #members
        }
    };

    TokenStream::from(expanded)
}
