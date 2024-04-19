# jest_lint

Lint your Jest unit tests to find problems. Built with Rust.

## Getting started

First: [Install Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

Then, either install the published create (easy) or download from this repo.

### Install published crate (easy)

```
cargo install jest_lint
jest_lint --help
```

To test all the files:
```
jest_lint -m
```

To test a specific dir:
```
jest_lint -d path/to/files
```

To test a single file:
```
jest_lint -m -f path/to/foobar.test.js
```


### Latest development build

Download this repository.

```
cd jest_lint
cargo run -- --help
```

PRs welcome!
