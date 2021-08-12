/// The `result!` macro provides ergonomic handling of debug messages when
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
///
/// * when ok, evaluate expression; discard ok
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     junk()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate code block with value
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; { if val == 42 { junk(); } }
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate expression with value
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; if val == 42 { junk(); }
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate code block with mutable value
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; { junk(&mut val); }
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// ```
/// * when ok, evaluate expression with mutable value
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; junk(&mut val)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// ```
/// * when error, output debug message
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     DEBUG  "foo failed"
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// ```
/// * when error, output formatted debug message
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     DEBUG  "foo failed: {}", 42
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// ```
/// * when error, output debug message without err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     _DEBUG "foo failed"
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// ```
/// * when error, output formatted debug message without err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     _DEBUG "foo failed: {}", 42
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// ```
/// * when error, output formatted debug message with custom err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     DEBUG  err; "foo failed: {}, err: {:?}", 42, err
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// ```
/// * when error, evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn process_error() {}
/// ```
/// * when error, evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate code block; discard ok\
///   when error, output debug message
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     { junk() }
///     DEBUG  "foo failed"
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate code block; discard ok\
///   when error, output formatted debug message
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     { junk() }
///     DEBUG  "foo failed: {}", 42
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate code block; discard ok\
///   when error, output debug message without err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     { junk() }
///     _DEBUG "foo failed"
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate code block; discard ok\
///   when error, output formatted debug message without err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     { junk() }
///     _DEBUG "foo failed: {}", 42
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate code block; discard ok\
///   when error, output formatted debug message with custom err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     { junk() }
///     DEBUG  err; "foo failed: {}, err: {:?}", 42, err
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate expression; discard ok\
///   when error, output debug message
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     junk();
///     DEBUG  "foo failed"
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate expression; discard ok\
///   when error, output formatted debug message
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     junk();
///     DEBUG  "foo failed: {}", 42
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate expression; discard ok\
///   when error, output debug message without err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     junk();
///     _DEBUG "foo failed"
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate expression; discard ok\
///   when error, output formatted debug message without err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     junk();
///     _DEBUG "foo failed: {}", 42
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate expression; discard ok\
///   when error, output formatted debug message with custom err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     junk();
///     DEBUG  err; "foo failed: {}, err: {:?}", 42, err
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate code block with value\
///   when error, output debug message
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; { if val == 42 { junk(); } }
///     DEBUG  "foo failed"
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate code block with value\
///   when error, output formatted debug message
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; { if val == 42 { junk(); } }
///     DEBUG  "foo failed: {}", 42
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate code block with value\
///   when error, output debug message without err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; { if val == 42 { junk(); } }
///     _DEBUG "foo failed"
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate code block with value\
///   when error, output formatted debug message without err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; { if val == 42 { junk(); } }
///     _DEBUG "foo failed: {}", 42
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate code block with value\
///   when error, output formatted debug message with custom err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; { if val == 42 { junk(); } }
///     DEBUG  err; "foo failed: {}, err: {:?}", 42, err
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate expression with value\
///   when error, output debug message
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; if val == 42 { junk(); };
///     DEBUG  "foo failed"
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate expression with value\
///   when error, output formatted debug message
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; if val == 42 { junk(); };
///     DEBUG  "foo failed: {}", 42
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate expression with value\
///   when error, output debug message without err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; if val == 42 { junk(); };
///     _DEBUG "foo failed"
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate expression with value\
///   when error, output formatted debug message without err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; if val == 42 { junk(); };
///     _DEBUG "foo failed: {}", 42
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate expression with value\
///   when error, output formatted debug message with custom err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; if val == 42 { junk(); };
///     DEBUG  err; "foo failed: {}, err: {:?}", 42, err
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// ```
/// * when ok, evaluate code block with mutable value\
///   when error, output debug message
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; { junk(&mut val); }
///     DEBUG  "foo failed"
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// ```
/// * when ok, evaluate code block with mutable value\
///   when error, output formatted debug message
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; { junk(&mut val); }
///     DEBUG  "foo failed: {}", 42
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// ```
/// * when ok, evaluate code block with mutable value\
///   when error, output debug message without err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; { junk(&mut val); }
///     _DEBUG "foo failed"
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// ```
/// * when ok, evaluate code block with mutable value\
///   when error, output formatted debug message without err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; { junk(&mut val); }
///     _DEBUG "foo failed: {}", 42
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// ```
/// * when ok, evaluate code block with mutable value\
///   when error, output formatted debug message with custom err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; { junk(&mut val); }
///     DEBUG  err; "foo failed: {}, err: {:?}", 42, err
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// ```
/// * when ok, evaluate expression with mutable value\
///   when error, output debug message
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; junk(&mut val);
///     DEBUG  "foo failed"
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// ```
/// * when ok, evaluate expression with mutable value\
///   when error, output formatted debug message
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; junk(&mut val);
///     DEBUG  "foo failed: {}", 42
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// ```
/// * when ok, evaluate expression with mutable value\
///   when error, output debug message without err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; junk(&mut val);
///     _DEBUG "foo failed"
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// ```
/// * when ok, evaluate expression with mutable value\
///   when error, output formatted debug message without err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; junk(&mut val);
///     _DEBUG "foo failed: {}", 42
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// ```
/// * when ok, evaluate expression with mutable value\
///   when error, output formatted debug message with custom err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; junk(&mut val);
///     DEBUG  err; "foo failed: {}, err: {:?}", 42, err
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// ```
/// * when ok, evaluate code block; discard ok\
///   when error, output debug message then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     { junk() }
///     DEBUG  "foo failed";
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate code block; discard ok\
///   when error, output debug message then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     { junk() }
///     DEBUG  "foo failed";
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate code block; discard ok\
///   when error, output formatted debug message then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     { junk() }
///     DEBUG  "foo failed: {}", 42;
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate code block; discard ok\
///   when error, output formatted debug message then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     { junk() }
///     DEBUG  "foo failed: {}", 42;
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate code block; discard ok\
///   when error, output debug message without err then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     { junk() }
///     _DEBUG "foo failed";
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate code block; discard ok\
///   when error, output debug message without err then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     { junk() }
///     _DEBUG "foo failed";
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate code block; discard ok\
///   when error, output formatted debug message without err then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     { junk() }
///     _DEBUG "foo failed: {}", 42;
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate code block; discard ok\
///   when error, output formatted debug message without err then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     { junk() }
///     _DEBUG "foo failed: {}", 42;
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate code block; discard ok\
///   when error, output formatted debug message with custom err then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     { junk() }
///     DEBUG  err; "foo failed: {}, err: {:?}", 42, err;
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate expression; discard ok\
///   when error, output debug message then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     junk();
///     DEBUG  "foo failed";
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate expression; discard ok\
///   when error, output debug message then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     junk();
///     DEBUG  "foo failed";
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate expression; discard ok\
///   when error, output formatted debug message then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     junk();
///     DEBUG  "foo failed: {}", 42;
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate expression; discard ok\
///   when error, output formatted debug message then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     junk();
///     DEBUG  "foo failed: {}", 42;
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate expression; discard ok\
///   when error, output debug message without err then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     junk();
///     _DEBUG "foo failed";
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate expression; discard ok\
///   when error, output debug message without err then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     junk();
///     _DEBUG "foo failed";
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate expression; discard ok\
///   when error, output formatted debug message without err then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     junk();
///     _DEBUG "foo failed: {}", 42;
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate expression; discard ok\
///   when error, output formatted debug message without err then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     junk();
///     _DEBUG "foo failed: {}", 42;
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate expression; discard ok\
///   when error, output formatted debug message with custom err then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     junk();
///     DEBUG  err; "foo failed: {}, err: {:?}", 42, err;
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate code block with value\
///   when error, output debug message then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; { if val == 42 { junk(); } }
///     DEBUG  "foo failed";
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate code block with value\
///   when error, output debug message then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; { if val == 42 { junk(); } }
///     DEBUG  "foo failed";
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate code block with value\
///   when error, output formatted debug message then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; { if val == 42 { junk(); } }
///     DEBUG  "foo failed: {}", 42;
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate code block with value\
///   when error, output formatted debug message then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; { if val == 42 { junk(); } }
///     DEBUG  "foo failed: {}", 42;
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate code block with value\
///   when error, output debug message without err then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; { if val == 42 { junk(); } }
///     _DEBUG "foo failed";
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate code block with value\
///   when error, output debug message without err then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; { if val == 42 { junk(); } }
///     _DEBUG "foo failed";
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate code block with value\
///   when error, output formatted debug message without err then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; { if val == 42 { junk(); } }
///     _DEBUG "foo failed: {}", 42;
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate code block with value\
///   when error, output formatted debug message without err then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; { if val == 42 { junk(); } }
///     _DEBUG "foo failed: {}", 42;
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate code block with value\
///   when error, output formatted debug message with custom err then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; { if val == 42 { junk(); } }
///     DEBUG  err; "foo failed: {}, err: {:?}", 42, err;
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate expression with value\
///   when error, output debug message then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; if val == 42 { junk(); };
///     DEBUG  "foo failed";
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate expression with value\
///   when error, output debug message then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; if val == 42 { junk(); };
///     DEBUG  "foo failed";
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate expression with value\
///   when error, output formatted debug message then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; if val == 42 { junk(); };
///     DEBUG  "foo failed: {}", 42;
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate expression with value\
///   when error, output formatted debug message then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; if val == 42 { junk(); };
///     DEBUG  "foo failed: {}", 42;
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate expression with value\
///   when error, output debug message without err then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; if val == 42 { junk(); };
///     _DEBUG "foo failed";
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate expression with value\
///   when error, output debug message without err then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; if val == 42 { junk(); };
///     _DEBUG "foo failed";
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate expression with value\
///   when error, output formatted debug message without err then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; if val == 42 { junk(); };
///     _DEBUG "foo failed: {}", 42;
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate expression with value\
///   when error, output formatted debug message without err then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; if val == 42 { junk(); };
///     _DEBUG "foo failed: {}", 42;
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate expression with value\
///   when error, output formatted debug message with custom err then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     val; if val == 42 { junk(); };
///     DEBUG  err; "foo failed: {}, err: {:?}", 42, err;
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk() {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate code block with mutable value\
///   when error, output debug message then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; { junk(&mut val); }
///     DEBUG  "foo failed";
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate code block with mutable value\
///   when error, output debug message then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; { junk(&mut val); }
///     DEBUG  "foo failed";
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate code block with mutable value\
///   when error, output formatted debug message then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; { junk(&mut val); }
///     DEBUG  "foo failed: {}", 42;
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate code block with mutable value\
///   when error, output formatted debug message then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; { junk(&mut val); }
///     DEBUG  "foo failed: {}", 42;
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate code block with mutable value\
///   when error, output debug message without err then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; { junk(&mut val); }
///     _DEBUG "foo failed";
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate code block with mutable value\
///   when error, output debug message without err then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; { junk(&mut val); }
///     _DEBUG "foo failed";
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate code block with mutable value\
///   when error, output formatted debug message without err then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; { junk(&mut val); }
///     _DEBUG "foo failed: {}", 42;
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate code block with mutable value\
///   when error, output formatted debug message without err then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; { junk(&mut val); }
///     _DEBUG "foo failed: {}", 42;
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate code block with mutable value\
///   when error, output formatted debug message with custom err then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; { junk(&mut val); }
///     DEBUG  err; "foo failed: {}, err: {:?}", 42, err;
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate expression with mutable value\
///   when error, output debug message then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; junk(&mut val);
///     DEBUG  "foo failed";
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate expression with mutable value\
///   when error, output debug message then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; junk(&mut val);
///     DEBUG  "foo failed";
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate expression with mutable value\
///   when error, output formatted debug message then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; junk(&mut val);
///     DEBUG  "foo failed: {}", 42;
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate expression with mutable value\
///   when error, output formatted debug message then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; junk(&mut val);
///     DEBUG  "foo failed: {}", 42;
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate expression with mutable value\
///   when error, output debug message without err then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; junk(&mut val);
///     _DEBUG "foo failed";
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate expression with mutable value\
///   when error, output debug message without err then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; junk(&mut val);
///     _DEBUG "foo failed";
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate expression with mutable value\
///   when error, output formatted debug message without err then evaluate expression; discard err
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; junk(&mut val);
///     _DEBUG "foo failed: {}", 42;
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// # fn process_error() {}
/// ```
/// * when ok, evaluate expression with mutable value\
///   when error, output formatted debug message without err then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; junk(&mut val);
///     _DEBUG "foo failed: {}", 42;
///     ERR    err; process_error(err)
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// # fn process_error<E>(_err: E) {}
/// ```
/// * when ok, evaluate expression with mutable value\
///   when error, output formatted debug message with custom err then evaluate expression
///
/// ```rust
/// # use macrofied_toolbox::result;
/// # use cli_toolbox::debug;
/// result! {
///     WHEN   foo();
///     OK     mut val; junk(&mut val);
///     DEBUG  err; "foo failed: {}, err: {:?}", 42, err;
///     ERR    process_error()
/// }
/// # fn foo() -> Result<usize, &'static str> { Ok(42) }
/// # fn junk(_val: &mut usize) {}
/// # fn process_error() {}
/// ```
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
