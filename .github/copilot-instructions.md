# Copilot instructions for sereno

## Project overview
This repository is described as "A lightweight PKI, certificate auditor, and secure vault built in Rust. [CISA Memory Safety Framework]". The current checkout is intentionally minimal, but the project intent is clearly Rust-first and security-focused.

When working here, keep the mental model centered on:
- PKI and certificate handling
- certificate auditing and trust validation
- secure storage and sensitive material handling
- memory-safe Rust patterns

The repository is provisioned with a Rust devcontainer (`mcr.microsoft.com/devcontainers/rust:1-bookworm`) and rust-analyzer, which is the expected environment for future implementation work.

## Build, test, and lint commands
No repository-defined build/test/lint scripts or Cargo manifest were present in this checkout, so there are no authoritative project commands to run yet.

When the project is populated with a Rust crate or workspace, use Cargo as the source of truth:
- `cargo build`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`

For a single test, prefer Cargo's filtering:
- `cargo test <test_name>`
- `cargo test <module>::<test_name>`

If the repo later adds a `justfile` or `Makefile`, prefer those scripts over ad hoc commands.

## High-level architecture
The repository's stated purpose points to a project with three main concerns that should remain conceptually separate unless the codebase intentionally changes:

1. PKI and certificate logic
   - parsing, validating, and interpreting certificate material
   - trust-chain and identity checks
2. Security auditing
   - certificate quality checks
   - policy or compliance review logic
   - detection of weak or dangerous certificate patterns
3. Secure vaulting / secret handling
   - storage boundaries for sensitive material
   - constrained access to protected data

In practice, keep certificate parsing, auditing, and vaulting concerns in distinct modules or crates rather than mixing them into a single layer. This project is intended to be security-sensitive, so isolation of responsibilities matters more than raw convenience.

## Key conventions
- Treat Rust memory safety and security as first-class constraints; prefer explicit ownership and type-safe APIs.
- Keep certificate and cryptography logic in clearly named modules instead of ad hoc helper functions.
- Prefer small, testable functions and typed errors over broad, hidden side effects.
- Follow Cargo-friendly layout conventions when adding code: use a crate root and focused submodules, not a monolithic implementation file.
- Keep unsafe usage minimal and well-justified; this repository's intent is aligned with the CISA Memory Safety Framework and should not be treated as a place for low-safety shortcuts.
- Until a different structure is introduced, assume the project will follow standard Rust crate/workspace conventions rather than framework-specific patterns.

No repo-specific AGENTS.md/CLAUDE.md/Cursor rules were present in this checkout; this file is the source of truth for future Copilot sessions in this repository.
