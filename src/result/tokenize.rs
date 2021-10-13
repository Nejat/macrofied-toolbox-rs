use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::result::{Message, OnError, OnOk, ResultMacro, WhenExpr};
use crate::result::parts::Parts;

impl ToTokens for ResultMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let when = &self.when;
        let ok = self.ok.as_ref().unwrap();

        match self.definition() {
            Parts::OK =>
                branch_only_ok(tokens, when, ok),
            Parts::DEBUG => {
                cfg_if! {
                    if #[cfg(all(debug_assertions, feature = "result-debug"))] {
                        branch_only_error(
                            tokens, when, || build_message(self.debug.as_ref().unwrap())
                        )
                    } else {
                        branch_only_ok(tokens, when, ok)
                    }
                }
            }
            Parts::ERROR |
            Parts::ERR_EXPR |
            Parts::ERROR_ERR_EXPR =>
                branch_only_error(tokens, when,|| build_on_error(self.error.as_ref().unwrap())),
            Parts::OK_DEBUG => {
                cfg_if! {
                    if #[cfg(all(debug_assertions, feature = "result-debug"))] {
                        branch_ok_or_error(
                            tokens, when, ok, || build_message(self.debug.as_ref().unwrap())
                        )
                    } else {
                        branch_only_ok(tokens, when, ok)
                    }
                }
            }
            Parts::OK_ERROR |
            Parts::OK_ERR_EXPR |
            Parts::OK_ERROR_ERR_EXPR =>
                branch_ok_or_error(
                    tokens, when, ok, || build_on_error(self.error.as_ref().unwrap())
                ),
            Parts::OK_DEBUG_ERROR |
            Parts::OK_DEBUG_ERR_EXPR |
            Parts::OK_DEBUG_ERROR_ERR_EXPR =>
                branch_ok_or_error(tokens, when, ok, || build_debugged_error(self)),
            Parts::DEBUG_ERROR |
            Parts::DEBUG_ERR_EXPR |
            Parts::DEBUG_ERROR_ERR_EXPR =>
                branch_only_error(tokens, when, || build_debugged_error(self)),
            _ => unimplemented!("{:?} is not supported", self.definition())
        }
    }
}

fn branch_ok_or_error(
    tokens: &mut TokenStream, when: &WhenExpr, ok: &OnOk, build_error: impl Fn() -> (bool, TokenStream),
) {
    let when_expr = &when.expr;
    let (ok_branch, on_ok) = build_on_ok(ok);
    let (captured, on_error) = build_error();
    let error_branch = if captured || when.tried { quote! { Err(err) } } else { quote! { Err(_) } };
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
    tokens: &mut TokenStream, when: &WhenExpr, build_error: impl Fn() -> (bool, TokenStream),
) {
    let when_expr = &when.expr;
    let (captured, on_error) = build_error();

    if captured || when.tried {
        tokens.extend(quote! { if let Err(err) = #when_expr { #on_error; return Err(err); } });
    } else {
        tokens.extend(quote! { if #when_expr.is_err() { #on_error; } });
    }
}

fn branch_only_ok(tokens: &mut TokenStream, when: &WhenExpr, ok: &OnOk) {
    let when_expr = &when.expr;

    match ok {
        OnOk::Message(message) => {
            let message_fmt = message.build_message();
            let ok_message = quote! { println!(#message_fmt); };

            if message.captured {
                tokens.extend(quote! { if let Ok(ok) = #when_expr { #ok_message } });
            } else {
                tokens.extend(quote! { if #when_expr.is_ok() { #ok_message } });
            }
        }
        OnOk::Expr(expr) => {
            let on_ok = &expr.expr;

            if expr.captured {
                tokens.extend(quote! { if let Ok(ok) = #when_expr { #on_ok } });
            } else {
                tokens.extend(quote! { if #when_expr.is_ok() { #on_ok } });
            }
        }
    }
}

fn build_debugged_error(result_macro: &ResultMacro) -> (bool, TokenStream) {
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

fn build_message(message: &Message) -> (bool, TokenStream) {
    let message_fmt = message.build_message();
    let error_message = quote! { eprintln!(#message_fmt); };

    (message.captured, error_message)
}

#[cfg(all(debug_assertions, feature = "result-debug"))]
fn build_on_debug_and_on_error(debug: &Message, error: &OnError) -> (bool, TokenStream) {
    let (captured_err, on_error) = build_on_error(error);
    let (captured_dbg, on_debug) = build_message(debug);

    (captured_dbg || captured_err, quote! { #on_debug  #on_error; })
}

fn build_on_error(error: &OnError) -> (bool, TokenStream) {
    let mut on_error = TokenStream::new();
    let mut captured = false;

    if error.message.is_some() {
        let (captured_error, message_error) = build_message(error.message.as_ref().unwrap());

        captured = captured || captured_error;

        on_error.extend(message_error);
    }

    if let Some(expr) = &error.expr {
        let error_expr = &expr.expr;

        captured = captured || expr.captured;

        on_error.extend(quote! { #error_expr });
    }

    (captured, on_error)
}

fn build_on_ok(ok: &OnOk) -> (TokenStream, TokenStream) {
    let (ok_captured, on_ok) = match ok {
        OnOk::Message(message) => {
            let ok_message = message.build_message();
            (message.captured, quote! { println!(#ok_message); })
        }
        OnOk::Expr(expr) => (expr.captured, expr.expr.to_token_stream())
    };
    let ok_branch = if ok_captured { quote! { Ok(ok) } } else { quote! { Ok(_) } };

    (ok_branch, on_ok)
}
