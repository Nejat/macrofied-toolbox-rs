# macrofied-toolbox

This library provides an ergonomic experience of adding debugging messages to rust's 
`Result<T,E>` and `Option<T>` patterns

Just like the [`cli-toolbox`](https://crates.io/crates/cli-toolbox) crate, that the debug logic
is based on, this is not a logging alternative; it's intended to produce debugging output to be
used during application development.

Although the macros were designed to make debugging more ergonomic, they include variations that 
do not include debugging to provide coding consistency, so you have the option use the same syntax
consistently throughout your crate.

### `Result<T,E>`
```rust
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

use macrofied_toolbox::result;

#[cfg(debug_assertions)]
use cli_toolbox::debug;

fn main() -> io::Result<()> {
    let file_name = "foo.txt";
    
    // attempts to create a file 
    result! {
        WHEN  File::create(file_name);
        // if the file is successfully created, write some content
        OK    file; {
            let mut out = BufWriter::new(file);
            
            writeln!(out, "some content")?;
            writeln!(out, "some more content")?;
        }
        // if an exception occurs output debug message to stderr
        DEBUG "problem creating file: {:?}", file_name
        
        // * debug messages are conditionally compiled 
        //   and do not output anything in release builds
        // * exceptions are appended to the debug message
    };

    Ok(())
}
```

### `Option<T>`

```
.
.
.
```

\* _the macros are automatically generated with custom build scripts, including their_ `docs` and `tests`

## Resources
* [Docs](https://docs.rs/macrofied-toolbox/0.1.0/macrofied_toolbox/) for more detailed information
* [Examples](https://github.com/Nejat/macrofied-toolbox-rs/tree/v0.1.0/examples) to see it in action

## Usage

Each macro is gated by a feature.

No feature is mutually exclusive and can be combined as needed.

```toml
[dependencies]
macrofied-toolbox = { version = "0.1", features = ["option", "result"] }
```

### Additional Feature

`macrofied-toolbox` can optionally use the [`cli-toolbox`](https://github.com/Nejat/cli-toolbox-rs) crate to output 
debug to the console by enabling an of these features

* `debug-all` - enables console debugging for all the features enabled
* `debug-option` - enables console debugging for the `option!` macro
* `debug-result` - enables console debugging for the `result!` macro

## Roadmap

* [ ] `option!` - handles an `Option<T>` of an expression
* [ ] enhance `result!` 
  * [ ] pass `Ok<t>` value through
  * [ ] add `?` syntax
* [ ] support more than mutable `Ok<T>` values
* [ ] debugging for `Ok<T>` values
* [ ] logging for both `Ok<T>` and `Err<E>`
* [ ] other patterns?

## Implemented
* [x] `result!` - handles a `Result<T,E>` of an expression

