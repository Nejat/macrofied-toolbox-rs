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


//! This library provides an ergonomic experience of adding debugging messages to rust's
//! `Result<T,E>` and `Option<T>` patterns
//!
//! Just like the [`cli-toolbox`](https://crates.io/crates/cli-toolbox) crate, that the debug logic
//! is based on, this is not a logging alternative; it's intended to produce debugging output to be
//! used during application development.
//!
//! Although the macros were designed to make debugging more ergonomic, they include variations that
//! do not include debugging to provide coding consistency, so you have the option use the same syntax
//! consistently throughout your crate.
//!
//! ### `Result<T,E>`
//! ```no_run
//! ```
//!
//! ### `Option<T>`
//!
//! ```text
//! .
//! .
//! .
//! ```
//!
//! \* _the macros are automatically generated with custom build scripts, including their_ `docs` and `tests`


#[cfg(feature = "result")]
mod result;

#[cfg(test)]
mod tests;