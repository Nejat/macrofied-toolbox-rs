use test_toolbox::capture;

use macrofied_toolbox::option;

#[test]
fn when_option_some_should_evaluate_ok_expression() {
    let expected = 42;
    let actual = option! {
        @some foo_some()
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
            @some foo_none()
            @none "It was None"; error_value()
        };
    }};

    assert_eq!(expected, actual);

    fn error_value() -> isize { -42 }
}

#[test]
fn when_option_none_no_message_should_evaluate_call_expression() {
    let expected = -42;
    let actual: isize = option! {
        @some foo_none()
        @none error_value()
    };

    assert_eq!(expected, actual);

    fn error_value() -> isize { -42 }
}

#[test]
fn when_option_none_should_evaluate_literal_expression() {
    let expected = -42;
    let actual: isize;

    let (_stdout, _stderr) = capture! {{
        actual = option! {
            @some foo_none()
            @none "It was None"; -42
        }
    }};

    assert_eq!(expected, actual);
}

#[test]
fn when_option_none_no_message_should_evaluate_literal_expression() {
    let expected = -42;
    let actual: isize = option! {
        @some foo_none()
        @none -42
    };

    assert_eq!(expected, actual);
}

fn foo_some() -> Option<isize> {
    Some(42)
}

fn foo_none() -> Option<isize> {
    None
}
