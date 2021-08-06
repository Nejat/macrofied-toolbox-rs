# macrofied-toolbox

This library provides the macroification of some fundamental rust boilerplate patterns, i.e. option, result, etc.

.

.

.

## Resources
* [Docs](https://docs.rs/macrofied-toolbox/0.1.0/macrofied_toolbox/) for more detailed information
* [Examples](https://github.com/Nejat/macrofied-toolbox-rs/tree/v0.1.0/examples) to see it in action

## Usage

Each macro is gated by a feature.

No feature is mutually exclusive and can be combined as needed.

```toml
[dependencies]
macrofied-toolbox = { version = "0.1", features = ["option", "result", "command", "path"] }
```

### Additional Feature

`macrofied-toolbox` can optionally use the [`cli-toolbox`](https://github.com/Nejat/cli-toolbox-rs) crate to output 
debug to the console by enabling an of these features

* `debug-all` - enables console debugging for all the features enabled
* `debug-option` - enables console debugging for the `option!` macro
* `debug-result` - enables console debugging for the `result!` macro
* `debug-command` - enables console debugging for the `command!` macro

## Roadmap

* [ ] `result!` - handles a `Result<T,E>` of an expression  
* [ ] `option!` - handles an `Option<T>` of an expression
* [ ] `command!` - execute a child process command
* [ ] `path!` - path building functionality

## Implemented
* [ ] ...

