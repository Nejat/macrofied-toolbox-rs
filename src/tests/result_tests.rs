#![allow(unused_mut)] // tests have mut value cases, however do not mutate values
#![allow(unused_variables)] // tests
#![allow(unused_assignments)] // tests

use crate::capture;
use crate::expect;
use crate::result;
#[cfg(debug_assertions)]
use cli_toolbox::debug;

const EXPECTED_BLANK: &str = "";

#[test]
fn when_ok_evaluate_expression_on_ok() {
    let expected_ok = 21;
    let mut actual_ok = 0;

        result! {
            WHEN   foo(true);
            OK     actual_ok = 21
        }

    assert_eq!(expected_ok, actual_ok);
}

#[test]
fn when_ok_evaluate_expression_on_err() {
    let expected_ok = 0;
    let mut actual_ok = 0;

        result! {
            WHEN   foo(false);
            OK     actual_ok = 21
        }

    assert_eq!(expected_ok, actual_ok);
}

#[test]
fn when_ok_evaluate_code_block_use_value_on_ok() {
    let expected_ok = 42;
    let mut actual_ok = 0;

        result! {
            WHEN   foo(true);
            OK     ok; { actual_ok = ok; }
        }

    assert_eq!(expected_ok, actual_ok);
}

#[test]
fn when_ok_evaluate_code_block_use_value_on_err() {
    let expected_ok = 0;
    let mut actual_ok = 0;

        result! {
            WHEN   foo(false);
            OK     ok; { actual_ok = ok; }
        }

    assert_eq!(expected_ok, actual_ok);
}

#[test]
fn when_ok_evaluate_expression_use_value_on_ok() {
    let expected_ok = 42;
    let mut actual_ok = 0;

        result! {
            WHEN   foo(true);
            OK     ok; actual_ok = ok
        }

    assert_eq!(expected_ok, actual_ok);
}

#[test]
fn when_ok_evaluate_expression_use_value_on_err() {
    let expected_ok = 0;
    let mut actual_ok = 0;

        result! {
            WHEN   foo(false);
            OK     ok; actual_ok = ok
        }

    assert_eq!(expected_ok, actual_ok);
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_on_ok() {
    let expected_ok = 42;
    let mut actual_ok = 0;

        result! {
            WHEN   foo(true);
            OK     mut ok; { actual_ok = ok }
        }

    assert_eq!(expected_ok, actual_ok);
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_on_err() {
    let expected_ok = 0;
    let mut actual_ok = 0;

        result! {
            WHEN   foo(false);
            OK     mut ok; { actual_ok = ok }
        }

    assert_eq!(expected_ok, actual_ok);
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_on_ok() {
    let expected_ok = 42;
    let mut actual_ok = 0;

        result! {
            WHEN   foo(true);
            OK     mut ok; actual_ok = ok
        }

    assert_eq!(expected_ok, actual_ok);
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_on_err() {
    let expected_ok = 0;
    let mut actual_ok = 0;

        result! {
            WHEN   foo(false);
            OK     mut ok; actual_ok = ok
        }

    assert_eq!(expected_ok, actual_ok);
}

#[test]
fn when_err_output_debug_message_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            DEBUG  "foo failed"
        }
    };

    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_err_output_debug_message_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed; \"something ain't right\"\n" }

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            DEBUG  "foo failed"
        }
    };

    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_err_output_formatted_debug_message_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            DEBUG  "foo failed - {}", 42
        }
    };

    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_err_output_formatted_debug_message_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; \"something ain't right\"\n" }

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            DEBUG  "foo failed - {}", 42
        }
    };

    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_err_output_debug_message_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            _DEBUG "foo failed"
        }
    };

    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_err_output_debug_message_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed\n" }

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            _DEBUG "foo failed"
        }
    };

    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_err_output_formatted_debug_message_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            _DEBUG "foo failed - {}", 42
        }
    };

    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_err_output_formatted_debug_message_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42\n" }

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            _DEBUG "foo failed - {}", 42
        }
    };

    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_err_output_custom_debug_message_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err
        }
    };

    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_err_output_custom_debug_message_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; ERR - \"something ain't right\"\n" }

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err
        }
    };

    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_err_evaluate_expression_discard_err_on_ok() {
    let expected_err = "";
    let mut actual_err = String::default();

        result! {
            WHEN   foo(true);
            ERR    actual_err = String::from("error encountered")
        }

    assert_eq!(expected_err, actual_err);
}

#[test]
fn when_err_evaluate_expression_discard_err_on_err() {
    let expected_err = "error encountered";
    let mut actual_err = String::default();

        result! {
            WHEN   foo(false);
            ERR    actual_err = String::from("error encountered")
        }

    assert_eq!(expected_err, actual_err);
}

#[test]
fn when_err_evaluate_expression_on_ok() {
    let expected_err = "";
    let mut actual_err = String::default();

        result! {
            WHEN   foo(true);
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }

    assert_eq!(expected_err, actual_err);
}

#[test]
fn when_err_evaluate_expression_on_err() {
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

        result! {
            WHEN   foo(false);
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }

    assert_eq!(expected_err, actual_err);
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_debug_message_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     { actual_ok = 21; }
            DEBUG  "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_debug_message_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     { actual_ok = 21; }
            DEBUG  "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_formatted_debug_message_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     { actual_ok = 21; }
            DEBUG  "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_formatted_debug_message_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     { actual_ok = 21; }
            DEBUG  "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_debug_message_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     { actual_ok = 21; }
            _DEBUG "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_debug_message_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     { actual_ok = 21; }
            _DEBUG "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_formatted_debug_message_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     { actual_ok = 21; }
            _DEBUG "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_formatted_debug_message_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     { actual_ok = 21; }
            _DEBUG "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_custom_debug_message_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     { actual_ok = 21; }
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_custom_debug_message_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; ERR - \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     { actual_ok = 21; }
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_debug_message_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     actual_ok = 21;
            DEBUG  "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_debug_message_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     actual_ok = 21;
            DEBUG  "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_formatted_debug_message_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     actual_ok = 21;
            DEBUG  "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_formatted_debug_message_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     actual_ok = 21;
            DEBUG  "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_debug_message_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     actual_ok = 21;
            _DEBUG "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_debug_message_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     actual_ok = 21;
            _DEBUG "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_formatted_debug_message_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     actual_ok = 21;
            _DEBUG "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_formatted_debug_message_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     actual_ok = 21;
            _DEBUG "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_custom_debug_message_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     actual_ok = 21;
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_custom_debug_message_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; ERR - \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     actual_ok = 21;
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_debug_message_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; { actual_ok = ok; }
            DEBUG  "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_debug_message_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; { actual_ok = ok; }
            DEBUG  "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_formatted_debug_message_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; { actual_ok = ok; }
            DEBUG  "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_formatted_debug_message_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; { actual_ok = ok; }
            DEBUG  "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_debug_message_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; { actual_ok = ok; }
            _DEBUG "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_debug_message_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; { actual_ok = ok; }
            _DEBUG "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_formatted_debug_message_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; { actual_ok = ok; }
            _DEBUG "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_formatted_debug_message_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; { actual_ok = ok; }
            _DEBUG "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_custom_debug_message_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; { actual_ok = ok; }
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_custom_debug_message_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; ERR - \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; { actual_ok = ok; }
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_debug_message_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; actual_ok = ok;
            DEBUG  "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_debug_message_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; actual_ok = ok;
            DEBUG  "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_formatted_debug_message_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; actual_ok = ok;
            DEBUG  "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_formatted_debug_message_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; actual_ok = ok;
            DEBUG  "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_debug_message_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; actual_ok = ok;
            _DEBUG "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_debug_message_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; actual_ok = ok;
            _DEBUG "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_formatted_debug_message_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; actual_ok = ok;
            _DEBUG "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_formatted_debug_message_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; actual_ok = ok;
            _DEBUG "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_custom_debug_message_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; actual_ok = ok;
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_custom_debug_message_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; ERR - \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; actual_ok = ok;
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_debug_message_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; { actual_ok = ok }
            DEBUG  "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_debug_message_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; { actual_ok = ok }
            DEBUG  "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_formatted_debug_message_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; { actual_ok = ok }
            DEBUG  "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_formatted_debug_message_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; { actual_ok = ok }
            DEBUG  "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_debug_message_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; { actual_ok = ok }
            _DEBUG "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_debug_message_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; { actual_ok = ok }
            _DEBUG "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_formatted_debug_message_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; { actual_ok = ok }
            _DEBUG "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_formatted_debug_message_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; { actual_ok = ok }
            _DEBUG "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_custom_debug_message_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; { actual_ok = ok }
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_custom_debug_message_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; ERR - \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; { actual_ok = ok }
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_debug_message_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; actual_ok = ok;
            DEBUG  "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_debug_message_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; actual_ok = ok;
            DEBUG  "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_formatted_debug_message_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; actual_ok = ok;
            DEBUG  "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_formatted_debug_message_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; actual_ok = ok;
            DEBUG  "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_debug_message_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; actual_ok = ok;
            _DEBUG "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_debug_message_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; actual_ok = ok;
            _DEBUG "foo failed"
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_formatted_debug_message_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; actual_ok = ok;
            _DEBUG "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_formatted_debug_message_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; actual_ok = ok;
            _DEBUG "foo failed - {}", 42
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_custom_debug_message_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; actual_ok = ok;
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_custom_debug_message_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; ERR - \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; actual_ok = ok;
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_debug_message_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     { actual_ok = 21; }
            DEBUG  "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_debug_message_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     { actual_ok = 21; }
            DEBUG  "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_debug_message_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     { actual_ok = 21; }
            DEBUG  "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_debug_message_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     { actual_ok = 21; }
            DEBUG  "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_formatted_debug_message_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     { actual_ok = 21; }
            DEBUG  "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_formatted_debug_message_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     { actual_ok = 21; }
            DEBUG  "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_formatted_debug_message_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     { actual_ok = 21; }
            DEBUG  "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_formatted_debug_message_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     { actual_ok = 21; }
            DEBUG  "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_debug_message_discard_err_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     { actual_ok = 21; }
            _DEBUG "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_debug_message_discard_err_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     { actual_ok = 21; }
            _DEBUG "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_debug_message_discard_err_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     { actual_ok = 21; }
            _DEBUG "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_debug_message_discard_err_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     { actual_ok = 21; }
            _DEBUG "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     { actual_ok = 21; }
            _DEBUG "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     { actual_ok = 21; }
            _DEBUG "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     { actual_ok = 21; }
            _DEBUG "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     { actual_ok = 21; }
            _DEBUG "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_custom_debug_message_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     { actual_ok = 21; }
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_or_when_err_output_custom_debug_message_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; ERR - \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     { actual_ok = 21; }
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_debug_message_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     actual_ok = 21;
            DEBUG  "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_debug_message_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     actual_ok = 21;
            DEBUG  "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_debug_message_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     actual_ok = 21;
            DEBUG  "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_debug_message_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     actual_ok = 21;
            DEBUG  "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_formatted_debug_message_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     actual_ok = 21;
            DEBUG  "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_formatted_debug_message_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     actual_ok = 21;
            DEBUG  "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_formatted_debug_message_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     actual_ok = 21;
            DEBUG  "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_formatted_debug_message_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     actual_ok = 21;
            DEBUG  "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_debug_message_discard_err_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     actual_ok = 21;
            _DEBUG "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_debug_message_discard_err_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     actual_ok = 21;
            _DEBUG "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_debug_message_discard_err_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     actual_ok = 21;
            _DEBUG "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_debug_message_discard_err_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     actual_ok = 21;
            _DEBUG "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     actual_ok = 21;
            _DEBUG "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     actual_ok = 21;
            _DEBUG "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     actual_ok = 21;
            _DEBUG "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     actual_ok = 21;
            _DEBUG "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_custom_debug_message_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 21;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     actual_ok = 21;
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_or_when_err_output_custom_debug_message_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; ERR - \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     actual_ok = 21;
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_debug_message_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; { actual_ok = ok; }
            DEBUG  "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_debug_message_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; { actual_ok = ok; }
            DEBUG  "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_debug_message_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; { actual_ok = ok; }
            DEBUG  "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_debug_message_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; { actual_ok = ok; }
            DEBUG  "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_formatted_debug_message_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; { actual_ok = ok; }
            DEBUG  "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_formatted_debug_message_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; { actual_ok = ok; }
            DEBUG  "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_formatted_debug_message_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; { actual_ok = ok; }
            DEBUG  "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_formatted_debug_message_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; { actual_ok = ok; }
            DEBUG  "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_debug_message_discard_err_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; { actual_ok = ok; }
            _DEBUG "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_debug_message_discard_err_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; { actual_ok = ok; }
            _DEBUG "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_debug_message_discard_err_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; { actual_ok = ok; }
            _DEBUG "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_debug_message_discard_err_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; { actual_ok = ok; }
            _DEBUG "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; { actual_ok = ok; }
            _DEBUG "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; { actual_ok = ok; }
            _DEBUG "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; { actual_ok = ok; }
            _DEBUG "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; { actual_ok = ok; }
            _DEBUG "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_custom_debug_message_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; { actual_ok = ok; }
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_value_or_when_err_output_custom_debug_message_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; ERR - \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; { actual_ok = ok; }
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_debug_message_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; actual_ok = ok;
            DEBUG  "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_debug_message_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; actual_ok = ok;
            DEBUG  "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_debug_message_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; actual_ok = ok;
            DEBUG  "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_debug_message_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; actual_ok = ok;
            DEBUG  "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_formatted_debug_message_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; actual_ok = ok;
            DEBUG  "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_formatted_debug_message_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; actual_ok = ok;
            DEBUG  "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_formatted_debug_message_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; actual_ok = ok;
            DEBUG  "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_formatted_debug_message_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; actual_ok = ok;
            DEBUG  "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_debug_message_discard_err_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; actual_ok = ok;
            _DEBUG "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_debug_message_discard_err_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; actual_ok = ok;
            _DEBUG "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_debug_message_discard_err_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; actual_ok = ok;
            _DEBUG "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_debug_message_discard_err_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; actual_ok = ok;
            _DEBUG "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; actual_ok = ok;
            _DEBUG "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; actual_ok = ok;
            _DEBUG "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; actual_ok = ok;
            _DEBUG "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; actual_ok = ok;
            _DEBUG "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_custom_debug_message_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     ok; actual_ok = ok;
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_value_or_when_err_output_custom_debug_message_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; ERR - \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     ok; actual_ok = ok;
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_debug_message_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; { actual_ok = ok }
            DEBUG  "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_debug_message_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; { actual_ok = ok }
            DEBUG  "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_debug_message_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; { actual_ok = ok }
            DEBUG  "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_debug_message_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; { actual_ok = ok }
            DEBUG  "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_formatted_debug_message_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; { actual_ok = ok }
            DEBUG  "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_formatted_debug_message_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; { actual_ok = ok }
            DEBUG  "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_formatted_debug_message_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; { actual_ok = ok }
            DEBUG  "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_formatted_debug_message_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; { actual_ok = ok }
            DEBUG  "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_debug_message_discard_err_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; { actual_ok = ok }
            _DEBUG "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_debug_message_discard_err_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; { actual_ok = ok }
            _DEBUG "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_debug_message_discard_err_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; { actual_ok = ok }
            _DEBUG "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_debug_message_discard_err_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; { actual_ok = ok }
            _DEBUG "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; { actual_ok = ok }
            _DEBUG "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; { actual_ok = ok }
            _DEBUG "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; { actual_ok = ok }
            _DEBUG "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; { actual_ok = ok }
            _DEBUG "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_custom_debug_message_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; { actual_ok = ok }
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_code_block_use_mutable_value_or_when_err_output_custom_debug_message_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; ERR - \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; { actual_ok = ok }
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_debug_message_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; actual_ok = ok;
            DEBUG  "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_debug_message_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; actual_ok = ok;
            DEBUG  "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_debug_message_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; actual_ok = ok;
            DEBUG  "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_debug_message_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; actual_ok = ok;
            DEBUG  "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_formatted_debug_message_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; actual_ok = ok;
            DEBUG  "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_formatted_debug_message_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; actual_ok = ok;
            DEBUG  "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_formatted_debug_message_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; actual_ok = ok;
            DEBUG  "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_formatted_debug_message_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; actual_ok = ok;
            DEBUG  "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_debug_message_discard_err_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; actual_ok = ok;
            _DEBUG "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_debug_message_discard_err_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; actual_ok = ok;
            _DEBUG "foo failed";
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_debug_message_discard_err_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; actual_ok = ok;
            _DEBUG "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_debug_message_discard_err_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; actual_ok = ok;
            _DEBUG "foo failed";
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; actual_ok = ok;
            _DEBUG "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; actual_ok = ok;
            _DEBUG "foo failed - {}", 42;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; actual_ok = ok;
            _DEBUG "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_formatted_debug_message_discard_err_then_evaluate_expression_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "ERR: \"something ain't right\"";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; actual_ok = ok;
            _DEBUG "foo failed - {}", 42;
            ERR    err; actual_err = format!("ERR: {:?}", err)
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_custom_debug_message_then_evaluate_expression_discard_err_on_ok() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, EXPECTED_BLANK }
    let expected_ok = 42;
    let mut actual_ok = 0;
    let expected_err = "";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(true);
            OK     mut ok; actual_ok = ok;
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

#[test]
fn when_ok_evaluate_expression_use_mutable_value_or_when_err_output_custom_debug_message_then_evaluate_expression_discard_err_on_err() {
    expect! { EXPECTED_DBG: &str => EXPECTED_BLANK, "ERROR: foo failed - 42; ERR - \"something ain't right\"\n" }
    let expected_ok = 0;
    let mut actual_ok = 0;
    let expected_err = "error encountered";
    let mut actual_err = String::default();

    let (actual_out, actual_dbg) = capture! {
        result! {
            WHEN   foo(false);
            OK     mut ok; actual_ok = ok;
            DEBUG  err; "foo failed - {}; ERR - {:?}", 42, err;
            ERR    actual_err = String::from("error encountered")
        }
    };

    assert_eq!(expected_ok, actual_ok);
    assert_eq!(expected_err, actual_err);
    assert_eq!(EXPECTED_DBG, actual_dbg);

    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );
}

fn foo(succeed: bool) -> Result<usize, &'static str> {
    if succeed { Ok(42) } else { Err("something ain't right") }
}
