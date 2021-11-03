use syn::parse::{Parse, ParseStream};

use crate::common::{OnExpr, OnFail};
use crate::common::parse::{
    parse_expression, parse_expression_debug, parse_expression_success,
    parse_expression_when, parse_message, parse_optional_semicolon,
};
use crate::option_macro::OptionMacro;

mod kw {
    custom_keyword![none];
    custom_keyword![some];
}

const NONE_SECTION: &str = "none";
pub const SOME_IDENT: &str = "some";
const SOME_SECTION: &str = "some";

impl Parse for OptionMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let option_macro = Self {
            when: parse_expression_when(input)?,
            some: parse_expression_success(
                input, kw::some, SOME_SECTION, Some(SOME_IDENT.to_string()),
            )?,
            debug: parse_expression_debug(input, &None)?,
            none: parse_expression_none(input)?,
        };

        #[cfg(feature = "trace")]
        println!("OPTION: {}", option_macro);

        Ok(option_macro)
    }
}

fn parse_expression_none(input: ParseStream) -> syn::Result<Option<OnFail>> {
    if input.peek(Token![@]) && input.peek2(kw::none) {
        <Token![@]>::parse(input)?;
        <kw::none>::parse(input)?;

        let message = parse_message(input, NONE_SECTION, &None).ok();

        let expr = if !input.is_empty() && !input.peek(Token![@]) {
            if message.is_some() {
                <Token![;]>::parse(input)?;
            }

            if input.is_empty() {
                None
            } else {
                let expr = parse_expression(input, NONE_SECTION)?;

                Some(OnExpr { captured: None, expr })
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
