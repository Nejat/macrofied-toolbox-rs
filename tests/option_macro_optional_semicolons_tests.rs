use test_toolbox::{capture, expect};

use macrofied_toolbox::option;

#[test]
fn when_none_option_with_some_should_eval_none_only() {
    let expected = false;
    let mut actual = true;

    option! {
        @when foo_none();
        @some "this will not output: {:?}", some;
        @none { actual = false; }
    }

    assert_eq!(expected, actual);
}

#[test]
fn when_none_option_with_some_should_output_and_eval_none_only() {
    let expected_err = "none: example message\n";
    let expected = false;
    let mut actual = true;

    let (_actual_out, actual_err) = capture! {
        option! {
            @when foo_none();
            @some "this will not output: {:?}", some;
            @none "none: example message";
                  actual = false
        }
    };

    assert_eq!(expected, actual);
    assert_eq!(expected_err, actual_err);
}

#[test]
fn when_none_option_with_some_should_output_dbg_and_none() {
    expect! { expected_err = "none: example message\n", "dbg: example message\nnone: example message\n" }

    let (_actual_out, actual_err) = capture! {
        option! {
            @when  foo_none();
            @some  "this will not output: {:?}", some;
            @debug "dbg: example message";
            @none  "none: example message"
        }
    };

    assert_eq!(expected_err, actual_err);
}

#[test]
fn when_none_option_with_some_should_output_dbg_and_none_and_eval_err() {
    expect! { expected_err = "none: example message\n", "dbg: example message\nnone: example message\n" }

    let expected = false;
    let mut actual = true;

    let (_actual_out, actual_err) = capture! {
        option! {
            @when  foo_none();
            @some  "this will not output: {:?}", some;
            @debug "dbg: example message";
            @none  "none: example message";
                   actual = false;
        }
    };

    assert_eq!(expected_err, actual_err);
    assert_eq!(expected, actual);
}

#[test]
fn when_none_option_with_some_should_output_dbg_only() {
    expect! { expected_err = "", "dbg: example message\n" }

    let (_actual_out, actual_err) = capture! {
        option! {
            @when  foo_none();
            @some  "this will not output: {:?}", some;
            @debug "dbg: example message";
        }
    };

    assert_eq!(expected_err, actual_err);
}

#[test]
fn when_none_option_with_some_should_output_none_only() {
    let expected_err = "none: example message\n";

    let (_actual_out, actual_err) = capture! {
        option! {
            @when foo_none();
            @some "this will not output: {:?}", some;
            @none "none: example message";
        }
    };

    assert_eq!(expected_err, actual_err);
}

#[test]
fn when_none_option_without_some_should_eval_none_only() {
    let expected = false;
    let mut actual = true;

    option! {
        @when foo_none();
        @none { actual = false; }
    }

    assert_eq!(expected, actual);
}

#[test]
fn when_none_option_without_some_should_output_and_eval_none_only() {
    let expected_err = "none: example message\n";
    let expected = false;
    let mut actual = true;

    let (_actual_out, actual_err) = capture! {
        option! {
            @when  foo_none();
            @none "none: example message";
                   actual = false;
        }
    };

    assert_eq!(expected, actual);
    assert_eq!(expected_err, actual_err);
}

#[test]
fn when_none_option_without_some_should_output_dbg_and_none() {
    expect! { expected_err = "none: example message\n", "dbg: example message\nnone: example message\n" }

    let (_actual_out, actual_err) = capture! {
        option! {
            @when  foo_none();
            @debug "dbg: example message";
            @none  "none: example message";
        }
    };

    assert_eq!(expected_err, actual_err);
}

#[test]
fn when_none_option_without_some_should_output_dbg_and_none_and_eval_err() {
    expect! { expected_err = "none: example message\n", "dbg: example message\nnone: example message\n" }

    let expected = false;
    let mut actual = true;

    let (_actual_out, actual_err) = capture! {
        option! {
            @when  foo_none();
            @debug "dbg: example message";
            @none  "none: example message";
                   actual = false;
        }
    };

    assert_eq!(expected_err, actual_err);
    assert_eq!(expected, actual);
}

#[test]
fn when_none_option_without_some_should_output_dbg_only() {
    expect! { expected_err = "", "dbg: example message\n" }

    let (_actual_out, actual_err) = capture! {
        option! {
            @when  foo_none();
            @debug "dbg: example message";
        }
    };

    assert_eq!(expected_err, actual_err);
}

#[test]
fn when_none_option_without_some_should_output_none_only() {
    let expected_err = "none: example message\n";

    let (_actual_out, actual_err) = capture! {
        option! {
            @when foo_none();
            @none "none: example message";
        }
    };

    assert_eq!(expected_err, actual_err);
}

#[test]
fn when_some_only_custom_result_should_output_some() {
    let expected_out = "some: 42\n";

    let (actual_out, _actual_err) = capture! {
        option! {
            @when foo_some();
            @some (foo) => "some: {:?}", foo;
        }
    };

    assert_eq!(expected_out, actual_out);
}

#[test]
fn when_some_only_custom_result_should_eval_some() {
    let expected = 42;
    let mut actual = 0;

    option! {
        @when foo_some();
        @some (foo) => { actual = foo; }
    }

    assert_eq!(expected, actual);
}

#[test]
fn when_some_only_result_should_eval_some() {
    let expected = 42;
    let mut actual = 0;

    option! {
        @when foo_some();
        @some actual = some;
    }

    assert_eq!(expected, actual);
}

#[test]
fn when_some_only_result_should_output_some() {
    let expected_out = "some: 42\n";

    let (actual_out, _actual_err) = capture! {
        option! {
            @when foo_some();
            @some (baz) => "some: {}", baz;
        }
    };

    assert_eq!(expected_out, actual_out);
}

#[test]
fn when_some_result_should_output_some() {
    let expected_out = "some: 42\n";

    let (actual_out, _actual_err) = capture! {
        option! {
            @when  foo_some();
            @some  (baz) => "some: {}", baz;
            @debug "dbg: this will not output";
            @none  "none: this will not output";
                   println!("none expression evaluated");
        }
    };

    assert_eq!(expected_out, actual_out);
}

fn foo_some() -> Option<usize> {
    Some(42)
}

fn foo_none() -> Option<usize> {
    None
}