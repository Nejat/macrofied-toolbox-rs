use cfg_if::cfg_if;
use test_toolbox::capture;
#[cfg(feature = "result-debug")]
use test_toolbox::expect;

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
    let expected_stderr = "err: \"foo failed!\"\n";
    let expected = "foo failed!";
    let mut actual = "";

    let (_actual_stdout, actual_stderr) = capture! {
        result! {
            @when  foo_err();
            @ok    "this will not output: {:?}", ok;
            @error "err: {:?}", err;
                   actual = err;
        }
    };

    assert_eq!(expected, actual);
    assert_eq!(expected_stderr, actual_stderr);
}

#[test]
fn when_err_result_with_ok_should_output_dbg_and_err() {
    cfg_if! {
        if #[cfg(feature = "result-debug")] {
            expect! { expected_stdout = "", "dbg: \"foo failed!\"\n" }
        } else {
            let expected_stdout = "";
        }
    }

    let expected_stderr = "err: foo failed!\n";

    let (actual_stdout, actual_stderr) = capture! {
        result! {
            @when  foo_err();
            @ok    "this will not output: {:?}", ok;
            @debug "dbg: {:?}", err;
            @error "err: {}", err;
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(expected_stderr, actual_stderr);
}

#[test]
fn when_err_result_with_ok_should_output_dbg_and_err_and_eval_err() {
    cfg_if! {
        if #[cfg(feature = "result-debug")] {
            expect! { expected_stdout = "", "dbg: \"foo failed!\"\n" }
        } else {
            let expected_stdout = "";
        }
    }

    let expected_stderr = "err: foo failed!\n";
    let expected = "foo failed!";
    let mut actual = "";

    let (actual_stdout, actual_stderr) = capture! {
        result! {
            @when  foo_err();
            @ok    "this will not output: {:?}", ok;
            @debug "dbg: {:?}", err;
            @error "err: {}", err;
                   actual = err;
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(expected_stderr, actual_stderr);
    assert_eq!(expected, actual);
}

#[test]
fn when_err_result_with_ok_should_output_dbg_only() {
    cfg_if! {
        if #[cfg(feature = "result-debug")] {
            expect! { expected_stdout = "", "dbg: \"foo failed!\"\n" }
        } else {
            let expected_stdout = "";
        }
    }

    let (actual_stdout, _actual_stderr) = capture! {
        result! {
            @when  foo_err();
            @ok    "this will not output: {:?}", ok;
            @debug "dbg: {:?}", err;
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
}

#[test]
fn when_err_result_with_ok_should_output_err_only() {
    let expected_stderr = "err: \"foo failed!\"\n";

    let (_actual_stdout, actual_stderr) = capture! {
        result! {
            @when  foo_err();
            @ok    "this will not output: {:?}", ok;
            @error "err: {:?}", err;
        }
    };

    assert_eq!(expected_stderr, actual_stderr);
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
    let expected_stderr = "err: \"foo failed!\"\n";
    let expected = "foo failed!";
    let mut actual = "";

    let (_actual_stdout, actual_stderr) = capture! {
        result! {
            @when  foo_err();
            @error "err: {:?}", err;
                   actual = err;
        }
    };

    assert_eq!(expected, actual);
    assert_eq!(expected_stderr, actual_stderr);
}

#[test]
fn when_err_result_without_ok_should_output_dbg_and_err() {
    cfg_if! {
        if #[cfg(feature = "result-debug")] {
            expect! { expected_stdout = "", "dbg: \"foo failed!\"\n" }
        } else {
            let expected_stdout = "";
        }
    }

    let expected_stderr = "err: foo failed!\n";

    let (actual_stdout, actual_stderr) = capture! {
        result! {
            @when  foo_err();
            @debug "dbg: {:?}", err;
            @error "err: {}", err;
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(expected_stderr, actual_stderr);
}

#[test]
fn when_err_result_without_ok_should_output_dbg_and_err_and_eval_err() {
    cfg_if! {
        if #[cfg(feature = "result-debug")] {
            expect! { expected_stdout = "", "dbg: \"foo failed!\"\n" }
        } else {
            let expected_stdout = "";
        }
    }

    let expected_stderr = "err: foo failed!\n";
    let expected = "foo failed!";
    let mut actual = "";

    let (actual_stdout, actual_stderr) = capture! {
        result! {
            @when  foo_err();
            @debug "dbg: {:?}", err;
            @error "err: {}", err;
                   actual = err;
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(expected_stderr, actual_stderr);
    assert_eq!(expected, actual);
}

#[test]
fn when_err_result_without_ok_should_output_dbg_only() {
    cfg_if! {
        if #[cfg(feature = "result-debug")] {
            expect! { expected_stdout = "", "dbg: \"foo failed!\"\n" }
        } else {
            let expected_stdout = "";
        }
    }

    let (actual_stdout, _actual_stderr) = capture! {
        result! {
            @when  foo_err();
            @debug "dbg: {:?}", err;
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
}

#[test]
fn when_err_result_without_ok_should_output_err_only() {
    let expected_stderr = "err: \"foo failed!\"\n";

    let (_actual_stdout, actual_stderr) = capture! {
        result! {
            @when  foo_err();
            @error "err: {:?}", err;
        }
    };

    assert_eq!(expected_stderr, actual_stderr);
}

#[test]
fn when_ok_only_custom_result_should_output_ok() {
    let expected_stdout = "ok: 42\n";

    let (actual_stdout, _actual_stderr) = capture! {
        result! {
            @when  foo_ok();
            @ok    (foo) => "ok: {:?}", foo;
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
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
    let expected_stdout = "ok: 42\n";

    let (actual_stdout, _actual_stderr) = capture! {
        result! {
            @when  foo_ok();
            @ok    "ok: {:?}", ok;
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
}

#[test]
fn when_ok_result_should_output_ok() {
    let expected_stdout = "ok: 42\n";

    let (actual_stdout, _actual_stderr) = capture! {
        result! {
            @when  foo_ok();
            @ok    "ok: {:?}", ok;
            @debug "dbg: this will not output {}", err;
            @error "err: this will not output {}", err;
                   println!("error expression evaluated");
        }
    };

    assert_eq!(expected_stdout, actual_stdout);
}

fn foo_ok() -> Result<usize, &'static str> {
    Ok(42)
}

fn foo_err() -> Result<(), &'static str> {
    Err("foo failed!")
}