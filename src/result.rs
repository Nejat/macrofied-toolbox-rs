///
#[macro_export]
macro_rules! result {
    // when ok, evaluate expression; discard ok
    (
        WHEN   $when:expr;
        OK     $on_ok:expr
    ) => {
        if $when.is_ok() { $on_ok }
    };
    // when ok, evaluate code block with value
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:block
    ) => {
        if let Ok($ok) = $when { $on_ok }
    };
    // when ok, evaluate expression with value
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:expr
    ) => {
        if let Ok($ok) = $when { $on_ok }
    };
    // when ok, evaluate code block with mutable value
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:block
    ) => {
        if let Ok(mut $ok) = $when { $on_ok }
    };
    // when ok, evaluate expression with mutable value
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:expr
    ) => {
        if let Ok(mut $ok) = $when { $on_ok }
    };
    // when error, output debug message
    (
        WHEN   $when:expr;
        DEBUG  $dbg:expr
    ) => {
        if let Err(err) = $when {
            #[cfg(feature = "debug-result")]
            cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), err }
        }
    };
    // when error, output formatted debug message
    (
        WHEN   $when:expr;
        DEBUG  $dbg:expr, $($arg:expr),+
    ) => {
        if let Err(err) = $when {
            #[cfg(feature = "debug-result")]
            cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $($arg),+, err }
        }
    };
    // when error, output debug message without err
    (
        WHEN   $when:expr;
        _DEBUG $dbg:expr
    ) => {
        if $when.is_err() {
            #[cfg(feature = "debug-result")]
            cli_toolbox::debug! { ERR $dbg }
        }
    };
    // when error, output formatted debug message without err
    (
        WHEN   $when:expr;
        _DEBUG $dbg:expr, $($arg:expr),+
    ) => {
        if $when.is_err() {
            #[cfg(feature = "debug-result")]
            cli_toolbox::debug! { ERR $dbg, $($arg),+ }
        }
    };
    // when error, output formatted debug message with custom err
    (
        WHEN   $when:expr;
        DEBUG  $err:ident; $dbg:expr, $($arg:expr),+
    ) => {
        if let Err($err) = $when {
            #[cfg(feature = "debug-result")]
            cli_toolbox::debug! { ERR $dbg, $($arg),+ }
        }
    };
    // when error, evaluate expression; discard err
    (
        WHEN   $when:expr;
        ERR    $on_err:expr
    ) => {
        if $when.is_err() {
        }
    };
    // when error, evaluate expression
    (
        WHEN   $when:expr;
        ERR    $err:ident; $on_err:expr
    ) => {
        if let Err($err) = $when {
            $on_err
        }
    };
    // when ok, evaluate code block; discard ok
    // when error, output debug message
    (
        WHEN   $when:expr;
        OK     $on_ok:block
        DEBUG  $dbg:expr
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if $when.is_ok() { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok(_) => $on_ok
            Err(err) => {
                    cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), err }
            }
        }
    };
    // when ok, evaluate code block; discard ok
    // when error, output formatted debug message
    (
        WHEN   $when:expr;
        OK     $on_ok:block
        DEBUG  $dbg:expr, $($arg:expr),+
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if $when.is_ok() { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok(_) => $on_ok
            Err(err) => {
                    cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $($arg),+, err }
            }
        }
    };
    // when ok, evaluate code block; discard ok
    // when error, output debug message without err
    (
        WHEN   $when:expr;
        OK     $on_ok:block
        _DEBUG $dbg:expr
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if $when.is_ok() { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok(_) => $on_ok
            Err(_) => {
                    cli_toolbox::debug! { ERR $dbg }
            }
        }
    };
    // when ok, evaluate code block; discard ok
    // when error, output formatted debug message without err
    (
        WHEN   $when:expr;
        OK     $on_ok:block
        _DEBUG $dbg:expr, $($arg:expr),+
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if $when.is_ok() { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok(_) => $on_ok
            Err(_) => {
                    cli_toolbox::debug! { ERR $dbg, $($arg),+ }
            }
        }
    };
    // when ok, evaluate code block; discard ok
    // when error, output formatted debug message with custom err
    (
        WHEN   $when:expr;
        OK     $on_ok:block
        DEBUG  $err:ident; $dbg:expr, $($arg:expr),+
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if $when.is_ok() { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok(_) => $on_ok
            Err($err) => {
                    cli_toolbox::debug! { ERR $dbg, $($arg),+ }
            }
        }
    };
    // when ok, evaluate expression; discard ok
    // when error, output debug message
    (
        WHEN   $when:expr;
        OK     $on_ok:expr;
        DEBUG  $dbg:expr
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if $when.is_ok() { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok(_) => $on_ok,
            Err(err) => {
                    cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), err }
            }
        }
    };
    // when ok, evaluate expression; discard ok
    // when error, output formatted debug message
    (
        WHEN   $when:expr;
        OK     $on_ok:expr;
        DEBUG  $dbg:expr, $($arg:expr),+
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if $when.is_ok() { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok(_) => $on_ok,
            Err(err) => {
                    cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $($arg),+, err }
            }
        }
    };
    // when ok, evaluate expression; discard ok
    // when error, output debug message without err
    (
        WHEN   $when:expr;
        OK     $on_ok:expr;
        _DEBUG $dbg:expr
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if $when.is_ok() { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok(_) => $on_ok,
            Err(_) => {
                    cli_toolbox::debug! { ERR $dbg }
            }
        }
    };
    // when ok, evaluate expression; discard ok
    // when error, output formatted debug message without err
    (
        WHEN   $when:expr;
        OK     $on_ok:expr;
        _DEBUG $dbg:expr, $($arg:expr),+
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if $when.is_ok() { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok(_) => $on_ok,
            Err(_) => {
                    cli_toolbox::debug! { ERR $dbg, $($arg),+ }
            }
        }
    };
    // when ok, evaluate expression; discard ok
    // when error, output formatted debug message with custom err
    (
        WHEN   $when:expr;
        OK     $on_ok:expr;
        DEBUG  $err:ident; $dbg:expr, $($arg:expr),+
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if $when.is_ok() { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok(_) => $on_ok,
            Err($err) => {
                    cli_toolbox::debug! { ERR $dbg, $($arg),+ }
            }
        }
    };
    // when ok, evaluate code block with value
    // when error, output debug message
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:block
        DEBUG  $dbg:expr
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if let Ok($ok) = $when { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok($ok) => $on_ok
            Err(err) => {
                    cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), err }
            }
        }
    };
    // when ok, evaluate code block with value
    // when error, output formatted debug message
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:block
        DEBUG  $dbg:expr, $($arg:expr),+
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if let Ok($ok) = $when { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok($ok) => $on_ok
            Err(err) => {
                    cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $($arg),+, err }
            }
        }
    };
    // when ok, evaluate code block with value
    // when error, output debug message without err
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:block
        _DEBUG $dbg:expr
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if let Ok($ok) = $when { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok($ok) => $on_ok
            Err(_) => {
                    cli_toolbox::debug! { ERR $dbg }
            }
        }
    };
    // when ok, evaluate code block with value
    // when error, output formatted debug message without err
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:block
        _DEBUG $dbg:expr, $($arg:expr),+
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if let Ok($ok) = $when { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok($ok) => $on_ok
            Err(_) => {
                    cli_toolbox::debug! { ERR $dbg, $($arg),+ }
            }
        }
    };
    // when ok, evaluate code block with value
    // when error, output formatted debug message with custom err
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:block
        DEBUG  $err:ident; $dbg:expr, $($arg:expr),+
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if let Ok($ok) = $when { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok($ok) => $on_ok
            Err($err) => {
                    cli_toolbox::debug! { ERR $dbg, $($arg),+ }
            }
        }
    };
    // when ok, evaluate expression with value
    // when error, output debug message
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:expr;
        DEBUG  $dbg:expr
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if let Ok($ok) = $when { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok($ok) => $on_ok,
            Err(err) => {
                    cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), err }
            }
        }
    };
    // when ok, evaluate expression with value
    // when error, output formatted debug message
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:expr;
        DEBUG  $dbg:expr, $($arg:expr),+
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if let Ok($ok) = $when { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok($ok) => $on_ok,
            Err(err) => {
                    cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $($arg),+, err }
            }
        }
    };
    // when ok, evaluate expression with value
    // when error, output debug message without err
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:expr;
        _DEBUG $dbg:expr
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if let Ok($ok) = $when { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok($ok) => $on_ok,
            Err(_) => {
                    cli_toolbox::debug! { ERR $dbg }
            }
        }
    };
    // when ok, evaluate expression with value
    // when error, output formatted debug message without err
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:expr;
        _DEBUG $dbg:expr, $($arg:expr),+
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if let Ok($ok) = $when { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok($ok) => $on_ok,
            Err(_) => {
                    cli_toolbox::debug! { ERR $dbg, $($arg),+ }
            }
        }
    };
    // when ok, evaluate expression with value
    // when error, output formatted debug message with custom err
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:expr;
        DEBUG  $err:ident; $dbg:expr, $($arg:expr),+
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if let Ok($ok) = $when { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok($ok) => $on_ok,
            Err($err) => {
                    cli_toolbox::debug! { ERR $dbg, $($arg),+ }
            }
        }
    };
    // when ok, evaluate code block with mutable value
    // when error, output debug message
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:block
        DEBUG  $dbg:expr
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if let Ok(mut $ok) = $when { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok(mut $ok) => $on_ok
            Err(err) => {
                    cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), err }
            }
        }
    };
    // when ok, evaluate code block with mutable value
    // when error, output formatted debug message
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:block
        DEBUG  $dbg:expr, $($arg:expr),+
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if let Ok(mut $ok) = $when { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok(mut $ok) => $on_ok
            Err(err) => {
                    cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $($arg),+, err }
            }
        }
    };
    // when ok, evaluate code block with mutable value
    // when error, output debug message without err
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:block
        _DEBUG $dbg:expr
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if let Ok(mut $ok) = $when { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok(mut $ok) => $on_ok
            Err(_) => {
                    cli_toolbox::debug! { ERR $dbg }
            }
        }
    };
    // when ok, evaluate code block with mutable value
    // when error, output formatted debug message without err
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:block
        _DEBUG $dbg:expr, $($arg:expr),+
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if let Ok(mut $ok) = $when { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok(mut $ok) => $on_ok
            Err(_) => {
                    cli_toolbox::debug! { ERR $dbg, $($arg),+ }
            }
        }
    };
    // when ok, evaluate code block with mutable value
    // when error, output formatted debug message with custom err
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:block
        DEBUG  $err:ident; $dbg:expr, $($arg:expr),+
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if let Ok(mut $ok) = $when { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok(mut $ok) => $on_ok
            Err($err) => {
                    cli_toolbox::debug! { ERR $dbg, $($arg),+ }
            }
        }
    };
    // when ok, evaluate expression with mutable value
    // when error, output debug message
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:expr;
        DEBUG  $dbg:expr
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if let Ok(mut $ok) = $when { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok(mut $ok) => $on_ok,
            Err(err) => {
                    cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), err }
            }
        }
    };
    // when ok, evaluate expression with mutable value
    // when error, output formatted debug message
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:expr;
        DEBUG  $dbg:expr, $($arg:expr),+
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if let Ok(mut $ok) = $when { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok(mut $ok) => $on_ok,
            Err(err) => {
                    cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $($arg),+, err }
            }
        }
    };
    // when ok, evaluate expression with mutable value
    // when error, output debug message without err
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:expr;
        _DEBUG $dbg:expr
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if let Ok(mut $ok) = $when { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok(mut $ok) => $on_ok,
            Err(_) => {
                    cli_toolbox::debug! { ERR $dbg }
            }
        }
    };
    // when ok, evaluate expression with mutable value
    // when error, output formatted debug message without err
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:expr;
        _DEBUG $dbg:expr, $($arg:expr),+
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if let Ok(mut $ok) = $when { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok(mut $ok) => $on_ok,
            Err(_) => {
                    cli_toolbox::debug! { ERR $dbg, $($arg),+ }
            }
        }
    };
    // when ok, evaluate expression with mutable value
    // when error, output formatted debug message with custom err
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:expr;
        DEBUG  $err:ident; $dbg:expr, $($arg:expr),+
    ) => {
        #[cfg(any(not(debug_assertions), not(feature = "debug-result")))]
        if let Ok(mut $ok) = $when { $on_ok }

        #[cfg(all(debug_assertions, feature = "debug-result"))]
        match $when {
            Ok(mut $ok) => $on_ok,
            Err($err) => {
                    cli_toolbox::debug! { ERR $dbg, $($arg),+ }
            }
        }
    };
    // when ok, evaluate code block; discard ok
    // when error, output debug message then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     $on_ok:block
        DEBUG  $dbg:expr;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok(_) => $on_ok
            Err(err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), err }

                $on_err
            }
        }
    };
    // when ok, evaluate code block; discard ok
    // when error, output debug message then evaluate expression
    (
        WHEN   $when:expr;
        OK     $on_ok:block
        DEBUG  $dbg:expr;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok(_) => $on_ok
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $err }

                $on_err
            }
        }
    };
    // when ok, evaluate code block; discard ok
    // when error, output formatted debug message then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     $on_ok:block
        DEBUG  $dbg:expr, $($arg:expr),+;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok(_) => $on_ok
            Err(err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $($arg),+, err }

                $on_err
            }
        }
    };
    // when ok, evaluate code block; discard ok
    // when error, output formatted debug message then evaluate expression
    (
        WHEN   $when:expr;
        OK     $on_ok:block
        DEBUG  $dbg:expr, $($arg:expr),+;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok(_) => $on_ok
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $($arg),+, $err }

                $on_err
            }
        }
    };
    // when ok, evaluate code block; discard ok
    // when error, output debug message without err then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     $on_ok:block
        _DEBUG $dbg:expr;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok(_) => $on_ok
            Err(_) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg }

                $on_err
            }
        }
    };
    // when ok, evaluate code block; discard ok
    // when error, output debug message without err then evaluate expression
    (
        WHEN   $when:expr;
        OK     $on_ok:block
        _DEBUG $dbg:expr;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok(_) => $on_ok
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg }

                $on_err
            }
        }
    };
    // when ok, evaluate code block; discard ok
    // when error, output formatted debug message without err then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     $on_ok:block
        _DEBUG $dbg:expr, $($arg:expr),+;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok(_) => $on_ok
            Err(_) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg, $($arg),+ }

                $on_err
            }
        }
    };
    // when ok, evaluate code block; discard ok
    // when error, output formatted debug message without err then evaluate expression
    (
        WHEN   $when:expr;
        OK     $on_ok:block
        _DEBUG $dbg:expr, $($arg:expr),+;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok(_) => $on_ok
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg, $($arg),+ }

                $on_err
            }
        }
    };
    // when ok, evaluate code block; discard ok
    // when error, output formatted debug message with custom err then evaluate expression
    (
        WHEN   $when:expr;
        OK     $on_ok:block
        DEBUG  $err:ident; $dbg:expr, $($arg:expr),+;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok(_) => $on_ok
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg, $($arg),+ }

                $on_err
            }
        }
    };
    // when ok, evaluate expression; discard ok
    // when error, output debug message then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     $on_ok:expr;
        DEBUG  $dbg:expr;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok(_) => $on_ok,
            Err(err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), err }

                $on_err
            }
        }
    };
    // when ok, evaluate expression; discard ok
    // when error, output debug message then evaluate expression
    (
        WHEN   $when:expr;
        OK     $on_ok:expr;
        DEBUG  $dbg:expr;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok(_) => $on_ok,
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $err }

                $on_err
            }
        }
    };
    // when ok, evaluate expression; discard ok
    // when error, output formatted debug message then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     $on_ok:expr;
        DEBUG  $dbg:expr, $($arg:expr),+;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok(_) => $on_ok,
            Err(err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $($arg),+, err }

                $on_err
            }
        }
    };
    // when ok, evaluate expression; discard ok
    // when error, output formatted debug message then evaluate expression
    (
        WHEN   $when:expr;
        OK     $on_ok:expr;
        DEBUG  $dbg:expr, $($arg:expr),+;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok(_) => $on_ok,
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $($arg),+, $err }

                $on_err
            }
        }
    };
    // when ok, evaluate expression; discard ok
    // when error, output debug message without err then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     $on_ok:expr;
        _DEBUG $dbg:expr;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok(_) => $on_ok,
            Err(_) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg }

                $on_err
            }
        }
    };
    // when ok, evaluate expression; discard ok
    // when error, output debug message without err then evaluate expression
    (
        WHEN   $when:expr;
        OK     $on_ok:expr;
        _DEBUG $dbg:expr;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok(_) => $on_ok,
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg }

                $on_err
            }
        }
    };
    // when ok, evaluate expression; discard ok
    // when error, output formatted debug message without err then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     $on_ok:expr;
        _DEBUG $dbg:expr, $($arg:expr),+;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok(_) => $on_ok,
            Err(_) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg, $($arg),+ }

                $on_err
            }
        }
    };
    // when ok, evaluate expression; discard ok
    // when error, output formatted debug message without err then evaluate expression
    (
        WHEN   $when:expr;
        OK     $on_ok:expr;
        _DEBUG $dbg:expr, $($arg:expr),+;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok(_) => $on_ok,
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg, $($arg),+ }

                $on_err
            }
        }
    };
    // when ok, evaluate expression; discard ok
    // when error, output formatted debug message with custom err then evaluate expression
    (
        WHEN   $when:expr;
        OK     $on_ok:expr;
        DEBUG  $err:ident; $dbg:expr, $($arg:expr),+;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok(_) => $on_ok,
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg, $($arg),+ }

                $on_err
            }
        }
    };
    // when ok, evaluate code block with value
    // when error, output debug message then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:block
        DEBUG  $dbg:expr;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok($ok) => $on_ok
            Err(err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), err }

                $on_err
            }
        }
    };
    // when ok, evaluate code block with value
    // when error, output debug message then evaluate expression
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:block
        DEBUG  $dbg:expr;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok($ok) => $on_ok
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $err }

                $on_err
            }
        }
    };
    // when ok, evaluate code block with value
    // when error, output formatted debug message then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:block
        DEBUG  $dbg:expr, $($arg:expr),+;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok($ok) => $on_ok
            Err(err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $($arg),+, err }

                $on_err
            }
        }
    };
    // when ok, evaluate code block with value
    // when error, output formatted debug message then evaluate expression
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:block
        DEBUG  $dbg:expr, $($arg:expr),+;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok($ok) => $on_ok
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $($arg),+, $err }

                $on_err
            }
        }
    };
    // when ok, evaluate code block with value
    // when error, output debug message without err then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:block
        _DEBUG $dbg:expr;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok($ok) => $on_ok
            Err(_) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg }

                $on_err
            }
        }
    };
    // when ok, evaluate code block with value
    // when error, output debug message without err then evaluate expression
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:block
        _DEBUG $dbg:expr;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok($ok) => $on_ok
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg }

                $on_err
            }
        }
    };
    // when ok, evaluate code block with value
    // when error, output formatted debug message without err then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:block
        _DEBUG $dbg:expr, $($arg:expr),+;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok($ok) => $on_ok
            Err(_) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg, $($arg),+ }

                $on_err
            }
        }
    };
    // when ok, evaluate code block with value
    // when error, output formatted debug message without err then evaluate expression
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:block
        _DEBUG $dbg:expr, $($arg:expr),+;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok($ok) => $on_ok
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg, $($arg),+ }

                $on_err
            }
        }
    };
    // when ok, evaluate code block with value
    // when error, output formatted debug message with custom err then evaluate expression
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:block
        DEBUG  $err:ident; $dbg:expr, $($arg:expr),+;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok($ok) => $on_ok
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg, $($arg),+ }

                $on_err
            }
        }
    };
    // when ok, evaluate expression with value
    // when error, output debug message then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:expr;
        DEBUG  $dbg:expr;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok($ok) => $on_ok,
            Err(err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), err }

                $on_err
            }
        }
    };
    // when ok, evaluate expression with value
    // when error, output debug message then evaluate expression
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:expr;
        DEBUG  $dbg:expr;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok($ok) => $on_ok,
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $err }

                $on_err
            }
        }
    };
    // when ok, evaluate expression with value
    // when error, output formatted debug message then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:expr;
        DEBUG  $dbg:expr, $($arg:expr),+;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok($ok) => $on_ok,
            Err(err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $($arg),+, err }

                $on_err
            }
        }
    };
    // when ok, evaluate expression with value
    // when error, output formatted debug message then evaluate expression
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:expr;
        DEBUG  $dbg:expr, $($arg:expr),+;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok($ok) => $on_ok,
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $($arg),+, $err }

                $on_err
            }
        }
    };
    // when ok, evaluate expression with value
    // when error, output debug message without err then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:expr;
        _DEBUG $dbg:expr;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok($ok) => $on_ok,
            Err(_) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg }

                $on_err
            }
        }
    };
    // when ok, evaluate expression with value
    // when error, output debug message without err then evaluate expression
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:expr;
        _DEBUG $dbg:expr;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok($ok) => $on_ok,
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg }

                $on_err
            }
        }
    };
    // when ok, evaluate expression with value
    // when error, output formatted debug message without err then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:expr;
        _DEBUG $dbg:expr, $($arg:expr),+;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok($ok) => $on_ok,
            Err(_) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg, $($arg),+ }

                $on_err
            }
        }
    };
    // when ok, evaluate expression with value
    // when error, output formatted debug message without err then evaluate expression
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:expr;
        _DEBUG $dbg:expr, $($arg:expr),+;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok($ok) => $on_ok,
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg, $($arg),+ }

                $on_err
            }
        }
    };
    // when ok, evaluate expression with value
    // when error, output formatted debug message with custom err then evaluate expression
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:expr;
        DEBUG  $err:ident; $dbg:expr, $($arg:expr),+;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok($ok) => $on_ok,
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg, $($arg),+ }

                $on_err
            }
        }
    };
    // when ok, evaluate code block with mutable value
    // when error, output debug message then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:block
        DEBUG  $dbg:expr;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok(mut $ok) => $on_ok
            Err(err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), err }

                $on_err
            }
        }
    };
    // when ok, evaluate code block with mutable value
    // when error, output debug message then evaluate expression
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:block
        DEBUG  $dbg:expr;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok(mut $ok) => $on_ok
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $err }

                $on_err
            }
        }
    };
    // when ok, evaluate code block with mutable value
    // when error, output formatted debug message then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:block
        DEBUG  $dbg:expr, $($arg:expr),+;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok(mut $ok) => $on_ok
            Err(err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $($arg),+, err }

                $on_err
            }
        }
    };
    // when ok, evaluate code block with mutable value
    // when error, output formatted debug message then evaluate expression
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:block
        DEBUG  $dbg:expr, $($arg:expr),+;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok(mut $ok) => $on_ok
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $($arg),+, $err }

                $on_err
            }
        }
    };
    // when ok, evaluate code block with mutable value
    // when error, output debug message without err then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:block
        _DEBUG $dbg:expr;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok(mut $ok) => $on_ok
            Err(_) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg }

                $on_err
            }
        }
    };
    // when ok, evaluate code block with mutable value
    // when error, output debug message without err then evaluate expression
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:block
        _DEBUG $dbg:expr;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok(mut $ok) => $on_ok
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg }

                $on_err
            }
        }
    };
    // when ok, evaluate code block with mutable value
    // when error, output formatted debug message without err then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:block
        _DEBUG $dbg:expr, $($arg:expr),+;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok(mut $ok) => $on_ok
            Err(_) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg, $($arg),+ }

                $on_err
            }
        }
    };
    // when ok, evaluate code block with mutable value
    // when error, output formatted debug message without err then evaluate expression
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:block
        _DEBUG $dbg:expr, $($arg:expr),+;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok(mut $ok) => $on_ok
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg, $($arg),+ }

                $on_err
            }
        }
    };
    // when ok, evaluate code block with mutable value
    // when error, output formatted debug message with custom err then evaluate expression
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:block
        DEBUG  $err:ident; $dbg:expr, $($arg:expr),+;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok(mut $ok) => $on_ok
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg, $($arg),+ }

                $on_err
            }
        }
    };
    // when ok, evaluate expression with mutable value
    // when error, output debug message then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:expr;
        DEBUG  $dbg:expr;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok(mut $ok) => $on_ok,
            Err(err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), err }

                $on_err
            }
        }
    };
    // when ok, evaluate expression with mutable value
    // when error, output debug message then evaluate expression
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:expr;
        DEBUG  $dbg:expr;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok(mut $ok) => $on_ok,
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $err }

                $on_err
            }
        }
    };
    // when ok, evaluate expression with mutable value
    // when error, output formatted debug message then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:expr;
        DEBUG  $dbg:expr, $($arg:expr),+;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok(mut $ok) => $on_ok,
            Err(err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $($arg),+, err }

                $on_err
            }
        }
    };
    // when ok, evaluate expression with mutable value
    // when error, output formatted debug message then evaluate expression
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:expr;
        DEBUG  $dbg:expr, $($arg:expr),+;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok(mut $ok) => $on_ok,
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $($arg),+, $err }

                $on_err
            }
        }
    };
    // when ok, evaluate expression with mutable value
    // when error, output debug message without err then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:expr;
        _DEBUG $dbg:expr;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok(mut $ok) => $on_ok,
            Err(_) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg }

                $on_err
            }
        }
    };
    // when ok, evaluate expression with mutable value
    // when error, output debug message without err then evaluate expression
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:expr;
        _DEBUG $dbg:expr;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok(mut $ok) => $on_ok,
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg }

                $on_err
            }
        }
    };
    // when ok, evaluate expression with mutable value
    // when error, output formatted debug message without err then evaluate expression; discard err
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:expr;
        _DEBUG $dbg:expr, $($arg:expr),+;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok(mut $ok) => $on_ok,
            Err(_) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg, $($arg),+ }

                $on_err
            }
        }
    };
    // when ok, evaluate expression with mutable value
    // when error, output formatted debug message without err then evaluate expression
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:expr;
        _DEBUG $dbg:expr, $($arg:expr),+;
        ERR    $err:ident; $on_err:expr
    ) => {
        match $when {
            Ok(mut $ok) => $on_ok,
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg, $($arg),+ }

                $on_err
            }
        }
    };
    // when ok, evaluate expression with mutable value
    // when error, output formatted debug message with custom err then evaluate expression
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:expr;
        DEBUG  $err:ident; $dbg:expr, $($arg:expr),+;
        ERR    $on_err:expr
    ) => {
        match $when {
            Ok(mut $ok) => $on_ok,
            Err($err) => {
                #[cfg(feature = "debug-result")]
                cli_toolbox::debug! { ERR $dbg, $($arg),+ }

                $on_err
            }
        }
    };
}
