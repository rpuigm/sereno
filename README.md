# sereno

A lightweight PKI, certificate auditor, and secure vault built in Rust. [CISA Memory Safety Framework]

## CLI

The current CLI provides `version` (print the binary version), `doctor` (show
basic runtime and host information), and `completion` (print a shell completion
script). PKI operations, certificate auditing, and vault commands are not
implemented yet.

```bash
cd sereno
cargo run -- --help
cargo run -- version
cargo run -- doctor
cargo run -- completion bash
```

Use `--format` to select `text` (the default) or `json` for informational
commands. The option can appear before or after the subcommand:

```bash
cargo run -- --format json version
cargo run -- version --format json
```

Run `cargo run -- <command> --help` for command-specific help. The completion
command prints a script to standard output; redirect it to a file if needed.

## Versioning

Sereno follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html). After
`1.0.0`, increase the major version for breaking changes, the minor version for
backward-compatible features, and the patch version for backward-compatible
fixes. Before `1.0.0`, the public API is considered unstable, so a minor
version may include breaking changes. The current package version is declared
in `sereno/Cargo.toml`.

For a release, update the version in `sereno/Cargo.toml`, run `cargo check` to
refresh `sereno/Cargo.lock`, move the release notes from `Unreleased` to a
versioned section in [CHANGELOG.md](CHANGELOG.md), and run the project's
validation commands (`cargo fmt --check`, `cargo test`, and
`cargo clippy --all-targets --all-features -- -D warnings`). After committing
those changes, create an annotated tag:

```bash
git tag -a vX.Y.Z -m "Release vX.Y.Z"
```
