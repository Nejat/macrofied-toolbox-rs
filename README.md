# macrofied-toolbox

This library provides an ergonomic experience of adding debugging messages to rust's 
`Result<T,E>` and `Option<T>` patterns

Just like the [`cli-toolbox`](https://crates.io/crates/cli-toolbox) crate, that the debug logic
resembles, this is not a logging alternative; it's intended to produce debugging output to be
used during application development.

Although the macros were designed to make debugging more ergonomic, they include variations that 
do not include debugging to provide coding consistency, so you have the option to use the same syntax
consistently throughout your project.

### `Result<T,E>`
```rust
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use macrofied_toolbox::result;

fn main() -> io::Result<()> {
    let file_name = "foo.txt";

    // attempts to create a file
    result! {
        // * if you use the try "?" punctuation, the result! macro will
        //   still output debug or error before returning the Result::Err
        @when  File::create(file_name)?;
        // if the file is successfully created, write some content
        @ok    (file) => {
            let mut out = BufWriter::new(file);

            writeln!(out, "some content")?;
            writeln!(out, "some more content")?;
        }
        // if an exception occurs output debug message to stdout
        @debug "problem creating file: {:?} - {}", file_name, err

        // * debug messages are conditionally compiled
        //   and do not output anything in release builds
        // * "err" contains the Result::Err value and can be optionally referenced,
        //   it is discarded if it is not referenced
    }

    Ok(())
}
```

### `Option<T>`

```rust
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::process::exit;

use macrofied_toolbox::option;

fn main() {
    let file_name = "foo.txt";

    if let None = example(file_name) {
        eprintln!("failed to create {:?} file!", file_name);
        exit(-1);
    }
}

fn example(file_name: &str) -> Option<()> {
    // attempts to create a file
    option! {
        // * if you use the try "?" punctuation, the result! macro will
        //   still output debug or error before returning the Result::Err
        @when  File::create(file_name).ok()?;
        // if the file is successfully created, write some content
        @some  (file) => {
            let mut out = BufWriter::new(file);

            writeln!(out, "some content").ok()?;
            writeln!(out, "some more content").ok()?;
        }
        // if an exception occurs output debug message to stdout
        @debug "problem creating file: {:?}", file_name

        // * debug messages are conditionally compiled
        //   and do not output anything in release builds
        // * "err" contains the Result::Err value and can be optionally referenced,
        //   it is discarded if it is not referenced
    }

    Some(())
}
```

## Resources
* [Docs](https://docs.rs/macrofied-toolbox/0.1.0/macrofied_toolbox/) for more detailed information
* [Examples](https://github.com/Nejat/macrofied-toolbox-rs/tree/v0.1.0/examples) to see it in action

## Usage

Each macro is gated by a feature; `all`, `option` or `result` respectively.

```toml
[dependencies]
macrofied-toolbox = { version = "0.1", features = ["option", "result"] }
```

### Features

Although `macrofied-toolbox` was designed to make adding debugging output more ergonomic,
the generation of debug output is gated by an optional `X-debug` feature. 

\* _Debug output is only effective in unoptimized builds_ \*

* `all-debug` - enables console debugging and both features
* `option-debug` - enables console debugging and the `option!` macro
* `result-debug` - enables console debugging and the `result!` macro

## Roadmap

* [ ] support mutable `Ok<T>` and `Some<T>` values
* [ ] ~~logging for both Ok\<T\> and Err\<E\>~~
* [ ] ~~other patterns?~~

## Implemented
* [x] `result!` - handles a `Result<T,E>` of an expression
* [x] `option!` - handles an `Option<T>` of an expression
