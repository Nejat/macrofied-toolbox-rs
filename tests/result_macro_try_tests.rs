#![allow(unused_assignments)]

use test_toolbox::{capture, expect};

use macrofied_toolbox::result;

type TestResult = Result<usize, &'static str>;

#[test]
fn when_err_result_with_ok_should_eval_err_only() {
    let expected = Err("foo failed!");
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestResult {
        let expected = "foo failed!";
        let mut actual = "";

        result! {
            @when  foo_err()?
            @ok    "this will not output: {:?}", ok
            @error { actual = err; }
        }

        assert_eq!(expected, actual);

        Ok(0)
    }
}

#[test]
fn when_err_result_with_ok_should_output_and_eval_err_only() {
    let expected = Err("foo failed!");
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestResult {
        let expected_err = "err: \"foo failed!\"\n";
        let expected = "foo failed!";
        let mut actual = "";

        let (_actual_out, actual_err) = capture! {
            result! {
                @when  foo_err()?
                @ok    "this will not output: {:?}", ok
                @error "err: {:?}", err;
                       actual = err
            }
        };

        assert_eq!(expected, actual);
        assert_eq!(expected_err, actual_err);

        Ok(0)
    }
}

#[test]
fn when_err_result_with_ok_should_output_dbg_and_err() {
    let expected = Err("foo failed!");
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestResult {
        expect! { expected_err = "err: foo failed!\n", "dbg: \"foo failed!\"\nerr: foo failed!\n" }

        let (_actual_out, actual_err) = capture! {
            result! {
                @when  foo_err()?
                @ok    "this will not output: {:?}", ok
                @debug "dbg: {:?}", err
                @error "err: {}", err
            }
        };

        assert_eq!(expected_err, actual_err);

        Ok(0)
    }
}

#[test]
fn when_err_result_with_ok_should_output_dbg_and_err_and_eval_err () {
    let expected = Err("foo failed!");
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestResult {
        expect! { expected_err = "err: foo failed!\n", "dbg: \"foo failed!\"\nerr: foo failed!\n" }

        let expected = "foo failed!";
        let mut actual = "";

        let (_actual_out, actual_err) = capture! {
            result! {
                @when  foo_err()?
                @ok    "this will not output: {:?}", ok
                @debug "dbg: {:?}", err
                @error "err: {}", err;
                       actual = err
            }
        };

        assert_eq!(expected_err, actual_err);
        assert_eq!(expected, actual);

        Ok(0)
    }
}

#[test]
fn when_err_result_with_ok_should_output_dbg_only() {
    let expected = Err("foo failed!");
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestResult {
        expect! { expected_err = "", "dbg: \"foo failed!\"\n" }

        let (_actual_out, actual_err) = capture! {
            result! {
                @when  foo_err()?
                @ok    "this will not output: {:?}", ok
                @debug "dbg: {:?}", err
            }
        };

        assert_eq!(expected_err, actual_err);

        Ok(0)
    }
}

#[test]
fn when_err_result_with_ok_should_output_err_only() {
    let expected = Err("foo failed!");
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestResult {
        let expected_err = "err: \"foo failed!\"\n";

        let (_actual_out, actual_err) = capture! {
            result! {
                @when  foo_err()?
                @ok    "this will not output: {:?}", ok
                @error "err: {:?}", err
            }
        };

        assert_eq!(expected_err, actual_err);

        Ok(0)
    }
}

#[test]
fn when_err_result_without_ok_should_eval_err_only() {
    let expected = Err("foo failed!");
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestResult {
        let expected = "foo failed!";
        let mut actual = "";

        result! {
            @when  foo_err()?
            @error { actual = err; }
        }

        assert_eq!(expected, actual);

        Ok(0)
    }
}

#[test]
fn when_err_result_without_ok_should_output_and_eval_err_only() {
    let expected = Err("foo failed!");
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestResult {
        let expected_err = "err: \"foo failed!\"\n";
        let expected = "foo failed!";
        let mut actual = "";

        let (_actual_out, actual_err) = capture! {
            result! {
                @when  foo_err()?
                @error "err: {:?}", err;
                       actual = err
            }
        };

        assert_eq!(expected, actual);
        assert_eq!(expected_err, actual_err);

        Ok(0)
    }
}

#[test]
fn when_err_result_without_ok_should_output_dbg_and_err() {
    let expected = Err("foo failed!");
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestResult {
        expect! { expected_err = "err: foo failed!\n", "dbg: \"foo failed!\"\nerr: foo failed!\n" }

        let (_actual_out, actual_err) = capture! {
            result! {
                @when  foo_err()?
                @debug "dbg: {:?}", err
                @error "err: {}", err
            }
        };

        assert_eq!(expected_err, actual_err);

        Ok(0)
    }
}

#[test]
fn when_err_result_without_ok_should_output_dbg_and_err_and_eval_err () {
    let expected = Err("foo failed!");
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestResult {
        expect! { expected_err = "err: foo failed!\n", "dbg: \"foo failed!\"\nerr: foo failed!\n" }

        let expected = "foo failed!";
        let mut actual = "";

        let (_actual_out, actual_err) = capture! {
            result! {
                @when  foo_err()?
                @debug "dbg: {:?}", err
                @error "err: {}", err;
                       actual = err
            }
        };

        assert_eq!(expected_err, actual_err);
        assert_eq!(expected, actual);

        Ok(0)
    }
}

#[test]
fn when_err_result_without_ok_should_output_dbg_only() {
    let expected = Err("foo failed!");
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestResult {
        expect! { expected_err = "", "dbg: \"foo failed!\"\n" }

        let (_actual_out, actual_err) = capture! {
            result! {
                @when  foo_err()?
                @debug "dbg: {:?}", err
            }
        };

        assert_eq!(expected_err, actual_err);

        Ok(0)
    }
}

#[test]
fn when_err_result_without_ok_should_output_err_only() {
    let expected = Err("foo failed!");
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestResult {
        let expected_err = "err: \"foo failed!\"\n";

        let (_actual_out, actual_err) = capture! {
            result! {
                @when  foo_err()?
                @error "err: {:?}", err
            }
        };

        assert_eq!(expected_err, actual_err);

        Ok(0)
    }
}

#[test]
fn when_ok_only_custom_result_should_output_ok() {
    let expected = Ok(0);
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestResult {
        let expected_out = "ok: 42\n";

        let (actual_out, _actual_err) = capture! {
            result! {
                @when  foo_ok()?
                @ok    (foo) => "ok: {:?}", foo
            }
        };

        assert_eq!(expected_out, actual_out);

        Ok(0)
    }
}

#[test]
fn when_ok_only_custom_result_should_eval_ok() {
    let expected = Ok(42);
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestResult {
        let expected = 42;
        let mut actual = 0;

        result! {
            @when foo_ok()?
            @ok   (foo) => { actual = foo; }
        }

        assert_eq!(expected, actual);

        Ok(actual)
    }
}

#[test]
fn when_ok_only_result_should_eval_ok() {
    let expected = Ok(42);
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestResult {
        let expected = 42;
        let mut actual = 0;

        result! {
            @when foo_ok()?
            @ok   actual = ok
        }

        assert_eq!(expected, actual);

        Ok(actual)
    }
}

#[test]
fn when_ok_only_result_should_output_ok() {
    let expected = Ok(0);
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestResult {
        let expected_out = "ok: 42\n";

        let (actual_out, _actual_err) = capture! {
            result! {
                @when  foo_ok()?
                @ok    "ok: {:?}", ok
            }
        };

        assert_eq!(expected_out, actual_out);

        Ok(0)
    }
}

#[test]
fn when_ok_result_should_output_ok() {
    let expected = Ok(0);
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestResult {
        let expected_out = "ok: 42\n";

        let (actual_out, _actual_err) = capture! {
            result! {
                @when  foo_ok()?
                @ok    "ok: {:?}", ok
                @debug "dbg: this will not output {}", err
                @error "err: this will not output {}", err;
                       println!("error expression evaluated")
            }
        };

        assert_eq!(expected_out, actual_out);

        Ok(0)
    }
}

fn foo_ok() -> TestResult {
    Ok(42)
}

fn foo_err() -> TestResult {
    Err("foo failed!")
}