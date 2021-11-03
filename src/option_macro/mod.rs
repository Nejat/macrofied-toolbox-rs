#[cfg(feature = "trace")]
use std::fmt::{self, Display, Formatter};

use crate::common::{Message, OnFail, OnSuccess, WhenExpr};
use crate::option_macro::parts::Parts;

mod parse;
mod parts;
mod tokenize;

pub struct OptionMacro {
    when: WhenExpr,
    some: Option<OnSuccess>,
    debug: Option<Message>,
    none: Option<OnFail>,
}

#[cfg(feature = "trace")]
impl Display for OptionMacro {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        let some = format_option(&self.some);
        let debug = format_option(&self.debug);
        let none = format_option(&self.none);

        return write!(
            fmt, "{{\n  when: {},\n  some: {},\n  debug: {},\n  none: {}\n}}",
            self.when, some, debug, none
        );

        //noinspection DuplicatedCode
        fn format_option<T: Display>(source: &Option<T>) -> String {
            if let Some(some) = source { format!("{}", some) } else { "None".to_string() }
        }
    }
}

impl OptionMacro {
    fn definition(&self) -> Parts {
        (if self.some.is_some() { Parts::SOME } else { Parts::NOTHING }) |
            (if self.debug.is_some() { Parts::DEBUG } else { Parts::NOTHING }) |
            (if self.none.is_some() { Parts::NONE } else { Parts::NOTHING })
    }
}
