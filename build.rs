use std::io;

pub mod build_macros;

use build_macros::generate_result_macro;

fn main() -> io::Result<()> {
    generate_result_macro()
}
