use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;

use crate::common::{Message, OnFail, OnSuccess, WhenExpr};
use crate::result::parse::OK_IDENT;
use crate::result::parts::Parts;
use crate::result::ResultMacro;

impl ToTokens for ResultMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let when = &self.when;

        match self.definition() {
            #[cfg(not(all(debug_assertions, feature = "result-debug")))]
            Parts::OK |
            Parts::OK_DEBUG =>
                branch_only_ok(tokens, when, self.ok.as_ref().unwrap()),
            #[cfg(all(debug_assertions, feature = "result-debug"))]
            Parts::OK =>
                branch_only_ok(tokens, when, self.ok.as_ref().unwrap()),
            #[cfg(not(all(debug_assertions, feature = "result-debug")))]
            Parts::DEBUG =>
                branch_only_error(tokens, when, || (None, TokenStream::new())),
            #[cfg(all(debug_assertions, feature = "result-debug"))]
            Parts::DEBUG =>
                branch_only_error(
                    tokens, when, || build_message(self.debug.as_ref().unwrap()),
                ),
            Parts::ERROR =>
                branch_only_error(tokens, when, || build_on_error(self.error.as_ref().unwrap())),
            #[cfg(all(debug_assertions, feature = "result-debug"))]
            Parts::OK_DEBUG =>
                branch_ok_or_error(
                    tokens, when, self.ok.as_ref().unwrap(),
                    || build_message(self.debug.as_ref().unwrap()),
                ),
            Parts::OK_ERROR =>
                branch_ok_or_error(
                    tokens, when, self.ok.as_ref().unwrap(),
                    || build_on_error(self.error.as_ref().unwrap()),
                ),
            Parts::OK_DEBUG_ERROR =>
                branch_ok_or_error(
                    tokens, when, self.ok.as_ref().unwrap(), || build_debugged_error(self),
                ),
            Parts::DEBUG_ERROR =>
                branch_only_error(tokens, when, || build_debugged_error(self)),
            _ => unimplemented!("{:?} is not supported", self.definition())
        }

        #[cfg(feature = "trace")]
        println!("EXPANSION: {}", tokens);
    }
}

fn branch_ok_or_error(
    tokens: &mut TokenStream, when: &WhenExpr, ok: &OnSuccess,
    build_error: impl Fn() -> (Option<String>, TokenStream),
) {
    let when_expr = &when.expr;
    let (ok_branch, on_ok) = build_on_ok(ok);
    let (captured, on_error) = build_error();
    let error_branch = if captured.is_some() || when.tried {
        quote! { Err(err) }
    } else {
        quote! { Err(_) }
    };
    let tried = if when.tried { quote! { return Err(err); } } else { TokenStream::new() };

    tokens.extend(
        quote! {
            match #when_expr {
                #ok_branch => { #on_ok }
                #error_branch => { #on_error; #tried }
            }
        }
    );
}

fn branch_only_error(
    tokens: &mut TokenStream, when: &WhenExpr,
    build_error: impl Fn() -> (Option<String>, TokenStream),
) {
    let when_expr = &when.expr;
    let (captured, on_error) = build_error();
    let when_tried = if when.tried { quote! { return Err(err); } } else { TokenStream::new() };

    if captured.is_some() || when.tried {
        tokens.extend(quote! { if let Err(err) = #when_expr { #on_error; #when_tried } });
    } else {
        tokens.extend(quote! { if #when_expr.is_err() { #on_error; } });
    }
}

fn branch_only_ok(tokens: &mut TokenStream, when: &WhenExpr, ok: &OnSuccess) {
    let when_expr = &when.expr;
    let (captured, on_ok) = match ok {
        OnSuccess::Expr(expr) =>
            (&expr.captured, expr.expr.to_token_stream()),
        OnSuccess::Message(message) => {
            let message_fmt = message.build_message();
            let ok_message = quote! { println!(#message_fmt); };

            (&message.captured, ok_message)
        }
    };

    if when.tried {
        let ok_branch = build_ok_branch(captured);

        tokens.extend(
            quote! {
                match #when_expr {
                    #ok_branch => { #on_ok }
                    Err(err) => { return Err(err); }
                }
            }
        );
    } else {
        match captured {
            Some(captured) => {
                let captured = Ident::new(captured, Span::call_site());

                tokens.extend(quote! { if let Ok(#captured) = #when_expr { #on_ok } });
            }
            None =>
                tokens.extend(quote! { if #when_expr.is_ok() { #on_ok } }),
        }
    }
}

fn build_debugged_error(result_macro: &ResultMacro) -> (Option<String>, TokenStream) {
    cfg_if! {
        if #[cfg(all(debug_assertions, feature = "result-debug"))] {
            build_on_debug_and_on_error(
                result_macro.debug.as_ref().unwrap(), result_macro.error.as_ref().unwrap(),
            )
        } else {
            build_on_error(result_macro.error.as_ref().unwrap())
        }
    }
}

fn build_message(message: &Message) -> (Option<String>, TokenStream) {
    let message_fmt = message.build_message();
    let error_message = quote! { eprintln!(#message_fmt); };

    (message.captured.clone(), error_message)
}

fn build_ok_branch(captured: &Option<String>) -> TokenStream {
    let capture = match &captured {
        None => Ident::new(OK_IDENT, Span::call_site()),
        Some(capture) => Ident::new(capture, Span::call_site())
    };

    if captured.is_some() { quote! { Ok(#capture) } } else { quote! { Ok(_) } }
}

#[cfg(all(debug_assertions, feature = "result-debug"))]
fn build_on_debug_and_on_error(debug: &Message, error: &OnFail) -> (Option<String>, TokenStream) {
    let (captured_err, on_error) = build_on_error(error);
    let (captured_dbg, on_debug) = build_message(debug);

    (captured_dbg.and(captured_err), quote! { #on_debug  #on_error; })
}

fn build_on_error(error: &OnFail) -> (Option<String>, TokenStream) {
    let mut on_error = TokenStream::new();
    let mut captured = None;

    if error.message.is_some() {
        let (captured_error, message_error) = build_message(error.message.as_ref().unwrap());

        captured = captured_error;

        on_error.extend(message_error);
    }

    if let Some(expr) = &error.expr {
        let error_expr = &expr.expr;

        if captured.is_none() && expr.captured.is_some() {
            captured = expr.captured.clone();
        }

        on_error.extend(quote! { #error_expr });
    }

    (captured, on_error)
}

fn build_on_ok(ok: &OnSuccess) -> (TokenStream, TokenStream) {
    let (ok_captured, on_ok) = match ok {
        OnSuccess::Message(message) => {
            let ok_message = message.build_message();
            (message.captured.clone(), quote! { println!(#ok_message); })
        }
        OnSuccess::Expr(expr) => (expr.captured.clone(), expr.expr.to_token_stream())
    };

    let ok_branch = build_ok_branch(&ok_captured);

    (ok_branch, on_ok)
}
