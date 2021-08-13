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
//! use std::fs::File;
//! use std::io;
//! use std::io::{BufWriter, Write};
//!
//! use macrofied_toolbox::result;
//!
//! #[cfg(debug_assertions)]
//! use cli_toolbox::debug;
//!
//! fn main() -> io::Result<()> {
//!     let file_name = "foo.txt";
//!
//!     // attempts to create a file
//!     result! {
//!         WHEN  File::create(file_name);
//!         // if the file is successfully created, write some content
//!         OK    file; {
//!             let mut out = BufWriter::new(file);
//!
//!             writeln!(out, "some content")?;
//!             writeln!(out, "some more content")?;
//!         }
//!         // if an exception occurs output debug message to stderr
//!         DEBUG "problem creating file: {:?}", file_name
//!
//!         // * debug messages are conditionally compiled
//!         //   and do not output anything in release builds
//!         // * exceptions are appended to the debug message
//!     }
//!
//!     Ok(())
//! }
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