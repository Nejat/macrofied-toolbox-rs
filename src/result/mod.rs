use proc_macro2::TokenStream;
use syn::{Expr, Lit};

use crate::result::parts::Parts;

mod kw;
mod parse;
mod parts;
mod tokenize;

pub(crate) struct ResultMacro {
    when: WhenExpr,
    ok: Option<OnOk>,
    debug: Option<Message>,
    error: Option<OnError>,
}

struct WhenExpr {
    expr: Expr,
    tried: bool,
}

enum OnOk {
    Expr(OnExpr),
    Message(Message),
}

struct OnExpr {
    captured: bool,
    expr: Expr,
}

struct Message {
    args: Option<Vec<Expr>>,
    captured: bool,
    fmt: Lit,
}

impl Message {
    fn build_message(&self) -> TokenStream {
        let mut tokens = TokenStream::new();

        let fmt = &self.fmt;

        tokens.extend(quote! { #fmt });

        if let Some(args) = &self.args {
            for arg in args {
                tokens.extend(quote! { , #arg });
            }
        }

        tokens
    }
}

struct OnError {
    message: Option<Message>,
    expr: Option<OnExpr>,
}

impl ResultMacro {
    fn definition(&self) -> Parts {
        (if self.ok.is_some() { Parts::OK } else { Parts::NONE }) |
            (if self.debug.is_some() { Parts::DEBUG } else { Parts::NONE }) |
            (if self.error.is_some() && self.error.as_ref().unwrap().message.is_some() { Parts::ERROR } else { Parts::NONE }) |
            (if self.error.is_some() && self.error.as_ref().unwrap().expr.is_some() { Parts::ERR_EXPR } else { Parts::NONE })
    }
}
