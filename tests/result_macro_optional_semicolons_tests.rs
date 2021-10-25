use test_toolbox::{capture, expect};

use macrofied_toolbox::result;

#[test]
fn when_err_result_with_ok_should_eval_err_only() {
    let expected = "foo failed!";
    let mut actual = "";

    result! {
        @when  foo_err();
        @ok    "this will not output: {:?}", ok;
        @error { actual = err; }
    }

    assert_eq!(expected, actual);
}

#[test]
fn when_err_result_with_ok_should_output_and_eval_err_only() {
    let expected_err = "err: \"foo failed!\"\n";
    let expected = "foo failed!";
    let mut actual = "";

    let (_actual_out, actual_err) = capture! {
        result! {
            @when  foo_err();
            @ok    "this will not output: {:?}", ok;
            @error "err: {:?}", err;
                   actual = err;
        }
    };

    assert_eq!(expected, actual);
    assert_eq!(expected_err, actual_err);
}

#[test]
fn when_err_result_with_ok_should_output_dbg_and_err() {
    expect! { expected_err = "err: foo failed!\n", "dbg: \"foo failed!\"\nerr: foo failed!\n" }

    let (_actual_out, actual_err) = capture! {
        result! {
            @when  foo_err();
            @ok    "this will not output: {:?}", ok;
            @debug "dbg: {:?}", err;
            @error "err: {}", err;
        }
    };

    assert_eq!(expected_err, actual_err);
}

#[test]
fn when_err_result_with_ok_should_output_dbg_and_err_and_eval_err () {
    expect! { expected_err = "err: foo failed!\n", "dbg: \"foo failed!\"\nerr: foo failed!\n" }

    let expected = "foo failed!";
    let mut actual = "";

    let (_actual_out, actual_err) = capture! {
        result! {
            @when  foo_err();
            @ok    "this will not output: {:?}", ok;
            @debug "dbg: {:?}", err;
            @error "err: {}", err;
                   actual = err;
        }
    };

    assert_eq!(expected_err, actual_err);
    assert_eq!(expected, actual);
}

#[test]
fn when_err_result_with_ok_should_output_dbg_only() {
    expect! { expected_err = "", "dbg: \"foo failed!\"\n" }

    let (_actual_out, actual_err) = capture! {
        result! {
            @when  foo_err();
            @ok    "this will not output: {:?}", ok;
            @debug "dbg: {:?}", err;
        }
    };

    assert_eq!(expected_err, actual_err);
}

#[test]
fn when_err_result_with_ok_should_output_err_only() {
    let expected_err = "err: \"foo failed!\"\n";

    let (_actual_out, actual_err) = capture! {
        result! {
            @when  foo_err();
            @ok    "this will not output: {:?}", ok;
            @error "err: {:?}", err;
        }
    };

    assert_eq!(expected_err, actual_err);
}

#[test]
fn when_err_result_without_ok_should_eval_err_only() {
    let expected = "foo failed!";
    let mut actual = "";

    result! {
        @when  foo_err();
        @error { actual = err; }
    }

    assert_eq!(expected, actual);
}

#[test]
fn when_err_result_without_ok_should_output_and_eval_err_only() {
    let expected_err = "err: \"foo failed!\"\n";
    let expected = "foo failed!";
    let mut actual = "";

    let (_actual_out, actual_err) = capture! {
        result! {
            @when  foo_err();
            @error "err: {:?}", err;
                   actual = err;
        }
    };

    assert_eq!(expected, actual);
    assert_eq!(expected_err, actual_err);
}

#[test]
fn when_err_result_without_ok_should_output_dbg_and_err() {
    expect! { expected_err = "err: foo failed!\n", "dbg: \"foo failed!\"\nerr: foo failed!\n" }

    let (_actual_out, actual_err) = capture! {
        result! {
            @when  foo_err();
            @debug "dbg: {:?}", err;
            @error "err: {}", err;
        }
    };

    assert_eq!(expected_err, actual_err);
}

#[test]
fn when_err_result_without_ok_should_output_dbg_and_err_and_eval_err () {
    expect! { expected_err = "err: foo failed!\n", "dbg: \"foo failed!\"\nerr: foo failed!\n" }

    let expected = "foo failed!";
    let mut actual = "";

    let (_actual_out, actual_err) = capture! {
        result! {
            @when  foo_err();
            @debug "dbg: {:?}", err;
            @error "err: {}", err;
                   actual = err;
        }
    };

    assert_eq!(expected_err, actual_err);
    assert_eq!(expected, actual);
}

#[test]
fn when_err_result_without_ok_should_output_dbg_only() {
    expect! { expected_err = "", "dbg: \"foo failed!\"\n" }

    let (_actual_out, actual_err) = capture! {
        result! {
            @when  foo_err();
            @debug "dbg: {:?}", err;
        }
    };

    assert_eq!(expected_err, actual_err);
}

#[test]
fn when_err_result_without_ok_should_output_err_only() {
    let expected_err = "err: \"foo failed!\"\n";

    let (_actual_out, actual_err) = capture! {
        result! {
            @when  foo_err();
            @error "err: {:?}", err;
        }
    };

    assert_eq!(expected_err, actual_err);
}

#[test]
fn when_ok_only_custom_result_should_output_ok() {
    let expected_out = "ok: 42\n";

    let (actual_out, _actual_err) = capture! {
        result! {
            @when  foo_ok();
            @ok    (foo) => "ok: {:?}", foo;
        }
    };

    assert_eq!(expected_out, actual_out);
}

#[test]
fn when_ok_only_custom_result_should_eval_ok() {
    let expected = 42;
    let mut actual = 0;

    result! {
        @when foo_ok();
        @ok   (foo) => { actual = foo; }
    }

    assert_eq!(expected, actual);
}

#[test]
fn when_ok_only_result_should_eval_ok() {
    let expected = 42;
    let mut actual = 0;

    result! {
        @when foo_ok();
        @ok   actual = ok;
    }

    assert_eq!(expected, actual);
}

#[test]
fn when_ok_only_result_should_output_ok() {
    let expected_out = "ok: 42\n";

    let (actual_out, _actual_err) = capture! {
        result! {
            @when  foo_ok();
            @ok    "ok: {:?}", ok;
        }
    };

    assert_eq!(expected_out, actual_out);
}

#[test]
fn when_ok_result_should_output_ok() {
    let expected_out = "ok: 42\n";

    let (actual_out, _actual_err) = capture! {
        result! {
            @when  foo_ok();
            @ok    "ok: {:?}", ok;
            @debug "dbg: this will not output {}", err;
            @error "err: this will not output {}", err;
                   println!("error expression evaluated");
        }
    };

    assert_eq!(expected_out, actual_out);
}

fn foo_ok() -> Result<usize, &'static str> {
    Ok(42)
}

fn foo_err() -> Result<(), &'static str> {
    Err("foo failed!")
}