# jest_lint

Lint your Jest unit tests to find problems. Built with Rust.

## Features

- **Mock checking** -- checks that all imports in the module under test have a corresponding `jest.mock()` in the test file
- **Flagged expect args** -- flags suspect words (e.g. "stub") inside `expect()` calls, since stub values shouldn't be asserted on

All features are enabled by default. Use `.jest_lint.json` to customize behavior.

## Getting started

First: [Install Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

Then, either install the published crate (easy) or download from this repo.

### Install published crate (easy)

```
cargo install jest_lint
jest_lint --help
```

Check all files in the current directory:

```
jest_lint
```

Check a specific directory:

```
jest_lint -d path/to/files
```

Check specific files:

```
jest_lint -f path/to/foobar.test.js
jest_lint path/to/foo.test.js path/to/bar.test.js
```

### Latest development build

Download this repository.

```
cd jest_lint
cargo run -- --help
```

PRs welcome!

## Configuration

Create a `.jest_lint.json` file in your project root. The config file is automatically discovered
by searching up from the checked file's directory.

```json
{
  "ignoredModules": [
    "zod",
    "@mui/**",
    "*.module.scss",
    "next/**",
    "**/types/*"
  ],
  "expectArgs": {
    "flagged": ["stub"],
    "severity": "warning"
  }
}
```

### Ignored modules

Patterns support exact matches and glob syntax (`*` for single level, `**` for nested paths).

You can also ignore individual imports inline in your test file with a comment:

```ts
// jest_lint:ignore ./utils
```

### Flagged expect args

Flags lines where `expect()` contains a word from the `flagged` list. This catches cases where
stub values leak into assertions, e.g. `expect(stubName).toBe("hello")`.

| Field | Default | Description |
|---|---|---|
| `enabled` | `true` | Set to `false` to disable this check |
| `flagged` | `[]` | Words to flag inside `expect()` calls |
| `severity` | `"error"` | `"error"` (exit 1) or `"warning"` (exit 0) |

See the [examples](examples/) directory for a working demo.

## Tips & Tricks

If you're using VS Code, you can add a task to `.vscode/tasks.json` to run `jest_lint` on the current file:

```json
{
  "label": "jest_lint",
  "type": "shell",
  "command": "jest_lint -f ${file}"
}
```

Then you can use a keyboard shortcut to check your mocks while you have your `.spec.*` or `.test.*` file open.
