# jest_lint

Lint your Jest unit tests to find problems. Built with Rust.

Right now this is a simple tool to check that you have mocked your imports. But if you have an
idea for adding a new feature, feel free to suggest 

If you want to ignore a region of imports (e.g. if you intentionally don't want to mock them),
you can use a region: `// #region not-mocked` (see the [samples](samples) dir for more context)

## Getting started

First: [Install Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

Then, either install the published create (easy) or download from this repo.

### Install published crate (easy)

```
cargo install jest_lint
jest_lint --help
```

To test for mocks in all your files:
```
jest_lint -m
```

To test for mocks in files for a specific dir:
```
jest_lint -d path/to/files
```

To test for mocks in a single file:
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

## Tips & Tricks

If you're using VS Code, you can add a task to run `jest_lint` on the current file:
```
    {
      "label": "jest_lint",
      "type": "shell",
      "command": "jest_lint -mf ${file}"
    }
```

Then you can use a keyboard shortcut to check your mocks while you have your `.spec.*` or `.test.*` file open.
