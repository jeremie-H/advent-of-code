# Advent of code

_First of all, navigate into one year directory, like : `2015/`_  
_Then use one of these command_

## Quickstart
### How to bench all days 🚀
```shell
cargo run --manifest-path bench/Cargo.toml --release
```

### How to test with live reload for one day 👀
```shell
cargo-watch -x "test -p day<X> -- --nocapture"
```
---

## Helping commands
_for helping, there are some more commands here, if needed_  

### How to initialize a new day
```shell
cargo new --lib src/day24
```

### How to run test on a specific day
```shell
cargo test -p day12
```


### How to build all
```shell
cargo build --workspace
```

### How to test all the workspace
```shell
cargo test --workspace
```


### How to check clippy on every crates ?
```shell
cargo clippy --all
```
