use std::env;

use macrofied_toolbox::result;

fn main() -> Result<(), &'static str> {
    let break_test = env::args().collect::<Vec<_>>().last().unwrap_or(&String::new()) == "break";

    if break_test {
        result! {
            @when  foo(false)?;
            @ok    "will not be seen: {:?}", ok;
            @debug "dbg msg: {}", -24;
            @error "err msg: {:?} - {}", err, -42;
                   { println!("error expression") }
        }
    } else {
        result! {
            @when  foo(true)?;
            @ok    "all ok: {:?}", ok;
            @debug "dbg msg will not be seen: {}", -24;
            @error "err msg will not be seen: {:?} - {}", err, -42;
                   { println!("error expression not run") }
        }
    }

    return Ok(());

    fn foo(succeed: bool) -> Result<usize, &'static str> {
        if succeed {
            Ok(42)
        } else {
            Err("foo failed")
        }
    }
}