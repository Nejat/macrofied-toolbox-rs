#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]
// ==============================================================
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::items_after_statements)]
// ==============================================================
#![doc(html_root_url = "https://docs.rs/macrofied-toolbox/0.1.0")]

//! This library provides the macroification of some fundamental rust boilerplate
//! patterns, i.e. option, result, etc.

#[cfg(feature = "result")]
mod result;

#[cfg(test)]
mod tests;