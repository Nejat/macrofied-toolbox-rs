///
#[macro_export]
macro_rules! result {
    (
        WHEN   $when:expr;
        OK     $on_ok:expr
    ) => {
        if $when.is_ok() { $on_ok }
    };
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:block
    ) => {
        if let Ok($ok) = $when { $on_ok }
    };
    (
        WHEN   $when:expr;
        OK     $ok:ident; $on_ok:expr
    ) => {
        if let Ok($ok) = $when { $on_ok }
    };
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:block
    ) => {
        if let Ok(mut $ok) = $when { $on_ok }
    };
    (
        WHEN   $when:expr;
        OK     mut $ok:ident; $on_ok:expr
    ) => {
        if let Ok(mut $ok) = $when { $on_ok }
    };
    (
        WHEN   $when:expr;
        DEBUG  $dbg:expr
    ) => {
        if let Err(err) = $when {
            #[cfg(feature = "debug-result")]
            cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), err }
        }
    };
    (
        WHEN   $when:expr;
        DEBUG  $dbg:expr, $($arg:expr),+
    ) => {
        if let Err(err) = $when {
            #[cfg(feature = "debug-result")]
            cli_toolbox::debug! { ERR concat!($dbg, ": {:?}"), $($arg),+, err }
        }
    };
    (
        WHEN   $when:expr;
        _DEBUG $dbg:expr
    ) => {
        if $when.is_err() {
            #[cfg(feature = "debug-result")]
            cli_toolbox::debug! { ERR $dbg }
        }
    };
    (
        WHEN   $when:expr;
        _DEBUG $dbg:expr, $($arg:expr),+
    ) => {
        if $when.is_err() {
            #[cfg(feature = "debug-result")]
            cli_toolbox::debug! { ERR $dbg, $($arg),+ }
        }
    };
    (
        WHEN   $when:expr;
        DEBUG  $err:ident; $dbg:expr, $($arg:expr),+
    ) => {
        if let Err($err) = $when {
            #[cfg(feature = "debug-result")]
            cli_toolbox::debug! { ERR $dbg, $($arg),+ }
        }
    };
    (
        WHEN   $when:expr;
        ERR    $on_err:expr
    ) => {
        if $when.is_err() {
        }
    };
    (
        WHEN   $when:expr;
        ERR    $err:ident; $on_err:expr
    ) => {
        if let Err($err) = $when {
            $on_err
        }
    };
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
