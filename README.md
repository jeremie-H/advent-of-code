# Advent of code

_First of all, navigate into one year directory, like : `2015/`_  
_Then use one of these command_

## Quickstart
### How to execute all days 🚀
```shell
cargo aoc
```

### How to bench all days 🚀
```shell
cargo aoc-bench
```

### How to test with live reload for one day 👀
_For example, live reload on day01_

```shell
cargo-watch -x "aoc-test day01"
```

You need as a requirement to install `cargo-watch` with the following command :
```
cargo install cargo-watch
```

---

## Helping commands
_for helping, there are some more commands here, if needed_  


### How to run test on a specific day
```shell
cargo aoc-test day12
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

### How to format code with the nightly edition
```shell
cargo +nightly fmt
```
