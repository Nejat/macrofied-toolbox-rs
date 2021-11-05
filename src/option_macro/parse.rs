use syn::parse::{Parse, ParseStream};

use crate::common::{trace_parsed, trace_source};
use crate::common::parse::{parse_debug, parse_failed, parse_successful, parse_when};
use crate::option_macro::OptionMacro;

mod kw {
    custom_keyword![none];
    custom_keyword![some];
}

const NONE_SECTION: &str = "none";
const SOME_IDENT: &str = "some";
const SOME_SECTION: &str = "some";

impl Parse for OptionMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        return trace_parsed(parse(trace_source(input)));

        #[inline]
        fn parse(input: ParseStream) -> syn::Result<OptionMacro> {
            let when = parse_when(input, kw::some)?;

            Ok(OptionMacro {
                some: parse_successful(input, &when, SOME_IDENT, SOME_SECTION, kw::some)?,
                when,
                debug: parse_debug(input, &None)?,
                none: parse_failed(input, kw::none, NONE_SECTION, None)?,
            })
        }
    }
}
