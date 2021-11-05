use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::common::{Capture, OnFail, OnSuccess, WhenExpr};
#[cfg(all(debug_assertions, feature = "result-debug"))]
use crate::common::Message;
use crate::common::tokenize::build_captured;
use crate::common::tracing::trace_expansion;
use crate::result_macro::parts::Parts;
use crate::result_macro::ResultMacro;

impl ToTokens for ResultMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(trace_expansion({
            let when = &self.when;

            match self.definition() {
                #[cfg(not(all(debug_assertions, feature = "result-debug")))]
                Parts::OK |
                Parts::OK_DEBUG =>
                    branch_only_ok(when, self.ok.as_ref().unwrap()),
                #[cfg(all(debug_assertions, feature = "result-debug"))]
                Parts::OK =>
                    branch_only_ok(when, self.ok.as_ref().unwrap()),
                #[cfg(not(all(debug_assertions, feature = "result-debug")))]
                Parts::DEBUG =>
                    branch_only_error(when, || (&None, TokenStream::new())),
                #[cfg(all(debug_assertions, feature = "result-debug"))]
                Parts::DEBUG =>
                    branch_only_error(when, || build_message_stdout(self.debug.as_ref().unwrap())),
                Parts::ERROR =>
                    branch_only_error(when, || build_on_error(self.error.as_ref().unwrap())),
                #[cfg(all(debug_assertions, feature = "result-debug"))]
                Parts::OK_DEBUG =>
                    branch_ok_or_error(
                        when, self.ok.as_ref().unwrap(),
                        || build_message_stdout(self.debug.as_ref().unwrap()),
                    ),
                Parts::OK_ERROR =>
                    branch_ok_or_error(
                        when, self.ok.as_ref().unwrap(),
                        || build_on_error(self.error.as_ref().unwrap()),
                    ),
                Parts::OK_DEBUG_ERROR =>
                    branch_ok_or_error(
                        when, self.ok.as_ref().unwrap(), || build_debugged_error(self),
                    ),
                Parts::DEBUG_ERROR =>
                    branch_only_error(when, || build_debugged_error(self)),
                _ => unimplemented!("{:?} is not supported", self.definition())
            }
        }));
    }
}

fn branch_ok_or_error<'a>(
    when: &'a WhenExpr, ok: &OnSuccess,
    build_error: impl Fn() -> (&'a Option<Capture>, TokenStream),
) -> TokenStream {
    let when_expr = &when.expr;
    let (ok_branch, on_ok) = build_on_ok(ok);
    let (captured, on_error) = build_error();
    let error_branch = if captured.is_some() || when.tried {
        quote! { Err(err) }
    } else {
        quote! { Err(_) }
    };
    let tried = if when.tried { quote! { ; return Err(err); } } else { TokenStream::new() };

    quote! {
        match #when_expr {
            #ok_branch => { #on_ok }
            #error_branch => { #on_error #tried }
        }
    }
}

fn branch_only_error<'a>(
    when: &'a WhenExpr, build_error: impl Fn() -> (&'a Option<Capture>, TokenStream),
) -> TokenStream {
    let when_expr = &when.expr;
    let (captured, on_error) = build_error();
    let when_tried = if when.tried { quote! { ; return Err(err); } } else { TokenStream::new() };

    if captured.is_some() || when.tried {
        quote! { if let Err(err) = #when_expr { #on_error #when_tried } }
    } else {
        quote! { if #when_expr.is_err() { #on_error } }
    }
}

fn branch_only_ok(when: &WhenExpr, ok: &OnSuccess) -> TokenStream {
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
        let ok_branch = build_branch_ok(captured);

        quote! {
            match #when_expr {
                #ok_branch => { #on_ok }
                Err(err) => { return Err(err); }
            }
        }
    } else if captured.is_some() {
        let captured = build_captured(captured);

        quote! { if let Ok(#captured) = #when_expr { #on_ok; } }
    } else {
        quote! { if #when_expr.is_ok() { #on_ok; } }
    }
}

fn build_branch_ok(captured: &Option<Capture>) -> TokenStream {
    let capture = build_captured(captured);

    quote! { Ok(#capture) }
}

fn build_debugged_error(result_macro: &ResultMacro) -> (&Option<Capture>, TokenStream) {
    cfg_if! {
        if #[cfg(all(debug_assertions, feature = "result-debug"))] {
            let (captured_err, on_error) = build_on_error(result_macro.error.as_ref().unwrap());
            let (captured_dbg, on_debug) = build_message_stdout(result_macro.debug.as_ref().unwrap());
            let captured = if captured_dbg.is_some() { captured_dbg } else { captured_err };

            (captured, quote! { #on_debug  #on_error })
        } else {
            build_on_error(result_macro.error.as_ref().unwrap())
        }
    }
}

#[cfg(all(debug_assertions, feature = "result-debug"))]
fn build_message_stdout(message: &Message) -> (&Option<Capture>, TokenStream) {
    let message_fmt = message.build_message();

    (&message.captured, quote! { println!(#message_fmt); })
}

fn build_on_error(error: &OnFail) -> (&Option<Capture>, TokenStream) {
    let mut on_error = TokenStream::new();
    let mut captured = &None;

    if error.message.is_some() {
        let (captured_error, message_error) = {
            let message_fmt = error.message.as_ref().unwrap().build_message();
            let error_message = quote! { eprintln!(#message_fmt); };

            (&error.message.as_ref().unwrap().captured, error_message)
        };

        captured = captured_error;

        on_error.extend(message_error);
    }

    if let Some(expr) = &error.expr {
        let error_expr = &expr.expr;

        if captured.is_none() && expr.captured.is_some() {
            captured = &expr.captured;
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

    let ok_branch = build_branch_ok(&ok_captured);

    (ok_branch, on_ok)
}
