# Examples

### Result-Macro

Run the `result!` macro example with the following commands to see its different behaviors

* no exceptions unoptimized build

```shell
cargo run --example result-macro --features="result-debug" 
```

* no exceptions optimized build

```shell
cargo run --release --example result-macro --features="result-debug" 
```

* exception in unoptimized build

```shell
cargo run --example result-macro --features="result-debug" -- break 
```

* exception in optimized build

```shell
cargo run --release --example result-macro --features="result-debug" -- break 
```

* exception in unoptimized build without the `result-debug` feature

```shell
cargo run --example result-macro --features="result" -- break 
```