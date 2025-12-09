extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use std::error;
use syn::{Data, DeriveInput, parse_macro_input};

#[proc_macro_derive(AsStr)]
pub fn derive_as_str(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;

    let variants =match ast.data {
        Data::Enum(e) => e.variants,
        _ => {syn::Error::new_spanned(name, "AsStr can only be derived for enums")
            .to_compile_error()
            .into()},
    }

    let expand = quote! {}
}
