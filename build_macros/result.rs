use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

type Configuration = (bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool);

pub fn generate_result_macro() -> io::Result<()> {
    let result_source_file = File::create("src/result.rs")?;
    let mut out = BufWriter::new(result_source_file);

    writeln!(out, "///")?;
    writeln!(out, "#[macro_export]")?;
    writeln!(out, "macro_rules! result {{")?;

    for configuration in macro_configurations() {
        let (
            ok_blk, ok, ok_val_blk, ok_val, ok_mut_val_blk, ok_mut_val,
            dbg, dbg_args, _dbg, _dbg_args, dbg_err_args,
            err, err_err
        ) = configuration;
        let ok_flags = [ok_blk, ok, ok_val_blk, ok_val, ok_mut_val_blk, ok_mut_val];

        if more_than_one(ok_flags) {
            panic!("multiple oks")
        }

        let dbg_flags = [dbg, dbg_args, _dbg, _dbg_args, dbg_err_args];

        if more_than_one(dbg_flags) {
            panic!("multiple debugs")
        }

        let err_flags = [err, err_err];

        if more_than_one(err_flags) {
            panic!("multiple errors")
        }

        if more_than_one([dbg_err_args, err_err]) {
            panic!("multiple err identifiers")
        }

        let has_ok = any(ok_flags);
        let has_debug = any(dbg_flags);
        let has_error = any(err_flags);

        writeln!(out, "    (")?;
        writeln!(out, "        WHEN   $when:expr;")?;

        if ok_blk {
            writeln!(out, "        OK     $on_ok:block")?;
        } else if ok_val_blk {
            writeln!(out, "        OK     $ok:ident; $on_ok:block")?;
        } else if ok_mut_val_blk {
            writeln!(out, "        OK     mut $ok:ident; $on_ok:block")?;
        } else if ok {
            write!(out, "        OK     $on_ok:expr")?;
        } else if ok_val {
            write!(out, "        OK     $ok:ident; $on_ok:expr")?;
        } else if ok_mut_val {
            write!(out, "        OK     mut $ok:ident; $on_ok:expr")?;
        }

        if ok || ok_val || ok_mut_val {
            if has_debug || has_error {
                writeln!(out, ";")?;
            } else {
                writeln!(out, )?;
            }
        }

        if dbg {
            write!(out, "        DEBUG  $dbg:expr")?;
        } else if dbg_args {
            write!(out, "        DEBUG  $dbg:expr, $($arg:expr),+")?;
        } else if _dbg {
            write!(out, "        _DEBUG $dbg:expr")?;
        } else if _dbg_args {
            write!(out, "        _DEBUG $dbg:expr, $($arg:expr),+")?;
        } else if dbg_err_args {
            write!(out, "        DEBUG  $err:ident; $dbg:expr, $($arg:expr),+")?;
        }

        if has_debug {
            if has_error {
                writeln!(out, ";")?;
            } else {
                writeln!(out, )?;
            }
        }

        if err {
            writeln!(out, "        ERR    $on_err:expr")?;
        } else if err_err {
            writeln!(out, "        ERR    $err:ident; $on_err:expr")?;
        }

        writeln!(out, "    ) => {{")?;

        macro_logic(&mut out, configuration, has_ok, has_debug, has_error)?;

        writeln!(out, "    }};")?;
    }

    writeln!(out, "}}")
}

fn macro_logic<W: Write>(
    out: &mut W, configuration: Configuration, has_ok: bool, has_debug: bool, has_error: bool,
) -> io::Result<()> {
    let (
        ok_blk, ok, ok_val_blk, ok_val, ok_mut_val_blk, ok_mut_val,
        dbg, dbg_args, _dbg, _dbg_args, dbg_err_args,
        err, err_err
    ) = configuration;

    let ok = any([ok, ok_blk]);
    let ok_val = any([ok_val, ok_val_blk]);
    let ok_mut_val = any([ok_mut_val, ok_mut_val_blk]);
    let ok_blk = any([ok_blk, ok_val_blk, ok_mut_val_blk]);

    let match_open = |out: &mut W| writeln!(out, "        match $when {{");
    let match_close = |out: &mut W| writeln!(out, "        }}");

    let debug_cfg = |out: &mut W, indent: &str|
        writeln!(out, "{}            #[cfg(feature = \"debug-result\")]", indent);

    let debug = |out: &mut W, indent: &str| {
        write!(out, "{}            cli_toolbox::debug! {{ ERR ", indent)?;

        if dbg || dbg_args {
            write!(out, "concat!($dbg, \": {{:?}}\")")?;
        } else {
            write!(out, "$dbg")?;
        }

        if dbg_args || _dbg_args || dbg_err_args {
            write!(out, ", $($arg),+")?;
        }

        if dbg || dbg_args {
            if err_err {
                write!(out, ", $err")?;
            } else {
                write!(out, ", err")?;
            }
        }
        writeln!(out, " }}")
    };

    let when_ok = |out: &mut W, grp: &str| {
        write!(out, "        if ")?;

        if ok {
            write!(out, "$when.is_ok()")?;
        } else {
            write!(out, "let Ok(")?;

            if ok_mut_val {
                write!(out, "mut ")?;
            } else if !ok_val {
                panic!("{} ok logic expected - when", grp);
            }

            write!(out, "$ok) = $when")?;
        }

        writeln!(out, " {{ $on_ok }}")
    };

    let when_dbg_err = |out: &mut W, grp: &str| {
        write!(out, "        if ")?;

        if err || _dbg || _dbg_args {
            write!(out, "$when.is_err()")?;
        } else {
            write!(out, "let Err(")?;
            if err_err || dbg_err_args {
                write!(out, "$err")?;
            } else if dbg || dbg_args {
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

        if has_debug && err_err {
            writeln!(out, )?;
        }

        if err_err {
            writeln!(out, "            $on_err")?;
        }

        writeln!(out, "        }}")
    };

    let ok_match = |out: &mut W, grp: &str| {
        write!(out, "            Ok(")?;

        if ok {
            write!(out, "_")?;
        } else if ok_val {
            write!(out, "$ok")?;
        } else if ok_mut_val {
            write!(out, "mut $ok")?;
        } else {
            panic!("{} ok logic expected - match", grp);
        }

        write!(out, ") => $on_ok")?;

        if ok_blk {
            writeln!(out, )
        } else {
            writeln!(out, ",")
        }
    };

    let err_match_open = |out: &mut W, grp: &str|
        if dbg_err_args || err_err {
            writeln!(out, "            Err($err) => {{")
        } else if dbg || dbg_args {
            writeln!(out, "            Err(err) => {{")
        } else if _dbg || _dbg_args {
            writeln!(out, "            Err(_) => {{")
        } else {
            panic!("{} dgb/err logic expected - err match open", grp);
        };

    let err_match_close = |out: &mut W| writeln!(out, "            }}");

    let err_expr = |out: &mut W| writeln!(out, "                $on_err");

    match (has_ok, has_debug, has_error) {
        (true, false, false) => {
            const SECTION: &str = "K--";

            when_ok(out, SECTION)?;
        }
        (true, false, true) => {
            const SECTION: &str = "K-E";

            match_open(out)?;

            ok_match(out, SECTION)?;

            if err {
                writeln!(out, "            Err() => $on_err")?;
            } else if err_err {
                writeln!(out, "            Err($err) => $on_err")?;
            } else {
                panic!("{} err logic expected", SECTION);
            }

            match_close(out)?;
        }
        (true, true, false) => {
            const SECTION: &str = "KD-";

            writeln!(out, "        #[cfg(any(not(debug_assertions), not(feature = \"debug-result\")))]")?;
            when_ok(out, SECTION)?;

            writeln!(out, )?;
            writeln!(out, "        #[cfg(all(debug_assertions, feature = \"debug-result\"))]")?;
            match_open(out)?;
            ok_match(out, SECTION)?;
            err_match_open(out, SECTION)?;
            debug(out, "        ")?;
            err_match_close(out)?;
            match_close(out)?;
        }
        (true, true, true) => {
            const SECTION: &str = "KDE";

            match_open(out)?;
            ok_match(out, SECTION)?;
            err_match_open(out, SECTION)?;
            debug_cfg(out, "    ")?;
            debug(out, "    ")?;
            writeln!(out, )?;
            err_expr(out)?;
            err_match_close(out)?;
            match_close(out)?;
        }
        (false, true, false) => {
            const SECTION: &str = "-D-";

            when_dbg_err(out, SECTION)?;
        }
        (false, true, true) => {
            const SECTION: &str = "-DE";

            when_dbg_err(out, SECTION)?;
        }
        (false, false, true) => {
            const SECTION: &str = "--E";

            when_dbg_err(out, SECTION)?;
        }
        _ => panic!("unsupported macro definition")
    }

    Ok(())
}

fn more_than_one<const BITS: usize>(bits: [bool; BITS]) -> bool {
    check(bits) > 1
}

fn any<const BITS: usize>(bits: [bool; BITS]) -> bool {
    check(bits) > 0
}

fn check<const BITS: usize>(bits: [bool; BITS]) -> usize {
    let mut check = 0;

    for bit in bits {
        if bit {
            check <<= 1;
            check += 1;
        }
    }

    check
}

fn macro_configurations() -> [Configuration; 96] {
    [
        (false, true, false, false, false, false, false, false, false, false, false, false, false),
        (false, false, true, false, false, false, false, false, false, false, false, false, false),
        (false, false, false, true, false, false, false, false, false, false, false, false, false),
        (false, false, false, false, true, false, false, false, false, false, false, false, false),
        (false, false, false, false, false, true, false, false, false, false, false, false, false),
        (false, false, false, false, false, false, true, false, false, false, false, false, false),
        (false, false, false, false, false, false, false, true, false, false, false, false, false),
        (false, false, false, false, false, false, false, false, true, false, false, false, false),
        (false, false, false, false, false, false, false, false, false, true, false, false, false),
        (false, false, false, false, false, false, false, false, false, false, true, false, false),
        (false, false, false, false, false, false, false, false, false, false, false, true, false),
        (false, false, false, false, false, false, false, false, false, false, false, false, true),
        (true, false, false, false, false, false, true, false, false, false, false, false, false),
        (true, false, false, false, false, false, false, true, false, false, false, false, false),
        (true, false, false, false, false, false, false, false, true, false, false, false, false),
        (true, false, false, false, false, false, false, false, false, true, false, false, false),
        (true, false, false, false, false, false, false, false, false, false, true, false, false),
        (false, true, false, false, false, false, true, false, false, false, false, false, false),
        (false, true, false, false, false, false, false, true, false, false, false, false, false),
        (false, true, false, false, false, false, false, false, true, false, false, false, false),
        (false, true, false, false, false, false, false, false, false, true, false, false, false),
        (false, true, false, false, false, false, false, false, false, false, true, false, false),
        (false, false, true, false, false, false, true, false, false, false, false, false, false),
        (false, false, true, false, false, false, false, true, false, false, false, false, false),
        (false, false, true, false, false, false, false, false, true, false, false, false, false),
        (false, false, true, false, false, false, false, false, false, true, false, false, false),
        (false, false, true, false, false, false, false, false, false, false, true, false, false),
        (false, false, false, true, false, false, true, false, false, false, false, false, false),
        (false, false, false, true, false, false, false, true, false, false, false, false, false),
        (false, false, false, true, false, false, false, false, true, false, false, false, false),
        (false, false, false, true, false, false, false, false, false, true, false, false, false),
        (false, false, false, true, false, false, false, false, false, false, true, false, false),
        (false, false, false, false, true, false, true, false, false, false, false, false, false),
        (false, false, false, false, true, false, false, true, false, false, false, false, false),
        (false, false, false, false, true, false, false, false, true, false, false, false, false),
        (false, false, false, false, true, false, false, false, false, true, false, false, false),
        (false, false, false, false, true, false, false, false, false, false, true, false, false),
        (false, false, false, false, false, true, true, false, false, false, false, false, false),
        (false, false, false, false, false, true, false, true, false, false, false, false, false),
        (false, false, false, false, false, true, false, false, true, false, false, false, false),
        (false, false, false, false, false, true, false, false, false, true, false, false, false),
        (false, false, false, false, false, true, false, false, false, false, true, false, false),
        (true, false, false, false, false, false, true, false, false, false, false, true, false),
        (true, false, false, false, false, false, true, false, false, false, false, false, true),
        (true, false, false, false, false, false, false, true, false, false, false, true, false),
        (true, false, false, false, false, false, false, true, false, false, false, false, true),
        (true, false, false, false, false, false, false, false, true, false, false, true, false),
        (true, false, false, false, false, false, false, false, true, false, false, false, true),
        (true, false, false, false, false, false, false, false, false, true, false, true, false),
        (true, false, false, false, false, false, false, false, false, true, false, false, true),
        (true, false, false, false, false, false, false, false, false, false, true, true, false),
        (false, true, false, false, false, false, true, false, false, false, false, true, false),
        (false, true, false, false, false, false, true, false, false, false, false, false, true),
        (false, true, false, false, false, false, false, true, false, false, false, true, false),
        (false, true, false, false, false, false, false, true, false, false, false, false, true),
        (false, true, false, false, false, false, false, false, true, false, false, true, false),
        (false, true, false, false, false, false, false, false, true, false, false, false, true),
        (false, true, false, false, false, false, false, false, false, true, false, true, false),
        (false, true, false, false, false, false, false, false, false, true, false, false, true),
        (false, true, false, false, false, false, false, false, false, false, true, true, false),
        (false, false, true, false, false, false, true, false, false, false, false, true, false),
        (false, false, true, false, false, false, true, false, false, false, false, false, true),
        (false, false, true, false, false, false, false, true, false, false, false, true, false),
        (false, false, true, false, false, false, false, true, false, false, false, false, true),
        (false, false, true, false, false, false, false, false, true, false, false, true, false),
        (false, false, true, false, false, false, false, false, true, false, false, false, true),
        (false, false, true, false, false, false, false, false, false, true, false, true, false),
        (false, false, true, false, false, false, false, false, false, true, false, false, true),
        (false, false, true, false, false, false, false, false, false, false, true, true, false),
        (false, false, false, true, false, false, true, false, false, false, false, true, false),
        (false, false, false, true, false, false, true, false, false, false, false, false, true),
        (false, false, false, true, false, false, false, true, false, false, false, true, false),
        (false, false, false, true, false, false, false, true, false, false, false, false, true),
        (false, false, false, true, false, false, false, false, true, false, false, true, false),
        (false, false, false, true, false, false, false, false, true, false, false, false, true),
        (false, false, false, true, false, false, false, false, false, true, false, true, false),
        (false, false, false, true, false, false, false, false, false, true, false, false, true),
        (false, false, false, true, false, false, false, false, false, false, true, true, false),
        (false, false, false, false, true, false, true, false, false, false, false, true, false),
        (false, false, false, false, true, false, true, false, false, false, false, false, true),
        (false, false, false, false, true, false, false, true, false, false, false, true, false),
        (false, false, false, false, true, false, false, true, false, false, false, false, true),
        (false, false, false, false, true, false, false, false, true, false, false, true, false),
        (false, false, false, false, true, false, false, false, true, false, false, false, true),
        (false, false, false, false, true, false, false, false, false, true, false, true, false),
        (false, false, false, false, true, false, false, false, false, true, false, false, true),
        (false, false, false, false, true, false, false, false, false, false, true, true, false),
        (false, false, false, false, false, true, true, false, false, false, false, true, false),
        (false, false, false, false, false, true, true, false, false, false, false, false, true),
        (false, false, false, false, false, true, false, true, false, false, false, true, false),
        (false, false, false, false, false, true, false, true, false, false, false, false, true),
        (false, false, false, false, false, true, false, false, true, false, false, true, false),
        (false, false, false, false, false, true, false, false, true, false, false, false, true),
        (false, false, false, false, false, true, false, false, false, true, false, true, false),
        (false, false, false, false, false, true, false, false, false, true, false, false, true),
        (false, false, false, false, false, true, false, false, false, false, true, true, false),
    ]
}
