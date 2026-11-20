# Copilot instructions for sereno

## Project overview
Sereno is a Rust PKI, certificate-auditing, and secure-vault project aligned with the CISA Memory Safety Framework. The checkout currently contains one small binary crate; implementation work may expand it substantially.

## Build, test, and lint commands
The Cargo package is in `sereno/`, not at the repository root.

```bash
cd sereno
cargo build
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

To run one test by name or module path:

```bash
cargo test <test_name>
cargo test <module>::<test_name>
```

There are currently no repository test cases (`cargo test` reports 0 tests). Formatting, test, and lint commands are Cargo-native; no Makefile or task runner is present.

## High-level architecture
- `sereno/Cargo.toml` defines the single `sereno` binary package using Rust edition 2024.
- `sereno/src/main.rs` is the only application source file and currently prints `Hello, world!`; it is not yet a library or modular CLI.
- The dependency set identifies intended implementation areas: `x509-parser` for certificate parsing, `sha2` for hashing, `chrono` for time and validity handling, `serde`/`serde_json` for structured data, and `anyhow` for application errors.
- README intent separates PKI/certificate processing, security auditing/trust validation, and secure vault storage. Keep these concerns in separate modules or crates as the implementation grows rather than combining certificate logic and vault handling in `main.rs`.
- `.devcontainer/devcontainer.json` uses the Rust Debian Bookworm image and rust-analyzer.

## Key conventions
- Run Cargo commands from `sereno/`; the repository root has no Cargo manifest.
- Use Rust 2024 edition idioms and keep ownership explicit.
- Preserve the separation among certificate parsing/validation, audit rules, and secret/vault storage; do not expose private key material or plaintext secrets in logs, diagnostics, serialized audit output, or example fixtures.
- Prefer typed, contextual errors (including `thiserror` if/where a public error enum is needed) over broad hidden side effects. `anyhow` is already the application dependency.
- For cryptographic and certificate code, keep parsing and policy decisions testable independently and minimize `unsafe` code.
- `target/` and Rust build artifacts are gitignored.

No separate AGENTS.md, CLAUDE.md, Cursor, or Cline instruction files are present.
