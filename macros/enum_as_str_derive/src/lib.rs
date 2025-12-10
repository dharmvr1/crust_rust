use proc_macro::TokenStream;
use quote::quote;
use syn::{DataEnum, DeriveInput, parse_macro_input};

#[proc_macro_derive(AsStr)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let enum_name = ast.ident;

    let variants = match ast.data {
        syn::Data::Enum(e) => e.variants,
        _ => {
            return syn::Error::new_spanned(enum_name, "only for enums")
                .into_compile_error()
                .into();
        }
    };

    let mut as_str_arr = Vec::new();
    let mut from_str_arr = Vec::new();

    for variant in variants {
        let v_ident = variant.ident;
        let v_str = v_ident.to_string();
        as_str_arr.push(quote! {
            #enum_name::#v_ident => #v_str
        });

        from_str_arr.push(quote! {
            #v_str => Ok(#enum_name::#v_ident)
        });
    }

    let expand = quote! {
        impl #enum_name{
            fn as_str(&self)-> &str{
                match self{


                #(#as_str_arr),*}
            }
        }


        impl std::fmt::Display for #enum_name{
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
        }


        impl std::str::FromStr for #enum_name{
             type Err =();
            fn from_str(s: &str) -> Result<Self, Self::Err>{
               
                 match s {
                    #(#from_str_arr),*,
                    _ => Err(())                 }
            }
        }
    };
   expand.into()

}
