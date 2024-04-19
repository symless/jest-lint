# jest_lint

Lint your Jest unit tests to find problems. Built with Rust.

## Getting started

```
cd jest_lint
cargo run -- --help
```

To test a single file:

```
cargo run -- -m -f path/to/foobar.test.js
```

To test all the files:

```
cargo run -- -m
```
