use test_toolbox::capture;

use macrofied_toolbox::result;

#[test]
fn when_result_ok_should_evaluate_ok_expression() {
    let expected = 42;
    let actual = result! {
        @ok    foo_ok()
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
            @ok    foo_err()
            @error "ERR: {:?}", err; error_value()
        };
    }};

    assert_eq!(expected, actual);

    fn error_value() -> isize { -42 }
}

#[test]
fn when_result_err_no_message_should_evaluate_call_expression() {
    let expected = -42;
    let actual: isize = result! {
        @ok    foo_err()
        @error error_value()
    };

    assert_eq!(expected, actual);

    fn error_value() -> isize { -42 }
}

#[test]
fn when_result_err_should_evaluate_literal_expression() {
    let expected = -42;
    let actual: isize;

    let (_stdout, _stderr) = capture! {{
        actual = result! {
            @ok    foo_err()
            @error "ERR: {:?}", err; -42
        }
    }};

    assert_eq!(expected, actual);
}

#[test]
fn when_result_err_no_message_should_evaluate_literal_expression() {
    let expected = -42;
    let actual: isize = result! {
        @ok    foo_err()
        @error -42
    };

    assert_eq!(expected, actual);
}

fn foo_ok() -> Result<isize, ()> {
    Ok(42)
}

fn foo_err() -> Result<isize, ()> {
    Err(())
}
