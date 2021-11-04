use cfg_if::cfg_if;
use test_toolbox::capture;
#[cfg(feature = "option-debug")]
use test_toolbox::expect;

use macrofied_toolbox::option;

#[test]
fn when_option_some_should_evaluate_ok_expression() {
    let expected = 42;
    let actual = option! {
        @when foo_some(21)
        @some (baz) => baz * 2
        @none unreachable!()
    };

    assert_eq!(expected, actual);
}

#[test]
fn when_option_none_should_evaluate_call_expression() {
    cfg_if! {
        if #[cfg(feature = "option-debug")] {
            expect! { expected_stdout = "", "It was None\n" }
        } else {
            let expected_stdout = "";
        }
    }

    let expected = -42;
    let actual: isize;

    let (actual_stdout, _stderr) = capture! {{
        actual = option! {
            @when  foo_none(21)
            @some  (baz) => baz * 2
            @debug "It was None"
            @none  error_value()
        };
    }};

    assert_eq!(expected, actual);
    assert_eq!(expected_stdout, actual_stdout);

    fn error_value() -> isize { -42 }
}

#[test]
fn when_option_none_no_message_should_evaluate_call_expression() {
    let expected = -42;
    let actual: isize = option! {
        @when foo_none(21)
        @some (baz) => baz * 2
        @none error_value()
    };

    assert_eq!(expected, actual);

    fn error_value() -> isize { -42 }
}

#[test]
fn when_option_none_should_evaluate_literal_expression() {
    cfg_if! {
        if #[cfg(feature = "option-debug")] {
            expect! { expected_stdout = "", "It was None\n" }
        } else {
            let expected_stdout = "";
        }
    }

    let expected = -42;
    let actual: isize;

    let (actual_stdout, _stderr) = capture! {{
        actual = option! {
            @when  foo_none(21)
            @some  (baz) => baz * 2
            @debug "It was None"
            @none  -42
        }
    }};

    assert_eq!(expected, actual);
    assert_eq!(expected_stdout, actual_stdout);
}

#[test]
fn when_option_none_no_message_should_evaluate_literal_expression() {
    let expected = -42;
    let actual: isize = option! {
        @when foo_none(21)
        @some (baz) => baz * 2
        @none -42
    };

    assert_eq!(expected, actual);
}

fn foo_some<T>(value: T) -> Option<T> {
    Some(value)
}

fn foo_none<T>(_value: T) -> Option<T> {
    None
}
