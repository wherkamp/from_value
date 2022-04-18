use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};
use quote::quote;


/// Example of user-defined [derive mode macro][1]
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-mode-macros
#[proc_macro_derive(FromValue)]
pub fn derive_from_value(stream: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(stream as DeriveInput);
    let ident = &input.ident;

    let tokens = quote! {


        impl core::convert::TryFrom<serde_json::Value> for #ident{
                type Error = serde_json::Error;

            fn try_from(value: Value) -> Result<Self, Self::Error> {
                return serde_json::from_value(value);
            }
        }
    };

    tokens.into()
}

#[proc_macro_derive(IntoValue)]
pub fn derive_into_value(stream: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(stream as DeriveInput);
    let ident = &input.ident;

    let tokens = quote! {

        impl core::convert::TryInto<serde_json::Value> for #ident{
            type Error = serde_json::Error;
            fn try_into(self) -> Result<serde_json::Value, Self::Error> {
                return serde_json::to_value(&self);
            }
        }
    };

    tokens.into()
}
