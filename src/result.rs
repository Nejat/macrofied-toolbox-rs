use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};

pub struct ResultMacro;

impl Parse for ResultMacro {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        Ok(Self)
    }
}

impl ToTokens for ResultMacro {
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        todo!()
    }
}