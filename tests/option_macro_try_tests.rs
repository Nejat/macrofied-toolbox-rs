#![allow(unused_assignments)]

use cfg_if::cfg_if;
use test_toolbox::capture;
#[cfg(feature = "option-debug")]
use test_toolbox::expect;

use macrofied_toolbox::option;

type TestOption = Option<usize>;

#[test]
fn when_none_option_with_some_should_eval_none_only() {
    let expected = None;
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestOption {
        let expected = false;
        let mut actual = true;

        option! {
            @when foo_none()?
            @some "this will not output: {:?}", some
            @none { actual = false; }
        }

        assert_eq!(expected, actual);

        Some(0)
    }
}

#[test]
fn when_none_option_with_some_should_output_and_eval_none_only() {
    let expected = None;
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestOption {
        let expected_stderr = "none: example message\n";
        let expected = false;
        let mut actual = true;

        let (_actual_stdout, actual_stderr) = capture! {
            option! {
                @when foo_none()?
                @some "this will not output: {:?}", some
                @none "none: example message";
                      actual = false
            }
        };

        assert_eq!(expected, actual);
        assert_eq!(expected_stderr, actual_stderr);

        Some(0)
    }
}

#[test]
fn when_none_option_with_some_should_output_dbg_and_none() {
    let expected = None;
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestOption {
        cfg_if! {
            if #[cfg(feature = "option-debug")] {
                expect! { expected_stdout = "", "dbg: example message\n" }
            } else {
                let expected_stdout = "";
            }
        }

        let expected_stderr = "none: example message\n";

        let (actual_stdout, actual_stderr) = capture! {
            option! {
                @when  foo_none()?
                @some  "this will not output: {:?}", some
                @debug "dbg: example message"
                @none  "none: example message"
            }
        };

        assert_eq!(expected_stdout, actual_stdout);
        assert_eq!(expected_stderr, actual_stderr);

        Some(0)
    }
}

#[test]
fn when_none_option_with_some_should_output_dbg_and_none_and_eval_err() {
    let expected = None;
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestOption {
        cfg_if! {
            if #[cfg(feature = "option-debug")] {
                expect! { expected_stdout = "", "dbg: example message\n" }
            } else {
                let expected_stdout = "";
            }
        }

        let expected_stderr = "none: example message\n";
        let expected = false;
        let mut actual = true;

        let (actual_stdout, actual_stderr) = capture! {
            option! {
                @when  foo_none()?
                @some  "this will not output: {:?}", some
                @debug "dbg: example message"
                @none  "none: example message";
                       actual = false
            }
        };

        assert_eq!(expected_stdout, actual_stdout);
        assert_eq!(expected_stderr, actual_stderr);
        assert_eq!(expected, actual);

        Some(0)
    }
}

#[test]
fn when_none_option_with_some_should_output_dbg_only() {
    let expected = None;
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestOption {
        cfg_if! {
            if #[cfg(feature = "option-debug")] {
                expect! { expected_stdout = "", "dbg: example message\n" }
            } else {
                let expected_stdout = "";
            }
        }

        let (actual_stdout, _actual_stderr) = capture! {
            option! {
                @when  foo_none()?
                @some  "this will not output: {:?}", some
                @debug "dbg: example message"
            }
        };

        assert_eq!(expected_stdout, actual_stdout);

        Some(0)
    }
}

#[test]
fn when_none_option_with_some_should_output_none_only() {
    let expected = None;
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestOption {
        let expected_stderr = "none: example message\n";

        let (_actual_stdout, actual_stderr) = capture! {
            option! {
                @when foo_none()?
                @some "this will not output: {:?}", some
                @none "none: example message"
            }
        };

        assert_eq!(expected_stderr, actual_stderr);

        Some(0)
    }
}

#[test]
fn when_none_option_without_some_should_eval_none_only() {
    let expected = None;
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestOption {
        let expected = false;
        let mut actual = true;

        option! {
            @when foo_none()?
            @none { actual = false; }
        }

        assert_eq!(expected, actual);

        Some(0)
    }
}

#[test]
fn when_none_option_without_some_should_output_and_eval_none_only() {
    let expected = None;
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestOption {
        let expected_stderr = "none: example message\n";
        let expected = false;
        let mut actual = true;

        let (_actual_stdout, actual_stderr) = capture! {
            option! {
                @when  foo_none()?
                @none "none: example message";
                       actual = false
            }
        };

        assert_eq!(expected, actual);
        assert_eq!(expected_stderr, actual_stderr);

        Some(0)
    }
}

#[test]
fn when_none_option_without_some_should_output_dbg_and_none() {
    let expected = None;
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestOption {
        cfg_if! {
            if #[cfg(feature = "option-debug")] {
                expect! { expected_stdout = "", "dbg: example message\n" }
            } else {
                let expected_stdout = "";
            }
        }

        let expected_stderr = "none: example message\n";

        let (actual_stdout, actual_stderr) = capture! {
            option! {
                @when  foo_none()?
                @debug "dbg: example message"
                @none  "none: example message"
            }
        };

        assert_eq!(expected_stdout, actual_stdout);
        assert_eq!(expected_stderr, actual_stderr);

        Some(0)
    }
}

#[test]
fn when_none_option_without_some_should_output_dbg_and_none_and_eval_err() {
    let expected = None;
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestOption {
        cfg_if! {
            if #[cfg(feature = "option-debug")] {
                expect! { expected_stdout = "", "dbg: example message\n" }
            } else {
                let expected_stdout = "";
            }
        }

        let expected_stderr = "none: example message\n";
        let expected = false;
        let mut actual = true;

        let (actual_stdout, actual_stderr) = capture! {
            option! {
                @when  foo_none()?
                @debug "dbg: example message"
                @none  "none: example message";
                       actual = false
            }
        };

        assert_eq!(expected_stdout, actual_stdout);
        assert_eq!(expected_stderr, actual_stderr);
        assert_eq!(expected, actual);

        Some(0)
    }
}

#[test]
fn when_none_option_without_some_should_output_dbg_only() {
    let expected = None;
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestOption {
        cfg_if! {
            if #[cfg(feature = "option-debug")] {
                expect! { expected_stdout = "", "dbg: example message\n" }
            } else {
                let expected_stdout = "";
            }
        }

        let (actual_stdout, _actual_stderr) = capture! {
            option! {
                @when  foo_none()?
                @debug "dbg: example message"
            }
        };

        assert_eq!(expected_stdout, actual_stdout);

        Some(0)
    }
}

#[test]
fn when_none_option_without_some_should_output_none_only() {
    let expected = None;
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestOption {
        let expected_stderr = "none: example message\n";

        let (_actual_stdout, actual_stderr) = capture! {
            option! {
                @when foo_none()?
                @none "none: example message"
            }
        };

        assert_eq!(expected_stderr, actual_stderr);

        Some(0)
    }
}

#[test]
fn when_some_only_custom_result_should_output_some() {
    let expected = Some(42);
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestOption {
        let expected_stdout = "some: 42\n";

        let (actual_stdout, _actual_stderr) = capture! {
            option! {
                @when foo_some()?
                @some (foo) => "some: {}", foo
            }
        };

        assert_eq!(expected_stdout, actual_stdout);

        Some(42)
    }
}

#[test]
fn when_some_only_custom_result_should_eval_some() {
    let expected = Some(42);
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestOption {
        let expected = 42;
        let mut actual = 0;

        option! {
            @when foo_some()?
            @some (foo) => { actual = foo; }
        }

        assert_eq!(expected, actual);

        Some(actual)
    }
}

#[test]
fn when_some_only_result_should_eval_some() {
    let expected = Some(42);
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestOption {
        let expected = 42;
        let mut actual = 0;

        option! {
            @when foo_some()?
            @some actual = some
        }

        assert_eq!(expected, actual);

        Some(actual)
    }
}

#[test]
fn when_some_only_result_should_output_some() {
    let expected = Some(42);
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestOption {
        let expected_stdout = "some: 42\n";

        let (actual_stdout, _actual_stderr) = capture! {
            option! {
                @when foo_some()?
                @some (baz) => "some: {}", baz
            }
        };

        assert_eq!(expected_stdout, actual_stdout);

        Some(42)
    }
}

#[test]
fn when_some_result_should_output_some() {
    let expected = Some(42);
    let actual = test_try();

    assert_eq!(expected, actual);

    fn test_try() -> TestOption {
        let expected_stdout = "some: 42\n";

        let (actual_stdout, _actual_stderr) = capture! {
            option! {
                @when  foo_some()?
                @some  (baz) => "some: {}", baz
                @debug "dbg: this will not output"
                @none  "none: this will not output";
                       println!("none expression evaluated")
            }
        };

        assert_eq!(expected_stdout, actual_stdout);

        Some(42)
    }
}

fn foo_some() -> TestOption {
    Some(42)
}

fn foo_none() -> TestOption {
    None
}