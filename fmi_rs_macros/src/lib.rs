use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Fmi1Ffi)]
pub fn derive_f1(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! { fmi_rs::generate_fmi1_ffi!(#name); };
    TokenStream::from(expanded)
}

#[proc_macro_derive(Fmi2Ffi)]
pub fn derive_f2(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! { fmi_rs::generate_fmi2_ffi!(#name); };
    TokenStream::from(expanded)
}

#[proc_macro_derive(Fmi3Ffi)]
pub fn derive_f3(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! { fmi_rs::generate_fmi3_ffi!(#name); };
    TokenStream::from(expanded)
}
