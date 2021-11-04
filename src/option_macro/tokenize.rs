use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;

use crate::common::{OnFail, OnSuccess, trace_expansion, WhenExpr};
#[cfg(all(debug_assertions, feature = "option-debug"))]
use crate::common::Message;
use crate::option_macro::OptionMacro;
use crate::option_macro::parse::SOME_IDENT;
use crate::option_macro::parts::Parts;

impl ToTokens for OptionMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(trace_expansion({
            let when = &self.when;

            match self.definition() {
                #[cfg(not(all(debug_assertions, feature = "option-debug")))]
                Parts::SOME |
                Parts::SOME_DEBUG =>
                    branch_only_some(when, self.some.as_ref().unwrap()),
                #[cfg(all(debug_assertions, feature = "option-debug"))]
                Parts::SOME =>
                    branch_only_some(when, self.some.as_ref().unwrap()),
                #[cfg(not(all(debug_assertions, feature = "option-debug")))]
                Parts::DEBUG =>
                    branch_only_none(when, TokenStream::new),
                #[cfg(all(debug_assertions, feature = "option-debug"))]
                Parts::DEBUG =>
                    branch_only_none(when, || build_message_stdout(self.debug.as_ref().unwrap())),
                Parts::NONE =>
                    branch_only_none(when, || build_on_none(self.none.as_ref().unwrap())),
                #[cfg(all(debug_assertions, feature = "option-debug"))]
                Parts::SOME_DEBUG =>
                    branch_some_or_none(
                        when, self.some.as_ref().unwrap(),
                        || build_message_stdout(self.debug.as_ref().unwrap()),
                    ),
                Parts::SOME_NONE =>
                    branch_some_or_none(
                        when, self.some.as_ref().unwrap(),
                        || build_on_none(self.none.as_ref().unwrap()),
                    ),
                Parts::SOME_DEBUG_NONE =>
                    branch_some_or_none(
                        when, self.some.as_ref().unwrap(), || build_debugged_none(self),
                    ),
                Parts::DEBUG_NONE =>
                    branch_only_none(when, || build_debugged_none(self)),
                _ => unimplemented!("{:?} is not supported", self.definition())
            }
        }));
    }
}

fn branch_some_or_none(
    when: &WhenExpr, some: &OnSuccess, build_none: impl Fn() -> TokenStream
) -> TokenStream {
    let when_expr = &when.expr;
    let (some_branch, on_some) = build_on_some(some);
    let on_none = build_none();
    let tried = if when.tried { quote! { ; return None; } } else { TokenStream::new() };

    quote! {
        match #when_expr {
            #some_branch => { #on_some }
            None => { #on_none #tried }
        }
    }
}

fn branch_only_none(when: &WhenExpr, build_none: impl Fn() -> TokenStream) -> TokenStream {
    let when_expr = &when.expr;
    let on_none = build_none();

    if when.tried {
        quote! { if let None = #when_expr { #on_none; return None; } }
    } else {
        quote! { if #when_expr.is_none() { #on_none; } }
    }
}

fn branch_only_some(when: &WhenExpr, some: &OnSuccess) -> TokenStream {
    let when_expr = &when.expr;
    let (captured, on_some) = match some {
        OnSuccess::Expr(expr) =>
            (&expr.captured, expr.expr.to_token_stream()),
        OnSuccess::Message(message) => {
            let message_fmt = message.build_message();
            let some_message = quote! { println!(#message_fmt); };

            (&message.captured, some_message)
        }
    };

    if when.tried {
        let some_branch = build_some_branch(captured);

        quote! {
            match #when_expr {
                #some_branch => { #on_some }
                None => { return None; }
            }
        }
    } else {
        match captured {
            Some(captured) => {
                let captured = Ident::new(captured, Span::call_site());

                quote! { if let Some(#captured) = #when_expr { #on_some; } }
            }
            None =>
                quote! { if #when_expr.is_some() { #on_some; } }
        }
    }
}

fn build_debugged_none(result_macro: &OptionMacro) -> TokenStream {
    cfg_if! {
        if #[cfg(all(debug_assertions, feature = "option-debug"))] {
            build_on_debug_and_on_none(
                result_macro.debug.as_ref().unwrap(), result_macro.none.as_ref().unwrap(),
            )
        } else {
            build_on_none(result_macro.none.as_ref().unwrap())
        }
    }
}

#[cfg(all(debug_assertions, feature = "option-debug"))]
fn build_message_stdout(message: &Message) -> TokenStream {
    let message_fmt = message.build_message();

    quote! { println!(#message_fmt); }
}

fn build_some_branch(captured: &Option<String>) -> TokenStream {
    let capture = match &captured {
        None => Ident::new(SOME_IDENT, Span::call_site()),
        Some(capture) => Ident::new(capture, Span::call_site())
    };

    if captured.is_some() { quote! { Some(#capture) } } else { quote! { Some(_) } }
}

#[cfg(all(debug_assertions, feature = "option-debug"))]
fn build_on_debug_and_on_none(debug: &Message, error: &OnFail) -> TokenStream {
    let on_none = build_on_none(error);
    let on_debug = build_message_stdout(debug);

    quote! { #on_debug  #on_none }
}

fn build_on_none(error: &OnFail) -> TokenStream {
    let mut on_none = TokenStream::new();

    if error.message.is_some() {
        let message_fmt = error.message.as_ref().unwrap().build_message();
        let message_none = quote! { eprintln!(#message_fmt); };

        on_none.extend(message_none);
    }

    if let Some(expr) = &error.expr {
        let none_expr = &expr.expr;

        on_none.extend(quote! { #none_expr });
    }

    on_none
}

fn build_on_some(some: &OnSuccess) -> (TokenStream, TokenStream) {
    let (some_captured, on_some) = match some {
        OnSuccess::Message(message) => {
            let some_message = message.build_message();
            (message.captured.clone(), quote! { println!(#some_message); })
        }
        OnSuccess::Expr(expr) => (expr.captured.clone(), expr.expr.to_token_stream())
    };
    let some_branch = build_some_branch(&some_captured);

    (some_branch, on_some)
}
