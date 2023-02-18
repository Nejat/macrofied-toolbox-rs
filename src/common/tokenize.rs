use proc_macro2::{Ident, Span, TokenStream};

use crate::common::Capture;

pub fn build_captured(captured: &Option<Capture>) -> TokenStream {
    captured
        .as_ref()
        .map_or_else(
            || quote! { _ },
            |captured| {
                let captured_ident = Ident::new(&captured.identifier, Span::call_site());

                match (captured.reference, captured.mutable) {
                    (false, false) => quote! { #captured_ident },
                    (true, false) => quote! { &#captured_ident },
                    (true, true) => quote! { &mut #captured_ident },
                    (false, true) => quote! { mut #captured_ident },
                }
            },
        )
}
