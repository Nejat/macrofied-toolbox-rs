use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

use OnErr::*;
use OnErrDebug::*;
use OnOk::*;
use ResultCase::*;

type Configuration = (OnOk, OnErrDebug, OnErr);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum OnOk {
    NoOk,
    // on ok evaluate expression
    OkExpr,
    // on ok evaluate expression w/value
    OkExprVal,
    // on ok evaluate expression w/mutable value
    OkExprMutVal,
    // on ok evaluate code block
    OkBlk,
    // on ok evaluate code block w/value
    OkBlkVal,
    // on ok evaluate code block w/mutable value
    OkBlkMutVal,
}

impl OnOk {
    pub(crate) fn discards_ok(self) -> bool {
        self == OkExpr || self == OkBlk
    }

    pub(crate) fn is_expr(self) -> bool {
        self == OkExpr || self == OkExprVal || self == OkExprMutVal
    }

    pub(crate) fn mutable_value(self) -> bool {
        self == OkExprMutVal || self == OkBlkMutVal
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum OnErrDebug {
    NoDbg,
    // on err output debug message
    Dbg,
    // on err output formatted debug message
    DbgFmt,
    // on err output debug message, discard err value
    DbgNoErr,
    // on err output formatted debug message, discard err value
    DbgFmtNoErr,
    // on err output custom debug err message
    DbgCustomErr,
}

impl OnErrDebug {
    pub(crate) fn discards_err(self) -> bool {
        self == DbgNoErr || self == DbgFmtNoErr
    }

    pub(crate) fn is_formatted(self) -> bool {
        // dbg_args || _dbg_args || dbg_err_args
        self == DbgFmt || self == DbgFmtNoErr || self == DbgCustomErr
    }

    pub(crate) fn outputs_err(self) -> bool {
        // dbg || dbg_args
        self == Dbg || self == DbgFmt
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum OnErr {
    NoErr,
    // on err evaluate expression
    ErrExpr,
    // on err evaluate expression w/error
    ErrExprErr,
}

impl OnErr {
    pub(crate) fn discards_err(self) -> bool {
        // err && !dbg_err_args
        self == ErrExpr
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum ResultCase {
    OkCase,
    ErrCase
}

const RESULT_ERROR: &str = "something ain't right";

pub fn generate_result_macro() -> io::Result<()> {
    let result_source_file = File::create("src/result.rs")?;
    let mut out = BufWriter::new(result_source_file);

    writeln!(out, r#"/// The `result!` macro provides ergonomic handling of debug messages when
/// dealing with `Result<T,E>` return values.
///
/// Just like the [`cli-toolbox`](https://crates.io/crates/cli-toolbox) crate, that the debug logic
/// is based on, this is not a logging alternative; it's intended to produce debugging output to be
/// used during application development.
///
/// Although this macro was designed to make debugging more ergonomic, it includes variations
/// that do not include debugging to provide coding consistency, so you can use the same syntax
/// consistently through out your crate.
/// \
/// \
/// \* _debugging output for OK results also makes sense and can be added in the future_\
/// \*\* _this macro is automatically generated in a custom build script, including_ `docs`
///
/// # Features
///
/// * you can output basic or formatted debugging information for `Err` results of an expression
///     * the `Err` value is appended to the debugging output
///     * you can discard the `Err` value and not append it to the debugging output
///     * you can obtain the `Err` value and provide custom error reporting
/// * you can evaluate code on `Ok` results of an expression
///     * you can obtain the `Ok` value
///     * you can discard the `Ok` value
/// * you can evaluate code on `Err` results of an expression
///     * you can obtain the `Err` value
///     * you can discard the `Err` value
///
/// # Examples
///"#)?;

    macro_doc_examples(&mut out)?;

    writeln!(out, "#[macro_export]")?;
    writeln!(out, "macro_rules! result {{")?;

    for configuration in macro_configurations() {
        let (on_ok, on_dbg, on_err, _has_ok, has_debug, has_error) = destructure(configuration);

        comment(&mut out, configuration, "    // ", "    // ", "")?;

        writeln!(out, "    (")?;
        writeln!(out, "        WHEN   $when:expr;")?;

        match on_ok {
            NoOk => {}
            OkExpr =>
                write!(out, "        OK     $on_ok:expr")?,
            OkExprVal =>
                write!(out, "        OK     $ok:ident; $on_ok:expr")?,
            OkExprMutVal =>
                write!(out, "        OK     mut $ok:ident; $on_ok:expr")?,
            OkBlk =>
                writeln!(out, "        OK     $on_ok:block")?,
            OkBlkVal =>
                writeln!(out, "        OK     $ok:ident; $on_ok:block")?,
            OkBlkMutVal =>
                writeln!(out, "        OK     mut $ok:ident; $on_ok:block")?
        }

        if on_ok.is_expr() {
            if has_debug || has_error {
                writeln!(out, ";")?;
            } else {
                writeln!(out)?;
            }
        }

        match on_dbg {
            NoDbg => {}
            Dbg => {}
            DbgFmt => {}
            DbgNoErr => {}
            DbgFmtNoErr => {}
            DbgCustomErr => {}
        }

        match on_dbg {
            NoDbg => {}
            Dbg =>
                write!(out, "        DEBUG  $dbg:expr")?,
            DbgFmt =>
                write!(out, "        DEBUG  $dbg:expr, $($arg:expr),+")?,
            DbgNoErr =>
                write!(out, "        _DEBUG $dbg:expr")?,
            DbgFmtNoErr =>
                write!(out, "        _DEBUG $dbg:expr, $($arg:expr),+")?,
            DbgCustomErr =>
                write!(out, "        DEBUG  $err:ident; $dbg:expr, $($arg:expr),+")?
        }

        if has_debug {
            if has_error {
                writeln!(out, ";")?;
            } else {
                writeln!(out)?;
            }
        }

        match on_err {
            NoErr => {}
            ErrExpr =>
                writeln!(out, "        ERR    $on_err:expr")?,
            ErrExprErr =>
                writeln!(out, "        ERR    $err:ident; $on_err:expr")?
        }

        writeln!(out, "    ) => {{")?;

        macro_logic(&mut out, configuration)?;

        writeln!(out, "    }};")?;
    }

    writeln!(out, "}}")
}

pub fn generate_result_macro_tests() -> io::Result<()> {
    let result_source_file = File::create("src/tests/result_tests.rs")?;
    let mut out = BufWriter::new(result_source_file);

    writeln!(out, "#![allow(unused_mut)] // tests have mut value cases, however do not mutate values")?;
    writeln!(out, "#![allow(unused_variables)] // tests")?;
    writeln!(out, "#![allow(unused_assignments)] // tests")?;
    writeln!(out)?;
    writeln!(out, "use crate::capture;")?;
    writeln!(out, "use crate::expect;")?;
    writeln!(out, "use crate::result;")?;
    writeln!(out, "#[cfg(debug_assertions)]")?;
    writeln!(out, "use cli_toolbox::debug;")?;
    writeln!(out)?;
    writeln!(out, "const EXPECTED_BLANK: &str = \"\";")?;
    writeln!(out)?;

    for configuration in macro_configurations() {
        const ON_OK_CASE: &str = "_on_ok";
        const ON_ERR_CASE: &str = "_on_err";

        let mut test_name = test_name(configuration);

        test_name.push_str(ON_OK_CASE);

        write_test_case(&mut out, OkCase, &test_name, configuration)?;

        test_name = test_name.replace(ON_OK_CASE, ON_ERR_CASE);

        write_test_case(&mut out, ErrCase, &test_name, configuration)?;
    }

    writeln!(out, r#"fn foo(succeed: bool) -> Result<usize, &'static str> {{
    if succeed {{ Ok(42) }} else {{ Err({:?}) }}
}}"#, RESULT_ERROR)?;

    Ok(())
}

fn write_test_case<W: Write>(
    out: &mut W, case: ResultCase, case_name: &str, configuration: Configuration
) -> io::Result<()> {
    let (on_ok, on_dbg, on_err, has_ok, has_debug, has_error) = destructure(configuration);

    writeln!(out, "#[test]")?;
    writeln!(out, "fn {}() {{", case_name)?;

    let write_expected = |out: &mut W| {
        if has_ok {
            let expected_ok = match on_ok {
                OkExpr | OkBlk if case == OkCase => 21,
                OkExpr | OkBlk if case == ErrCase => 0,
                _ if case == OkCase  => 42,
                _ if case == ErrCase  => 0,
                _ => unreachable!()
            };

            writeln!(out, "    let expected_ok = {};", expected_ok)?;
            writeln!(out, "    let mut actual_ok = 0;")?;
        }

        if has_error {
            let expected_err = match on_err {
                ErrExpr if case == ErrCase => String::from("error encountered"),
                _ if case == OkCase  => String::default(),
                _ if case == ErrCase  => format!("ERR: {:?}", RESULT_ERROR),
                _ => unreachable!()
            };

            writeln!(out, "    let expected_err = {:?};", expected_err)?;
            writeln!(out, "    let mut actual_err = String::default();")?;
        }

        <io::Result<()>>::Ok(())
    };

    let write_sut = |out: &mut W| {
        if !has_debug {
            writeln!(out)?;
        }

        writeln!(out, "        result! {{")?;
        writeln!(out, "            WHEN   foo({});", case == OkCase)?;

        match on_ok {
            NoOk => {}
            OkExpr =>
                write!(out, "            OK     actual_ok = 21")?,
            OkExprVal =>
                write!(out, "            OK     ok; actual_ok = ok")?,
            OkExprMutVal =>
                write!(out, "            OK     mut ok; actual_ok = ok")?,
            OkBlk =>
                write!(out, "            OK     {{ actual_ok = 21; }}")?,
            OkBlkVal =>
                write!(out, "            OK     ok; {{ actual_ok = ok; }}")?,
            OkBlkMutVal =>
                write!(out, "            OK     mut ok; {{ actual_ok = ok }}")?,
        }

        if (has_error || has_debug) && on_ok.is_expr() {
            writeln!(out, ";")?;
        } else if has_ok {
            writeln!(out)?;
        }

        if has_debug {
            match on_dbg {
                Dbg =>
                    write!(out, "            DEBUG  \"foo failed\"")?,
                DbgFmt =>
                    write!(out, "            DEBUG  \"foo failed - {{}}\", 42")?,
                DbgNoErr =>
                    write!(out, "            _DEBUG \"foo failed\"")?,
                DbgFmtNoErr =>
                    write!(out, "            _DEBUG \"foo failed - {{}}\", 42")?,
                DbgCustomErr =>
                    write!(out, "            DEBUG  err; \"foo failed - {{}}; ERR - {{:?}}\", 42, err")?,
                NoDbg => unreachable!()
            }

            if has_error {
                writeln!(out, ";")?;
            } else {
                writeln!(out)?;
            }
        }

        match on_err {
            NoErr => {}
            ErrExpr =>
                writeln!(out, "            ERR    actual_err = String::from(\"error encountered\")")?,
            ErrExprErr =>
                writeln!(out, "            ERR    err; actual_err = format!(\"ERR: {{:?}}\", err)")?,
        }

        writeln!(out, "        }}")?;

        if has_debug {
            writeln!(out, "    }};")?;
        }

        writeln!(out)?;

        if has_ok {
            writeln!(out, "    assert_eq!(expected_ok, actual_ok);")?;
        }

        if has_error {
            writeln!(out, "    assert_eq!(expected_err, actual_err);")?;
        }

        <io::Result<()>>::Ok(())
    };

    if has_debug {
        let expected_dbg = if case == OkCase {
            String::from("EXPECTED_BLANK")
        } else {
            format!(
                "\"ERROR: foo failed{}{}{}\\n\"",
                match on_dbg {
                    DbgFmt | DbgFmtNoErr | DbgCustomErr => " - 42",
                    _ => "",
                },
                if on_dbg == DbgCustomErr {
                    "; ERR - "
                } else if on_dbg.discards_err() {
                    ""
                } else {
                    "; "
                },
                match on_dbg {
                    Dbg | DbgFmt | DbgCustomErr => format!("\\\"{}\\\"", RESULT_ERROR),
                    _ => String::default()
                }
            )
        };

        writeln!(out, "    expect! {{ EXPECTED_DBG: &str => EXPECTED_BLANK, {} }}", expected_dbg)?;

        write_expected(out)?;

        writeln!(out)?;
        writeln!(out, "    let (actual_out, actual_dbg) = capture! {{")?;

        write_sut(out)?;

        writeln!(out, "    assert_eq!(EXPECTED_DBG, actual_dbg);")?;
        writeln!(out)?;
        writeln!(out, r#"    assert_eq!(
        EXPECTED_BLANK, actual_out,
        "alternate io expected to be blank"
    );"#)?;
    } else {
        write_expected(out)?;
        write_sut(out)?;
    }

    writeln!(out, "}}")?;
    writeln!(out)?;

    Ok(())
}

fn test_name(configuration: Configuration) -> String {
    let (on_ok, on_dbg, on_err, has_ok, has_debug, has_error) = destructure(configuration);
    let mut test_name = String::from("when");

    if has_ok {
        test_name.push_str("_ok_evaluate_");
        test_name.push_str(
            match on_ok {
                OkExpr => "expression",
                OkExprVal => "expression_use_value",
                OkExprMutVal => "expression_use_mutable_value",
                OkBlk => "code_block",
                OkBlkVal => "code_block_use_value",
                OkBlkMutVal => "code_block_use_mutable_value",
                NoOk => unreachable!(),
            }
        );
    }

    if has_debug || has_error {
        if has_ok { test_name.push_str("_or_when"); }

        test_name.push_str("_err");

        if has_debug {
            test_name.push('_');
            test_name.push_str(
                match on_dbg {
                    Dbg => "output_debug_message",
                    DbgFmt => "output_formatted_debug_message",
                    DbgNoErr => "output_debug_message_discard_err",
                    DbgFmtNoErr => "output_formatted_debug_message_discard_err",
                    DbgCustomErr => "output_custom_debug_message",
                    NoDbg => unreachable!(),
                }
            );
        }

        if has_debug && has_error {
            test_name.push_str("_then");
        }

        if on_err != NoErr {
            test_name.push_str("_evaluate_expression");

            if on_err == ErrExpr {
                test_name.push_str("_discard_err");
            }
        }
    }

    test_name
}

fn macro_doc_examples<W: Write>(out: &mut W) -> io::Result<()> {
    for configuration in macro_configurations() {
        let (on_ok, on_dbg, on_err, has_ok, has_debug, has_error) = destructure(configuration);

        comment(out, configuration, "/// * ", "///   ", "\\")?;

        writeln!(out, "///")?;
        writeln!(out, "/// ```rust")?;
        writeln!(out, "/// # use macrofied_toolbox::result;")?;
        writeln!(out, "/// # use cli_toolbox::debug;")?;
        writeln!(out, "/// result! {{")?;
        writeln!(out, "///     WHEN   foo();")?;

        if has_ok {
            match on_ok {
                NoOk => {}
                OkExpr =>
                    write!(out, "///     OK     junk()")?,
                OkExprVal =>
                    write!(out, "///     OK     val; if val == 42 {{ junk(); }}")?,
                OkExprMutVal =>
                    write!(out, "///     OK     mut val; junk(&mut val)")?,
                OkBlk =>
                    writeln!(out, "///     OK     {{ junk() }}")?,
                OkBlkVal =>
                    writeln!(out, "///     OK     val; {{ if val == 42 {{ junk(); }} }}")?,
                OkBlkMutVal =>
                    writeln!(out, "///     OK     mut val; {{ junk(&mut val); }}")?,
            }

            if on_ok.is_expr() {
                if has_debug || has_error {
                    writeln!(out, ";")?;
                } else {
                    writeln!(out)?;
                }
            }
        }

        if has_debug {
            match on_dbg {
                NoDbg => {}
                Dbg =>
                    write!(out, "///     DEBUG  \"foo failed\"")?,
                DbgFmt =>
                    write!(out, "///     DEBUG  \"foo failed: {{}}\", 42")?,
                DbgNoErr =>
                    write!(out, "///     _DEBUG \"foo failed\"")?,
                DbgFmtNoErr =>
                    write!(out, "///     _DEBUG \"foo failed: {{}}\", 42")?,
                DbgCustomErr =>
                    write!(out, "///     DEBUG  err; \"foo failed: {{}}, err: {{:?}}\", 42, err")?
            }

            if has_error {
                writeln!(out, ";")?;
            } else {
                writeln!(out)?;
            }
        }

        if has_error {
            match on_err {
                NoErr => {}
                ErrExpr =>
                    write!(out, "///     ERR    process_error()")?,
                ErrExprErr =>
                    write!(out, "///     ERR    err; process_error(err)")?
            }

            writeln!(out)?;
        }

        writeln!(out, "/// }}")?;
        writeln!(out, "/// # fn foo() -> Result<usize, &'static str> {{ Ok(42) }}")?;

        if has_ok {
            if on_ok.mutable_value() {
                writeln!(out, "/// # fn junk(_val: &mut usize) {{}}")?;
            } else {
                writeln!(out, "/// # fn junk() {{}}")?;
            }
        }

        if has_error {
            if on_err == ErrExpr {
                writeln!(out, "/// # fn process_error() {{}}")?;
            } else {
                writeln!(out, "/// # fn process_error<E>(_err: E) {{}}")?;
            }
        }

        writeln!(out, "/// ```")?;
    }

    Ok(())
}

fn macro_logic<W: Write>(out: &mut W, configuration: Configuration) -> io::Result<()> {
    let (on_ok, on_dbg, on_err, has_ok, has_debug, has_error) = destructure(configuration);

    let match_open = |out: &mut W| writeln!(out, "        match $when {{");
    let match_close = |out: &mut W| writeln!(out, "        }}");

    let debug_cfg = |out: &mut W, indent: &str|
        writeln!(out, "{}            #[cfg(feature = \"debug-result\")]", indent);

    let debug = |out: &mut W, indent: &str| {
        write!(out, "{}            cli_toolbox::debug! {{ ERR ", indent)?;

        if on_dbg.outputs_err() {
            write!(out, "concat!($dbg, \"; {{:?}}\")")?;
        } else {
            write!(out, "$dbg")?;
        }

        if on_dbg.is_formatted() {
            write!(out, ", $($arg),+")?;
        }

        if on_dbg.outputs_err() {
            if on_err == ErrExprErr {
                write!(out, ", $err")?;
            } else {
                write!(out, ", err")?;
            }
        }
        writeln!(out, " }}")
    };

    let when_ok = |out: &mut W, grp: &str| {
        write!(out, "        if ")?;

        if on_ok.discards_ok() {
            write!(out, "$when.is_ok()")?;
        } else {
            write!(out, "let Ok(")?;

            if on_ok.mutable_value() {
                write!(out, "mut ")?;
            } else if on_ok == NoOk {
                panic!("{} ok logic expected - when", grp);
            }

            write!(out, "$ok) = $when")?;
        }

        writeln!(out, " {{ $on_ok }}")
    };

    let when_dbg_err = |out: &mut W, grp: &str| {
        if !has_error {
            writeln!(out, "        #[cfg(debug_assertions)]")?;
        }

        write!(out, "        if ")?;

        if on_err == ErrExpr || on_dbg.discards_err() {
            write!(out, "$when.is_err()")?;
        } else {
            write!(out, "let Err(")?;
            if on_err == ErrExprErr || on_dbg == DbgCustomErr {
                write!(out, "$err")?;
            } else if on_dbg.outputs_err() {
                write!(out, "err")?;
            } else {
                panic!("{} err logic expected - when", grp);
            }

            write!(out, ") = $when")?;
        }

        writeln!(out, " {{")?;

        if has_debug {
            debug_cfg(out, "")?;
            debug(out, "")?;
        }

        if has_debug && on_err == ErrExprErr {
            writeln!(out)?;
        }

        if has_error {
            writeln!(out, "            $on_err")?;
        }

        writeln!(out, "        }}")
    };

    let ok_match = |out: &mut W, grp: &str| {
        write!(out, "            Ok(")?;

        match on_ok {
            OkExpr | OkBlk =>
                write!(out, "_")?,
            OkExprVal | OkBlkVal =>
                write!(out, "$ok")?,
            OkExprMutVal | OkBlkMutVal =>
                write!(out, "mut $ok")?,
            NoOk =>
                panic!("{} ok logic expected - match", grp),
        }

        write!(out, ") => $on_ok")?;

        if on_ok.is_expr() {
            writeln!(out, ",")
        } else {
            writeln!(out)
        }
    };

    let err_match = |out: &mut W, grp: &str| {
        match on_err {
            ErrExpr =>
                writeln!(out, "            Err() => $on_err"),
            ErrExprErr =>
                writeln!(out, "            Err($err) => $on_err"),
            NoErr =>
                panic!("{} err logic expected", grp)
        }
    };

    let err_match_open = |out: &mut W, grp: &str|
        if on_dbg == DbgCustomErr || on_err == ErrExprErr {
            writeln!(out, "            Err($err) => {{")
        } else if on_dbg.outputs_err() {
            writeln!(out, "            Err(err) => {{")
        } else if on_dbg.discards_err() {
            writeln!(out, "            Err(_) => {{")
        } else {
            panic!("{} dgb/err logic expected - err match open", grp);
        };

    let err_match_close = |out: &mut W| writeln!(out, "            }}");

    let err_expr = |out: &mut W| writeln!(out, "                $on_err");

    const OK: bool = true;
    const NO_OK: bool = false;
    const DBG: bool = true;
    const NO_DBG: bool = false;
    const ERR: bool = true;
    const NO_ERR: bool = false;

    match (has_ok, has_debug, has_error) {
        (OK, NO_DBG, NO_ERR) => {
            const SECTION: &str = "K--";

            when_ok(out, SECTION)?;
        }
        (OK, NO_DBG, ERR) => {
            const SECTION: &str = "K-E";

            match_open(out)?;
            ok_match(out, SECTION)?;
            err_match(out, SECTION)?;
            match_close(out)?;
        }
        (OK, DBG, NO_ERR) => {
            const SECTION: &str = "KD-";

            writeln!(out, "        #[cfg(any(not(debug_assertions), not(feature = \"debug-result\")))]")?;
            when_ok(out, SECTION)?;

            writeln!(out)?;
            writeln!(out, "        #[cfg(all(debug_assertions, feature = \"debug-result\"))]")?;
            match_open(out)?;
            ok_match(out, SECTION)?;
            err_match_open(out, SECTION)?;
            debug(out, "        ")?;
            err_match_close(out)?;
            match_close(out)?;
        }
        (OK, DBG, ERR) => {
            const SECTION: &str = "KDE";

            match_open(out)?;
            ok_match(out, SECTION)?;
            err_match_open(out, SECTION)?;
            debug_cfg(out, "    ")?;
            debug(out, "    ")?;
            writeln!(out)?;
            err_expr(out)?;
            err_match_close(out)?;
            match_close(out)?;
        }
        (NO_OK, DBG, NO_ERR) => {
            const SECTION: &str = "-D-";

            when_dbg_err(out, SECTION)?;
        }
        (NO_OK, DBG, ERR) => {
            const SECTION: &str = "-DE";

            when_dbg_err(out, SECTION)?;
        }
        (NO_OK, NO_DBG, ERR) => {
            const SECTION: &str = "--E";

            when_dbg_err(out, SECTION)?;
        }
        _ => panic!("unsupported macro definition")
    }

    Ok(())
}

fn comment<W: Write>(
    out: &mut W, configuration: Configuration, prefix1: &str, prefix2: &str, suffix: &str,
) -> io::Result<()> {
    let (on_ok, on_dbg, on_err, has_ok, has_debug, has_error) = destructure(configuration);

    let ok_section = |out: &mut W| {
        match on_ok {
            NoOk => Ok(()),
            OkExpr =>
                write!(out, " ok, evaluate expression"),
            OkExprVal =>
                write!(out, " ok, evaluate expression with value"),
            OkExprMutVal =>
                write!(out, " ok, evaluate expression with mutable value"),
            OkBlk =>
                write!(out, " ok, evaluate code block"),
            OkBlkVal =>
                write!(out, " ok, evaluate code block with value"),
            OkBlkMutVal =>
                write!(out, " ok, evaluate code block with mutable value"),
        }
    };

    let debug_section = |out: &mut W, context: &str| {
        match on_dbg {
            NoDbg => Ok(()),
            Dbg =>
                write!(out, "{}output debug message", context),
            DbgFmt =>
                write!(out, "{}output formatted debug message", context),
            DbgNoErr =>
                write!(out, "{}output debug message without err", context),
            DbgFmtNoErr =>
                write!(out, "{}output formatted debug message without err", context),
            DbgCustomErr =>
                write!(out, "{}output formatted debug message with custom err", context),
        }
    };

    let error_section = |out: &mut W, context: &str| {
        if has_error {
            write!(out, "{}evaluate expression", context)
        } else {
            Ok(())
        }
    };

    let discards_ok = |out: &mut W| {
        if on_ok.discards_ok() {
            write!(out, "; discard ok")
        } else {
            Ok(())
        }
    };

    let discards_err = |out: &mut W| {
        if on_err.discards_err() && on_dbg != DbgCustomErr {
            write!(out, "; discard err")
        } else {
            Ok(())
        }
    };

    let second_when = |out: &mut W| write!(out, "{}\n{}when", suffix, prefix2);

    const ERROR_CONTEXT: &str = " error, ";
    const NEXT_CONTEXT: &str = " then ";

    const OK: bool = true;
    const NO_OK: bool = false;
    const DBG: bool = true;
    const NO_DBG: bool = false;
    const ERR: bool = true;
    const NO_ERR: bool = false;

    write!(out, "{}when", prefix1)?;

    match (has_ok, has_debug, has_error) {
        (OK, NO_DBG, NO_ERR) => {
            ok_section(out)?;
            discards_ok(out)?;
        }
        (OK, NO_DBG, ERR) => {
            ok_section(out)?;
            discards_ok(out)?;
            second_when(out)?;
            error_section(out, ERROR_CONTEXT)?;
            discards_err(out)?;
        }
        (OK, DBG, NO_ERR) => {
            ok_section(out)?;
            discards_ok(out)?;
            second_when(out)?;
            debug_section(out, ERROR_CONTEXT)?;
        }
        (OK, DBG, ERR) => {
            ok_section(out)?;
            discards_ok(out)?;
            second_when(out)?;
            debug_section(out, ERROR_CONTEXT)?;
            error_section(out, NEXT_CONTEXT)?;
            discards_err(out)?;
        }
        (NO_OK, DBG, NO_ERR) => {
            debug_section(out, ERROR_CONTEXT)?;
        }
        (NO_OK, DBG, ERR) => {
            debug_section(out, ERROR_CONTEXT)?;
            error_section(out, NEXT_CONTEXT)?;
            discards_err(out)?;
        }
        (NO_OK, NO_DBG, ERR) => {
            error_section(out, ERROR_CONTEXT)?;
            discards_err(out)?;
        }
        _ => panic!("unsupported macro definition")
    }

    writeln!(out)
}

fn macro_configurations() -> [Configuration; 96] {
    [
        (OkExpr, NoDbg, NoErr, ),
        (OkBlkVal, NoDbg, NoErr, ),
        (OkExprVal, NoDbg, NoErr, ),
        (OkBlkMutVal, NoDbg, NoErr, ),
        (OkExprMutVal, NoDbg, NoErr, ),
        (NoOk, Dbg, NoErr, ),
        (NoOk, DbgFmt, NoErr, ),
        (NoOk, DbgNoErr, NoErr, ),
        (NoOk, DbgFmtNoErr, NoErr, ),
        (NoOk, DbgCustomErr, NoErr, ),
        (NoOk, NoDbg, ErrExpr, ),
        (NoOk, NoDbg, ErrExprErr, ),
        (OkBlk, Dbg, NoErr, ),
        (OkBlk, DbgFmt, NoErr, ),
        (OkBlk, DbgNoErr, NoErr, ),
        (OkBlk, DbgFmtNoErr, NoErr, ),
        (OkBlk, DbgCustomErr, NoErr, ),
        (OkExpr, Dbg, NoErr, ),
        (OkExpr, DbgFmt, NoErr, ),
        (OkExpr, DbgNoErr, NoErr, ),
        (OkExpr, DbgFmtNoErr, NoErr, ),
        (OkExpr, DbgCustomErr, NoErr, ),
        (OkBlkVal, Dbg, NoErr, ),
        (OkBlkVal, DbgFmt, NoErr, ),
        (OkBlkVal, DbgNoErr, NoErr, ),
        (OkBlkVal, DbgFmtNoErr, NoErr, ),
        (OkBlkVal, DbgCustomErr, NoErr, ),
        (OkExprVal, Dbg, NoErr, ),
        (OkExprVal, DbgFmt, NoErr, ),
        (OkExprVal, DbgNoErr, NoErr, ),
        (OkExprVal, DbgFmtNoErr, NoErr, ),
        (OkExprVal, DbgCustomErr, NoErr, ),
        (OkBlkMutVal, Dbg, NoErr, ),
        (OkBlkMutVal, DbgFmt, NoErr, ),
        (OkBlkMutVal, DbgNoErr, NoErr, ),
        (OkBlkMutVal, DbgFmtNoErr, NoErr, ),
        (OkBlkMutVal, DbgCustomErr, NoErr, ),
        (OkExprMutVal, Dbg, NoErr, ),
        (OkExprMutVal, DbgFmt, NoErr, ),
        (OkExprMutVal, DbgNoErr, NoErr, ),
        (OkExprMutVal, DbgFmtNoErr, NoErr, ),
        (OkExprMutVal, DbgCustomErr, NoErr, ),
        (OkBlk, Dbg, ErrExpr, ),
        (OkBlk, Dbg, ErrExprErr, ),
        (OkBlk, DbgFmt, ErrExpr, ),
        (OkBlk, DbgFmt, ErrExprErr, ),
        (OkBlk, DbgNoErr, ErrExpr, ),
        (OkBlk, DbgNoErr, ErrExprErr, ),
        (OkBlk, DbgFmtNoErr, ErrExpr, ),
        (OkBlk, DbgFmtNoErr, ErrExprErr, ),
        (OkBlk, DbgCustomErr, ErrExpr, ),
        (OkExpr, Dbg, ErrExpr, ),
        (OkExpr, Dbg, ErrExprErr, ),
        (OkExpr, DbgFmt, ErrExpr, ),
        (OkExpr, DbgFmt, ErrExprErr, ),
        (OkExpr, DbgNoErr, ErrExpr, ),
        (OkExpr, DbgNoErr, ErrExprErr, ),
        (OkExpr, DbgFmtNoErr, ErrExpr, ),
        (OkExpr, DbgFmtNoErr, ErrExprErr, ),
        (OkExpr, DbgCustomErr, ErrExpr, ),
        (OkBlkVal, Dbg, ErrExpr, ),
        (OkBlkVal, Dbg, ErrExprErr, ),
        (OkBlkVal, DbgFmt, ErrExpr, ),
        (OkBlkVal, DbgFmt, ErrExprErr, ),
        (OkBlkVal, DbgNoErr, ErrExpr, ),
        (OkBlkVal, DbgNoErr, ErrExprErr, ),
        (OkBlkVal, DbgFmtNoErr, ErrExpr, ),
        (OkBlkVal, DbgFmtNoErr, ErrExprErr, ),
        (OkBlkVal, DbgCustomErr, ErrExpr, ),
        (OkExprVal, Dbg, ErrExpr, ),
        (OkExprVal, Dbg, ErrExprErr, ),
        (OkExprVal, DbgFmt, ErrExpr, ),
        (OkExprVal, DbgFmt, ErrExprErr, ),
        (OkExprVal, DbgNoErr, ErrExpr, ),
        (OkExprVal, DbgNoErr, ErrExprErr, ),
        (OkExprVal, DbgFmtNoErr, ErrExpr, ),
        (OkExprVal, DbgFmtNoErr, ErrExprErr, ),
        (OkExprVal, DbgCustomErr, ErrExpr, ),
        (OkBlkMutVal, Dbg, ErrExpr, ),
        (OkBlkMutVal, Dbg, ErrExprErr, ),
        (OkBlkMutVal, DbgFmt, ErrExpr, ),
        (OkBlkMutVal, DbgFmt, ErrExprErr, ),
        (OkBlkMutVal, DbgNoErr, ErrExpr, ),
        (OkBlkMutVal, DbgNoErr, ErrExprErr, ),
        (OkBlkMutVal, DbgFmtNoErr, ErrExpr, ),
        (OkBlkMutVal, DbgFmtNoErr, ErrExprErr, ),
        (OkBlkMutVal, DbgCustomErr, ErrExpr, ),
        (OkExprMutVal, Dbg, ErrExpr, ),
        (OkExprMutVal, Dbg, ErrExprErr, ),
        (OkExprMutVal, DbgFmt, ErrExpr, ),
        (OkExprMutVal, DbgFmt, ErrExprErr, ),
        (OkExprMutVal, DbgNoErr, ErrExpr, ),
        (OkExprMutVal, DbgNoErr, ErrExprErr, ),
        (OkExprMutVal, DbgFmtNoErr, ErrExpr, ),
        (OkExprMutVal, DbgFmtNoErr, ErrExprErr, ),
        (OkExprMutVal, DbgCustomErr, ErrExpr, ),
    ]
}

fn destructure(configuration: Configuration) -> (OnOk, OnErrDebug, OnErr, bool, bool, bool) {
    let (on_ok, on_dbg, on_err) = configuration;

    (
        on_ok,
        on_dbg,
        on_err,
        on_ok != NoOk,
        on_dbg != NoDbg,
        on_err != NoErr,
    )
}
