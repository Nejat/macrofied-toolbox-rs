#[cfg(all(debug_assertions, feature = "trace"))]
use std::fmt::Display;

pub use models::{Message, OnExpr, OnFail, OnSuccess, WhenExpr};

mod models;
pub mod parse;

#[cfg(all(debug_assertions, feature = "trace"))]
#[inline]
pub fn trace_expansion<T: Display>(traced: T) -> T {
    println!("EXPANSION: {}", traced);

    traced
}

#[cfg(not(all(debug_assertions, feature = "trace")))]
#[inline]
pub const fn trace_expansion<T>(traced: T) -> T { traced }

#[cfg(all(debug_assertions, feature = "trace"))]
#[inline]
pub fn trace_parsed<T: Display, E: Display>(traced: Result<T, E>) -> Result<T, E> {
    match &traced {
        Ok(ok) =>
            println!("PARSED: {}", ok),
        Err(err) =>
            println!("PARSE-ERR: {}", err)
    }

    traced
}

#[cfg(not(all(debug_assertions, feature = "trace")))]
#[inline]
pub const fn trace_parsed<T>(traced: T) -> T { traced }
