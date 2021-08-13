# Examples

### Result-Macro

Run the `result!` macro example with the following commands to see it's different behaviors

* no exceptions [release|debug]

```shell
cargo run --example result-macro --features="result debug-result" 
```

* exception in debug build

```shell
cargo run --example result-macro --features="result debug-result" -- break 
```

* exception in release build

```shell
cargo run --release --example result-macro --features="result debug-result" -- break 
```

* exception in debug build without the `debug-result` feature

```shell
cargo run --example result-macro --features="result" -- break 
```