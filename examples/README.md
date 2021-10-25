# Examples

### Result-Macro

Run the `result!` macro example with the following commands to see its different behaviors

* no exceptions in optimized build

```shell
cargo run --example result-macro --features="result-debug" --release 
```

* exception in optimized build

```shell
cargo run --example result-macro --features="result-debug" --release -- break 
```

* no exceptions in unoptimized build

```shell
cargo run --example result-macro --features="result-debug" 
```

* exception in unoptimized build

```shell
cargo run --example result-macro --features="result-debug" -- break 
```

* exception in unoptimized build without the `result-debug` feature

```shell
cargo run --example result-macro --features="result" -- break 
```

### Option-Macro

Run the `option!` macro example with the following commands to see its different behaviors

* some in optimized build

```shell
cargo run --example option-macro --features="option-debug" --release 
```

* none in optimized build

```shell
cargo run --example option-macro --features="option-debug" --release -- none 
```

* some in unoptimized build

```shell
cargo run --example option-macro --features="option-debug" 
```

* none in unoptimized build

```shell
cargo run --example option-macro --features="option-debug" -- none 
```

* none in unoptimized build without the `option-debug` feature

```shell
cargo run --example option-macro --features="option" -- none 
```
