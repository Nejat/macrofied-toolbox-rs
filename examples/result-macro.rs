use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

#[cfg(all(debug_assertions, feature = "debug-result"))]
use cli_toolbox::debug;

use macrofied_toolbox::result;
use std::env::args;
use std::process::exit;

fn main() -> io::Result<()> {
    let file_name = if args().count() == 1 { "foo.txt" } else { "foo/foo.txt" };

    // attempts to create a file
    result! {
         WHEN   File::create(file_name);
         // if the file is successfully created, write some content
         OK     file; {
             let mut out = BufWriter::new(file);

             writeln!(out, "some content")?;
             writeln!(out, "some more content")?;

            println!("done - created {}", file_name);
         }
         // if an exception occurs output debug message to stderr
         DEBUG  "problem creating file: {:?}", file_name;
         ERR    exit(-1)

         // * debug messages are conditionally compiled
         //   and do not output anything in release builds
         // * exceptions are appended to the debug message
     }

    Ok(())
}