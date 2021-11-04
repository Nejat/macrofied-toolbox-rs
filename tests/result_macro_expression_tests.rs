use cfg_if::cfg_if;
use test_toolbox::capture;
#[cfg(feature = "option-debug")]
use test_toolbox::expect;

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
    cfg_if! {
        if #[cfg(feature = "result-debug")] {
            expect! { expected_stdout = "", "ERR: Foo Failed!\n" }
        } else {
            let expected_stdout = "";
        }
    }

    let expected = -42;
    let actual: isize;

    let (actual_stdout, _stderr) = capture! {
        actual = result! {
            @when  foo_err(21)
            @ok    (baz) => baz * 2
            @debug "ERR: {}", err
            @error error_value()
        }
    };

    assert_eq!(expected, actual);
    assert_eq!(expected_stdout, actual_stdout);

    fn error_value() -> isize { -42 }
}

#[test]
fn when_result_err_no_message_should_evaluate_call_expression() {
    let expected = -42;
    let actual: isize = result! {
        @when  foo_err(21)
        @ok    (baz) => baz * 2
        @error error_value()
    };

    assert_eq!(expected, actual);

    fn error_value() -> isize { -42 }
}

#[test]
fn when_result_err_should_evaluate_literal_expression() {
    cfg_if! {
        if #[cfg(feature = "result-debug")] {
            expect! { expected_stdout = "", "ERR: Foo Failed!\n" }
        } else {
            let expected_stdout = "";
        }
    }

    let expected = -42;
    let actual: isize;

    let (actual_stdout, _stderr) = capture! {
        actual = result! {
            @when  foo_err(21)
            @ok    (baz) => baz * 2
            @debug "ERR: {}", err
            @error -42
        }
    };

    assert_eq!(expected, actual);
    assert_eq!(expected_stdout, actual_stdout);
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
