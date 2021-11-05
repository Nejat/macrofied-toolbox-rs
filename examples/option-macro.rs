use std::env;

use macrofied_toolbox::option;

fn main() {
    let some = env::args().collect::<Vec<_>>().last().unwrap_or(&String::new()) == "none";

    if some {
        option! {
            @when  foo(false);
            @some  (foo_value) => "will not be seen {}", foo_value;
            @debug "dbg msg: {}", -24;
            @none  "none msg: {}", -42;
                   println!("none expression")
        }
    } else {
        option! {
            @when  foo(true);
            @some  "has some: {}", some;
            @debug "dbg msg will not be seen: {}", -24;
            @none  "none msg will not be seen: {}", -42;
                   println!("none expression not run")
        }
    }

    fn foo(some: bool) -> Option<usize> {
        if some {
            Some(42)
        } else {
            None
        }
    }
}