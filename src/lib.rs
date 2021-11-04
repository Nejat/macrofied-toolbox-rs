#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]
// ==============================================================
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::items_after_statements)]
// ==============================================================
#![doc(html_root_url = "https://docs.rs/macrofied-toolbox/0.3.0")]

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
//! ```rust,no_run
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
//! ```rust,no_run
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
mod option_macro;

#[cfg(feature = "result")]
mod result_macro;

#[cfg(test)]
mod tests;

/// a macro for making debugging more ergonomic when handling `Option<T>` results
///
/// ## Anotomy of the `option!` macro
///
/// The `option!` macro consists of a `@when` section and one to three optional evaluation 
/// sections `@some`, `@debug` and/or `@none`, at least one must be defined.
/// 
/// When the `option!` macro is used in place of an expression and the intention is to
/// use the `Some(T)` value, the `@when` section can be skipped and replaced with a
/// required simplified `@some` section, which behaves as the `@when` section, _* see
/// below for more details_
///
/// <br/>__\* _any_ `<expr>`_s which are code blocks, i.e._ `{ ... }` _, can not have an optional
/// section terminator_ `;`__<br/>
///
/// ### `@when`
///
/// The `@when` section is defined as `[@when] <expr>[?][;]`
///
/// * `@when` - the identifier itself is optional
/// * `<expr>` - an expression that must  evaluate to an `Option<T>` value
/// * `[?]` - the try operator will return `None` after completing `@debug` and/or `@none`
/// * `[;]` - optional section terminator
///
/// __`Example A:`__ `@when foo();`<br/>
/// __`Example B:`__ `@when foo()?;`<br/>
///
/// ### `@some`
///
/// The `@some` section is defined as `@some <[[(identifier) =>]<message|expr>[;]|[<expr>[?][;]]]`
///
/// * `@some` - required section identifier
/// * __In Success Mode__
///     * `[(identifier) =>]` - a custom defined identifier which is available in
///                               the `message` or `expr`
///     * `<message|expr>` - can access the custom defined identifier or the `some` keyword
///         * `message` - outputs to `stdout` with a `println!` statement, therefore has the same `args`
///         * `expr` - any expression to evaluate
///     * `[;]` - optional section terminator<br/><br/>
/// \* _only evaluates if the result of the `@when` expression is_ `Option::Some`<br/><br/>
/// __`Example A:`__ `@some "success: {}", some;`<br/>
/// __`Example B:`__ `@some (foo) => "success: {}", foo;`<br/>
/// __`Example C:`__ `@some (foo) => { success(foo); }`<br/>
/// * __In Expression Mode__
///     * `<expr>` - an expression that must  evaluate to an `Option<T>` value
///     * `[?]` - the try operator will return `None` after completing `@debug` and/or `@none`
///     * `[;]` - optional section terminator<br/><br/>
/// __`Example A:`__ `@some foo();`<br/>
/// __`Example B:`__ `@some foo()?;`<br/>
/// ### `@debug`
///
/// The `@debug` section is defined as `@debug <message>[;]`
///
/// \* _only evaluates if the result of the `@when` expression is_ `Option::None`
///
/// * `@debug` - required section identifier
/// * `message` - outputs to `stdout` with a `println!` statement, therefore has the same `args`
/// * `[;]` - optional section terminator
///
/// __`Example:`__ `@debug "dbg: foo failed!";`
///
/// ### `@none`
///
/// The `@none` section is defined as `@none [<message>[;]][<expr>][;]`, must
/// provide at least a `message` and/or `expr`
///
/// \* _only evaluates if the result of the `@when` expression is_ `Option::None`
///
/// * `@none` - required section identifier
/// * `[message][;]` - _optional_, outputs to `stderr` with a `println!` statement, therefore
///                    has the same `args`, requires the `;` terminator if an `<expr>[;]` is
///                    also defined
/// * `[<expr>]` - _optional_, any expression to evaluate
/// * `[;]` - optional section terminator
///
/// __`Example A:`__ `@none { on_fail_baz(); }`<br/>
/// __`Example B:`__ `@none "err: foo failed!"`<br/>
/// __`Example C:`__ `@none "err: foo failed!"; { on_fail_baz(); }`<br/>
///
/// ## Example
///
/// * Success Mode
/// ```rust,no_run
/// # use std::fs::File;
/// # use std::io;
/// # use std::io::{BufWriter, Write};
/// # use std::process::exit;
/// use macrofied_toolbox::option;
///
/// fn main() {
///     let file_name = "foo.txt";
///
///     if let None = example(file_name) {
///         eprintln!("failed to create {:?} file!", file_name);
///         exit(-1);
///     }
/// }
///
/// fn example(file_name: &str) -> Option<()> {
///     option! {
///         @when  File::create(file_name).ok()?;
///         @some  (file) => {
///                    let mut out = BufWriter::new(file);
///
///                    writeln!(out, "some content").ok()?;
///                    writeln!(out, "some more content").ok()?;
///                }
///         @debug "problem creating file: {:?}", file_name;
///         @none  "{:?} failed; attempting recovery ...", file_name;
///                recovery_from_fail(file_name);
///     }
///
///     Some(())
/// }
///
/// fn recovery_from_fail(_: &str) {
///     // some very import recovery logic
/// }
/// ```
/// * Expression Mode
/// ```rust
/// use macrofied_toolbox::option;
///
/// let result = option! {
///     @some  computed_value(21)
///     @debug "Invalid input"
///     @none  0
/// };
///
/// assert_eq!(42, result);
///
/// fn computed_value(input: usize) -> Option<usize> {
///     if input == 21  {
///         Some(input * 2)
///     } else {
///         None
///     }
/// }
/// ```
#[cfg(feature = "option")]
#[proc_macro]
pub fn option(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as option_macro::OptionMacro).into_token_stream().into()
}

/// a macro for making debugging more ergonomic when handling `Result<T,E>` results
///
/// ## Anotomy of the `result!` macro
///
/// The `result!` macro consists of a required `@when` section and one to three
/// optional evaluation sections `@ok`, `@debug` and/or `@error`, at least one must be
/// defined.
///
/// When the `result!` macro is used in place of an expression and the intention is to
/// use the `Ok(T)` value, the `@when` section can be skipped and replaced with a
/// required `@ok` section, which behaves as the `@when` section, _* see below
/// for more details_
///
/// <br/>__\* _any_ `<expr>`_s which are code blocks, i.e._ `{ ... }` _, can not have an optional
/// section terminator_ `;`__<br/>
///
/// ### `@when`
///
/// The `@when` section is defined as `[@when] <expr>[?][;]`
///
/// * `@when` - the identifier itself is optional
/// * `<expr>` - an expression that must  evaluate to a `Result<T,E>` value
/// * `[?]` - the try operator will return `Result::Err` after completing `@debug` and/or `@error`
/// * `[;]` - optional section terminator
///
/// __`Example A:`__ `@when foo()?;`<br/>
/// __`Example B:`__ `@when foo()?;`<br/>
///
/// ### `@ok`
///
/// The `@ok` section is defined as `@some [[(identifier) =>]<message|expr>[;]|[<expr>[?][;]]`
///
/// * `@ok` - required section identifier
/// * __In Success Mode__
///     * `[(identifier) =>]` - a custom defined identifier which is available in
///                               the `message` or `expr`
///     * `<message|expr>` - can access the custom defined identifier or the `ok` keyword
///         * `message` - outputs to `stdout` with a `println!` statement, therefore has the same `args`
///         * `expr` - any expression to evaluate
///     * `[;]` - optional section terminator<br/><br/>
/// \* _only evaluates if the result of the `@when` expression is_ `Result::Ok`<br/><br/>
/// __`Example:`__ `@ok "success: {}", ok;`<br/>
/// __`Example:`__ `@ok (foo) => "success: {}", foo;`<br/>
/// __`Example:`__ `@ok (foo) => { success(foo) }`<br/>
/// * __In Expression Mode__
///     * `<expr>` - an expression that must  evaluate to an `Option<T>` value
///     * `[?]` - the try operator will return `None` after completing `@debug` and/or `@none`
///     * `[;]` - optional section terminator<br/><br/>
/// __`Example A:`__ `@ok foo();`<br/>
/// __`Example B:`__ `@ok foo()?;`<br/>
///
/// ### `@debug`
///
/// The `@debug` section is defined as `@debug <message>[;]`
///
/// \* _only evaluates if the result of the `@when` expression is_ `Result::Err`
///
/// * `@debug` - required section identifier
/// * `message` - outputs to `stdout` with a `println!` statement, therefore has the same `args`,
///               can use optional `err` keyword to report `Result::Err(err)`
/// * `[;]` - optional section terminator
///
/// __`Example:`__ `@debug "dbg: foo failed! - {}", err;`
///
/// ### `@none`
///
/// The `@error` section is defined as `@error [<message>[;]][<expr>][;]`, must
/// provide at least a `message` and/or `expr`
///
/// \* _only evaluates if the result of the `@when` expression is_ `Result::Err`
///
/// * `@error` - required section identifier
/// * `[message][;]` - _optional_, outputs to `stderr` with a `println!` statement, therefore
///                    has the same `args`, requires the `;` terminator if an `<expr>[;]` is
///                    also defined, can use optional `err` keyword to report `Result::Err(err)`
/// * `[<expr>]` - _optional_, any expression to evaluate, can use optional `err` keyword to
///                report `Result::Err(err)`
/// * `[;]` - optional section terminator
///
/// __`Example A:`__ `@err "err: foo failed! - {}", err`<br/>
/// __`Example B:`__ `{ on_fail_baz(err); }`<br/>
/// __`Example C:`__ `@err "err: foo failed! - {}", err; { on_fail_baz(err); }`<br/>
///
/// ## Example
///
/// * Success Mode
///
/// ```rust,no_run
/// # use std::fs::File;
/// # use std::io;
/// # use std::io::{BufWriter, Write};
/// # use std::process::exit;
/// use macrofied_toolbox::result;
///
/// fn main() -> io::Result<()> {
///     let file_name = "foo.txt";
///
///     result! {
///         @when  File::create(file_name)?;
///         @ok    (file) => {
///                    let mut out = BufWriter::new(file);
///
///                    writeln!(out, "some content")?;
///                    writeln!(out, "some more content")?;
///                }
///         @debug "problem creating file: {:?} - {}", file_name, err;
///         @error "{:?} failed - {}; attempting recovery ...", file_name, err;
///                recovery_from_fail(file_name);
///     }
///
///     Ok(())
/// }
///
/// fn recovery_from_fail(_: &str) {
///     // some very import recovery logic
/// }
/// ```
/// * Expression Mode
/// ```rust
/// use macrofied_toolbox::result;
///
/// let result = result! {
///     @ok    computed_value(21)
///     @debug "ERR: {:?}", err
///     @error 0
/// };
///
/// assert_eq!(42, result);
///
/// fn computed_value(input: usize) -> Result<usize, &'static str> {
///     if input == 21  {
///         Ok(input * 2)
///     } else {
///         Err("I can't let you do that")
///     }
/// }
/// ```
#[cfg(feature = "result")]
#[proc_macro]
pub fn result(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as result_macro::ResultMacro).into_token_stream().into()
}