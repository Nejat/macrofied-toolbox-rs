use syn::parse::{Parse, ParseStream};

use crate::common::Capture;
use crate::common::parse::{parse_debug, parse_failed, parse_successful, parse_when};
use crate::common::tracing::{trace_parsed, trace_source};
use crate::result_macro::ResultMacro;

mod kw {
    custom_keyword![error];
    custom_keyword![ok];
}

const ERR_IDENT: &str = "err";
const ERROR_SECTION: &str = "error";
const OK_IDENT: &str = "ok";
const OK_SECTION: &str = "ok";

impl Parse for ResultMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        return trace_parsed(parse(trace_source(input)));

        #[inline]
        fn parse(input: ParseStream) -> syn::Result<ResultMacro> {
            let when = parse_when(input, kw::ok)?;
            let err_capture = Some(Capture::from(ERR_IDENT));

            Ok(ResultMacro {
                ok: parse_successful(input, &when, OK_IDENT, OK_SECTION, kw::ok)?,
                when,
                debug: parse_debug(input, &err_capture)?,
                error: parse_failed(input, kw::error, ERROR_SECTION, err_capture)?,
            })
        }
    }
}
