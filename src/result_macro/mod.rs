#[cfg(feature = "trace")]
use std::fmt::{self, Display, Formatter};

use crate::common::{Message, OnFail, OnSuccess, WhenExpr};
#[cfg(feature = "trace")]
use crate::display;
use crate::result_macro::parts::Parts;

mod parse;
mod parts;
mod tokenize;

pub struct ResultMacro {
    when: WhenExpr,
    ok: Option<OnSuccess>,
    debug: Option<Message>,
    error: Option<OnFail>,
}

#[cfg(feature = "trace")]
impl Display for ResultMacro {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        let ok = display(&self.ok);
        let debug = display(&self.debug);
        let error = display(&self.error);

        write!(
            fmt,
            "result! {{\n  when: {ok},\n  ok: {debug},\n  debug: {error},\n  error: {}\n}}",
            self.when
        )
    }
}

impl ResultMacro {
    fn definition(&self) -> Parts {
        (if self.ok.is_some() { Parts::OK } else { Parts::NONE }) |
            (if self.debug.is_some() { Parts::DEBUG } else { Parts::NONE }) |
            (if self.error.is_some() { Parts::ERROR } else { Parts::NONE })
    }
}
