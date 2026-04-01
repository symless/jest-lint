# jest_lint

Lint your Jest unit tests to find problems. Built with Rust.

Right now this is a simple tool to check that you have mocked your imports. But if you have an
idea for adding a new feature, feel free to suggest.

## Getting started

First: [Install Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

Then, either install the published crate (easy) or download from this repo.

### Install published crate (easy)

```
cargo install jest_lint
jest_lint --help
```

To check for mocks in all files in the current directory:

```
jest_lint -m
```

To check for mocks in a specific directory:

```
jest_lint -m -d path/to/files
```

To check for mocks in specific files:

```
jest_lint -m -f path/to/foobar.test.js
jest_lint -m path/to/foo.test.js path/to/bar.test.js
```

### Latest development build

Download this repository.

```
cd jest_lint
cargo run -- --help
```

PRs welcome!

## Configuration

Create a `.jest_lint.json` file in your project root to configure which modules should be ignored
when checking for mocks. The config file is automatically discovered by searching up from the
checked file's directory.

```json
{
  "ignoredModules": [
    "zod",
    "@mui/**",
    "*.module.scss",
    "next/**",
    "**/types/*"
  ]
}
```

Patterns support exact matches and glob syntax (`*` for single level, `**` for nested paths).

## Tips & Tricks

If you're using VS Code, you can add a task to `.vscode/tasks.json` to run `jest_lint` on the current file:

```json
{
  "label": "jest_lint",
  "type": "shell",
  "command": "jest_lint -mf ${file}"
}
```

Then you can use a keyboard shortcut to check your mocks while you have your `.spec.*` or `.test.*` file open.
