#[cfg(feature = "trace")]
use std::fmt::{self, Display, Formatter};

use proc_macro2::TokenStream;
#[cfg(feature = "trace")]
use quote::ToTokens;
use syn::{Expr, Lit};

pub struct Message {
    pub args: Option<Vec<Expr>>,
    pub captured: Option<Capture>,
    pub fmt: Lit,
}

#[cfg(feature = "trace")]
impl Display for Message {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        let args = if let Some(args) = &self.args {
            format!(
                "{:?}",
                args.iter().map(|v| format!("{}", v.to_token_stream())).collect::<Vec<String>>()
            )
        } else {
            "None".to_string()
        };
        let captured = if let Some(captured) = &self.captured {
            format!("{}", captured)
        } else {
            "None".to_string()
        };

        write!(
            fmt, "{{ args: {}, captured: {}, fmt: {} }}",
            args, captured, self.fmt.to_token_stream()
        )
    }
}

impl Message {
    pub fn build_message(&self) -> TokenStream {
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

#[derive(Clone)]
pub struct Capture {
    pub identifier: String,
    pub mutable: bool,
    pub reference: bool,
}

impl<T: Into<String>> From<T> for Capture {
    fn from(identifier: T) -> Self {
        Self {
            identifier: identifier.into(),
            mutable: false,
            reference: false
        }
    }
}

#[cfg(feature = "trace")]
impl Display for Capture {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(
            fmt, "{{ identifier: {:?}, mutable: {}, reference: {} }}",
            self.identifier, self.mutable, self.reference
        )
    }
}

pub struct OnExpr {
    pub captured: Option<Capture>,
    pub expr: Expr,
}

#[cfg(feature = "trace")]
impl Display for OnExpr {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        let captured = if let Some(captured) = &self.captured {
            format!("{}", captured)
        } else {
            "None".to_string()
        };

        write!(
            fmt, "{{ captured: {}, expr: \"{}\" }}", captured, self.expr.to_token_stream()
        )
    }
}

pub struct OnFail {
    pub expr: Option<OnExpr>,
    pub message: Option<Message>,
}

#[cfg(feature = "trace")]
impl Display for OnFail {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        let expr = if let Some(expr) = &self.expr {
            format!("{}", expr)
        } else {
            "None".to_string()
        };
        let message = if let Some(message) = &self.message {
            format!("{}", message)
        } else {
            "None".to_string()
        };

        write!(fmt, "{{ expr: {}, message: {} }}", expr, message)
    }
}

pub enum OnSuccess {
    Expr(OnExpr),
    Message(Message),
}

#[cfg(feature = "trace")]
impl Display for OnSuccess {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expr(expr) =>
                write!(fmt, "{}", expr),
            Self::Message(message) =>
                write!(fmt, "{}", message)
        }
    }
}

pub struct WhenExpr {
    pub expr: Expr,
    pub tried: bool,
    pub ok_when: bool
}

#[cfg(feature = "trace")]
impl Display for WhenExpr {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(
            fmt, "{{ expr: \"{}\", ok_when: {}, tried: {} }}",
            self.expr.to_token_stream(), self.ok_when, self.tried
        )
    }
}
