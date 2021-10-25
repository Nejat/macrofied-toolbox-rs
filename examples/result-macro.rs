use std::env;

use macrofied_toolbox::result;

fn main() -> Result<(), &'static str> {
    let break_test = env::args().collect::<Vec<_>>().last().unwrap_or(&String::new()) == "break";

    if break_test {
        result! {
            @when  foo(false)?;
            @ok    (baz) => "will not be seen: {:?}", baz;
            @debug "dbg msg: {}", -24;
            @error "err msg: {}", -42;
                   { println!("error expression, {}", err) }
        }
    } else {
        result! {
            @when  foo(true)?;
            @ok    (baz) => "all ok: {:?} {}", baz + 1, 42;
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