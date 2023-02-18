#[cfg(feature = "trace")]
use std::fmt::{self, Display, Formatter};

use crate::common::{Message, OnFail, OnSuccess, WhenExpr};
#[cfg(feature = "trace")]
use crate::display;
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
        let some = display(&self.some);
        let debug = display(&self.debug);
        let none = display(&self.none);

        write!(
            fmt, "option! {{\n  when: {},\n  some: {some},\n  debug: {debug},\n  none: {none}\n}}",
            self.when
        )
    }
}

impl OptionMacro {
    fn definition(&self) -> Parts {
        (if self.some.is_some() { Parts::SOME } else { Parts::NOTHING }) |
            (if self.debug.is_some() { Parts::DEBUG } else { Parts::NOTHING }) |
            (if self.none.is_some() { Parts::NONE } else { Parts::NOTHING })
    }
}
