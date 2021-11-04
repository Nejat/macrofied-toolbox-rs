use test_toolbox::capture;

use macrofied_toolbox::result;

#[test]
fn when_result_ok_should_evaluate_ok_expression() {
    let expected = 42;
    let actual = result! {
        @when foo_ok(21)
        @ok   (baz) => baz * 2
        @error unreachable!()
    };

    assert_eq!(expected, actual);
}

#[test]
fn when_result_err_should_evaluate_call_expression() {
    let expected = -42;
    let actual: isize;

    let (_stdout, _stderr) = capture! {{
        actual = result! {
            @when  foo_err(21)
            @ok    (baz) => baz * 2
            @error "ERR: {}", err;
                   error_happened()
        };
    }};

    assert_eq!(expected, actual);

    fn error_happened() -> isize { -42 }
}

#[test]
fn when_result_err_no_message_should_evaluate_call_expression() {
    let expected = -42;
    let actual: isize = result! {
        @when  foo_err(21)
        @ok    (baz) => baz * 2
        @error error_happened()
    };

    assert_eq!(expected, actual);

    fn error_happened() -> isize { -42 }
}

#[test]
fn when_result_err_should_evaluate_literal_expression() {
    let expected = -42;
    let actual: isize;

    let (_stdout, _stderr) = capture! {{
        actual = result! {
            @when  foo_err(21)
            @ok    (baz) => baz * 2
            @error "ERR: {}", err;
                   -42
        }
    }};

    assert_eq!(expected, actual);
}

#[test]
fn when_result_err_no_message_should_evaluate_literal_expression() {
    let expected = -42;
    let actual: isize = result! {
        @when  foo_err(21)
        @ok    (baz) => baz * 2
        @error -42
    };

    assert_eq!(expected, actual);
}

fn foo_ok<T>(value: T) -> Result<T, &'static str> {
    Ok(value)
}

fn foo_err<T>(_value: T) -> Result<T, &'static str> {
    Err("Foo Failed!")
}
