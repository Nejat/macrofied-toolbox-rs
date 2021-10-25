#[cfg(feature = "trace")]
use std::fmt::{self, Display, Formatter};

use crate::common::{Message, OnFail, OnSuccess, WhenExpr};
use crate::result::parts::Parts;

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
        let ok = format_option(&self.ok);
        let debug = format_option(&self.debug);
        let error = format_option(&self.error);

        return write!(
            fmt, "{{\n  when: {},\n  ok: {},\n  debug: {},\n  error: {}\n}}",
            self.when, ok, debug, error
        );

        //noinspection DuplicatedCode
        fn format_option<T: Display>(source: &Option<T>) -> String {
            if let Some(some) = source { format!("{}", some) } else { "None".to_string() }
        }
    }
}

impl ResultMacro {
    fn definition(&self) -> Parts {
        (if self.ok.is_some() { Parts::OK } else { Parts::NONE }) |
            (if self.debug.is_some() { Parts::DEBUG } else { Parts::NONE }) |
            (if self.error.is_some() { Parts::ERROR } else { Parts::NONE })
    }
}
