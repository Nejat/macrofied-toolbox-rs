use test_toolbox::capture;

use macrofied_toolbox::option;

#[test]
fn when_message_with_an_array_expression_should_output() {
    let expected_stdout = "some: [42, 42]\n";

    let (actual_stdout, _actual_stderr) = capture! {
        option! {
            @when foo_ok()
            @some (baz) => "some: {:?}", [baz * 2, 42]
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
}

#[test]
fn when_message_with_a_binary_expression_should_output() {
    let expected_stdout = "some: 42\n";

    let (actual_stdout, _actual_stderr) = capture! {
        option! {
            @when foo_ok()
            @some (baz) => "some: {:?}", baz * 2
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
}

#[test]
fn when_message_with_a_block_expression_should_output() {
    let expected_stdout = "some: 42\n";

    let (actual_stdout, _actual_stderr) = capture! {
        option! {
            @when foo_ok()
            @some (baz) => "some: {:?}", { baz * 2 }
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
}

#[test]
fn when_message_with_a_call_expression_should_output() {
    let expected_stdout = "some: 42\n";

    let (actual_stdout, _actual_stderr) = capture! {
        option! {
            @when foo_ok()
            @some (baz) => "some: {:?}", method_to_call(baz)
        }
    };

    assert_eq!(expected_stdout, actual_stdout);

    fn method_to_call(value: usize) -> usize { value * 2 }
}

#[test]
fn when_message_with_a_cast_expression_should_output() {
    let expected_stdout = "some: 42\n";

    let (actual_stdout, _actual_stderr) = capture! {
        option! {
            @when foo_ok()
            @some (baz) => "some: {:?}", (baz as isize * 2_isize) as usize
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
}

#[test]
fn when_message_with_a_field_expression_should_output() {
    struct Foo(usize);

    let expected_stdout = "some: 42\n";

    let (actual_stdout, _actual_stderr) = capture! {
        option! {
            @when foo_ok()
            @some (baz) => "some: {:?}", Foo(baz * 2).0
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
}

#[test]
fn when_message_with_an_if_expression_should_output() {
    let expected_stdout = "some: 42\n";

    let (actual_stdout, _actual_stderr) = capture! {
        option! {
            @when foo_ok()
            @some (baz) => "some: {:?}", if baz == 21 { 42 } else { 0 }
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
}

#[test]
fn when_message_with_an_index_expression_should_output() {
    let expected_stdout = "some: 42\n";
    let foo = [42; 25];

    let (actual_stdout, _actual_stderr) = capture! {
        option! {
            @when foo_ok()
            @some (baz) => "some: {:?}", foo[baz]
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
}

#[test]
fn when_message_with_an_literal_expression_should_output() {
    let expected_stdout = "some: 42\n";

    let (actual_stdout, _actual_stderr) = capture! {
        option! {
            @when foo_ok()
            @some "some: {:?}", 42
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
}

#[test]
fn when_message_with_a_macro_expression_should_output() {
    let expected_stdout = "some: <42>\n";

    let (actual_stdout, _actual_stderr) = capture! {
        option! {
            @when foo_ok()
            @some (baz) => "some: {}", format!("<{}>", baz * 2);
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
}

#[test]
fn when_message_with_a_match_expression_should_output() {
    let expected_stdout = "some: 42\n";

    let (actual_stdout, _actual_stderr) = capture! {
        option! {
            @when foo_ok()
            @some (baz) => "some: {}", match baz { 21 => "42", _ => "" }
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
}

#[test]
fn when_message_with_a_method_call_expression_should_output() {
    struct Bar;

    impl Bar { fn method_to_call(&self, value: usize) -> usize { value * 2 } }

    let expected_stdout = "some: 42\n";
    let bar = Bar;

    let (actual_stdout, _actual_stderr) = capture! {
        option! {
            @when foo_ok()
            @some (baz) => "some: {}", bar.method_to_call(baz)
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
}

#[test]
fn when_message_with_a_paren_expression_should_output() {
    let expected_stdout = "some: 42\n";

    let (actual_stdout, _actual_stderr) = capture! {
        option! {
            @when foo_ok()
            @some (baz) => "some: {}", (baz * 2)
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
}

#[test]
fn when_message_with_a_path_expression_should_output() {
    mod bar { pub mod junk { pub const BAZ: usize = 42; } }

    let expected_stdout = "some: 42\n";

    let (actual_stdout, _actual_stderr) = capture! {
        option! {
            @when foo_ok()
            @some "some: {}", bar::junk::BAZ
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
}

#[test]
fn when_message_with_a_range_expression_should_output() {
    let expected_stdout = "some: ..42\n";

    let (actual_stdout, _actual_stderr) = capture! {
        option! {
            @when foo_ok()
            @some "some: {:?}", ..42
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
}

#[test]
fn when_message_with_a_repeat_expression_should_output() {
    let expected_stdout = "some: [42, 42, 42]\n";

    let (actual_stdout, _actual_stderr) = capture! {
        option! {
            @when foo_ok()
            @some "some: {:?}", [42; 3]
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
}

#[test]
fn when_message_with_a_try_expression_should_output() {
    inner().unwrap();

    fn inner() -> Option<()> {
        let expected_stdout = "some: 42\n";

        let (actual_stdout, _actual_stderr) = capture! {
            option! {
                @when foo_ok()
                @some (baz) => "some: {}", method_to_try(baz)?
            }
        };

        assert_eq!(expected_stdout, actual_stdout);

        return Some(());

        fn method_to_try(value: usize) -> Option<usize> {
            Some(value * 2)
        }
    }
}

#[test]
fn when_message_with_a_tuple_expression_should_output() {
    let expected_stdout = "some: (42, 42, 42)\n";

    let (actual_stdout, _actual_stderr) = capture! {
        option! {
            @when foo_ok()
            @some (baz) => "some: {:?}", (baz * 2, baz * 2, baz * 2)
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
}

#[test]
fn when_message_with_a_unary_expression_should_output() {
    let expected_stdout = "some: 42\n";

    let (actual_stdout, _actual_stderr) = capture! {
        option! {
            @when foo_ok()
            @some (baz) => "some: {:?}", -(baz as isize * -2)
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
}

#[test]
fn when_message_with_an_unsafe_expression_should_output() {
    let expected_stdout = "some: 42\n";

    let (actual_stdout, _actual_stderr) = capture! {
        option! {
            @when foo_ok()
            @some (baz) => "some: {:?}", unsafe { unsafe_method_to_call(baz) }
        }
    };

    assert_eq!(expected_stdout, actual_stdout);

    // need to test message parser
    // * although I'm not sure if this gets wiped before getting to the macro
    #[allow(unused_unsafe)]
    unsafe fn unsafe_method_to_call(value: usize) -> usize {
        unsafe { value * 2 }
    }
}

fn foo_ok() -> Option<usize> {
    Some(21)
}

