use proc_macro2::{Ident, Span};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};

use crate::common::{Capture, OnExpr, OnFail, OnSuccess, trace_parsed, trace_source};
use crate::common::parse::{
    parse_expression, parse_expression_debug, parse_expression_success,
    parse_expression_when, parse_message, parse_optional_semicolon, utils,
};
use crate::result_macro::ResultMacro;

mod kw {
    custom_keyword![error];
    custom_keyword![ok];
}

const ERR_IDENT: &str = "err";
const ERROR_SECTION: &str = "error";
pub const OK_IDENT: &str = "ok";
const OK_SECTION: &str = "ok";

impl Parse for ResultMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        return trace_parsed(parse(trace_source(input)));

        #[inline]
        fn parse(input: ParseStream) -> syn::Result<ResultMacro> {
            let when = parse_expression_when(input, kw::ok)?;

            Ok(ResultMacro {
                ok: if when.ok_when {
                    let ok = Ident::new(OK_IDENT, Span::call_site());

                    Some(OnSuccess::Expr(OnExpr {
                        captured: Some(Capture::from(OK_IDENT)),
                        expr: parse_quote! { #ok },
                    }))
                } else {
                    parse_expression_success(
                        input, kw::ok, OK_SECTION, Some(Capture::from(OK_IDENT))
                    )?
                },
                when,
                debug: parse_expression_debug(input, &Some(Capture::from(ERR_IDENT)))?,
                error: parse_expression_error(input)?,
            })
        }
    }
}

fn parse_expression_error(input: ParseStream) -> syn::Result<Option<OnFail>> {
    if input.peek(Token![@]) && input.peek2(kw::error) {
        <Token![@]>::parse(input)?;
        <kw::error>::parse(input)?;

        let message = parse_message(input, ERROR_SECTION, &Some(Capture::from(ERR_IDENT))).ok();

        let expr = if !input.is_empty() && !input.peek(Token![@]) {
            if message.is_some() {
                <Token![;]>::parse(input)?;
            }

            if input.is_empty() {
                None
            } else {
                let expr = parse_expression(input, ERROR_SECTION)?;
                let captured = if utils::search_for_ident(expr.to_token_stream(), ERR_IDENT) {
                    Some(Capture::from(ERR_IDENT))
                } else {
                    None
                };

                Some(OnExpr { captured, expr })
            }
        } else {
            parse_optional_semicolon(input)?;

            None
        };

        Ok(Some(OnFail { expr, message }))
    } else {
        Ok(None)
    }
}
