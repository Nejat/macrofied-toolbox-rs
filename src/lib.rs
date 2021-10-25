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
//! resembles, this is not a logging alternative; it's intended to produce debugging output to be
//! used during application development.
//!
//! Although the macros were designed to make debugging more ergonomic, they include variations that
//! do not include debugging to provide coding consistency, so you have the option to use the same syntax
//! consistently throughout your project.
//!
//! ### `Result<T,E>`
//! ```no_run
//! # use std::fs::File;
//! # use std::io;
//! # use std::io::{BufWriter, Write};
//! use macrofied_toolbox::result;
//!
//! fn main() -> io::Result<()> {
//!     let file_name = "foo.txt";
//!
//!     // attempts to create a file
//!     result! {
//!         // * if you use the try "?" operator, the result! macro will
//!         //   still output debug or error before returning the Result::Err
//!         @when  File::create(file_name)?;
//!         // if the file is successfully created, write some content
//!         @ok    (file) => {
//!             let mut out = BufWriter::new(file);
//!
//!             writeln!(out, "some content")?;
//!             writeln!(out, "some more content")?;
//!         }
//!         // if an exception occurs output debug message to stdout
//!         @debug "problem creating file: {:?} - {}", file_name, err
//!
//!         // * debug messages are conditionally compiled
//!         //   and do not output anything in release builds
//!         // * "err" contains the Result::Err value and can be optionally referenced,
//!         //   it is discarded if it is not referenced
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### `Option<T>`
//!
//! ```no_run
//! # use std::fs::File;
//! # use std::io;
//! # use std::io::{BufWriter, Write};
//! # use std::process::exit;
//! use macrofied_toolbox::option;
//!
//! fn main() {
//!     let file_name = "foo.txt";
//!
//!     if let None = example(file_name) {
//!         eprintln!("failed to create {:?} file!", file_name);
//!         exit(-1);
//!     }
//! }
//!
//! fn example(file_name: &str) -> Option<()> {
//!     // attempts to create a file
//!     option! {
//!         // * if you use the try "?" operator, the result! macro will
//!         //   still output debug or error before returning the Result::Err
//!         @when  File::create(file_name).ok()?;
//!         // if the file is successfully created, write some content
//!         @some  (file) => {
//!             let mut out = BufWriter::new(file);
//!
//!             writeln!(out, "some content").ok()?;
//!             writeln!(out, "some more content").ok()?;
//!         }
//!         // if an exception occurs output debug message to stdout
//!         @debug "problem creating file: {:?}", file_name
//!
//!         // * debug messages are conditionally compiled
//!         //   and do not output anything in release builds
//!         // * "err" contains the Result::Err value and can be optionally referenced,
//!         //   it is discarded if it is not referenced
//!     }
//!
//!     Some(())
//! }
//! ```

#[cfg(any(feature = "result", feature = "option"))]
#[macro_use]
extern crate bitflags;
#[cfg(any(feature = "result", feature = "option"))]
#[macro_use]
extern crate cfg_if;
#[cfg(any(feature = "result", feature = "option"))]
#[macro_use]
extern crate quote;
#[cfg(any(feature = "result", feature = "option"))]
#[macro_use]
extern crate syn;

#[cfg(any(feature = "result", feature = "option"))]
use proc_macro::TokenStream;

#[cfg(any(feature = "result", feature = "option"))]
use quote::ToTokens;

#[cfg(any(feature = "option", feature = "result"))]
mod common;

#[cfg(feature = "option")]
mod option;

#[cfg(feature = "result")]
mod result;

#[cfg(test)]
mod tests;

///
#[cfg(feature = "option")]
#[proc_macro]
pub fn option(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as option::OptionMacro).into_token_stream().into()
}

///
#[cfg(feature = "result")]
#[proc_macro]
pub fn result(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as result::ResultMacro).into_token_stream().into()
}