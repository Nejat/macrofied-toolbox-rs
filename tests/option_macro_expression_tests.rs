use test_toolbox::capture;

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
    let expected = -42;
    let actual: isize;

    let (_stdout, _stderr) = capture! {{
        actual = option! {
            @when foo_none(21)
            @some (baz) => baz * 2
            @none "It was None";
                  error_happened()
        };
    }};

    assert_eq!(expected, actual);

    fn error_happened() -> isize { -42 }
}

#[test]
fn when_option_none_no_message_should_evaluate_call_expression() {
    let expected = -42;
    let actual: isize = option! {
        @when foo_none(21)
        @some (baz) => baz * 2
        @none error_happened()
    };

    assert_eq!(expected, actual);

    fn error_happened() -> isize { -42 }
}

#[test]
fn when_option_none_should_evaluate_literal_expression() {
    let expected = -42;
    let actual: isize;

    let (_stdout, _stderr) = capture! {{
        actual = option! {
            @when foo_none(21)
            @some (baz) => baz * 2
            @none "It was None";
                  -42
        }
    }};

    assert_eq!(expected, actual);
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
