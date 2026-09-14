# Project rules for AI coding tools

## Project

ddapi-rs — Rust client library for the DDNet and DDStats APIs (Teeworlds).
`src/scheme/` — serde structs mirroring the live APIs; `src/api/` — request
layer; `tests/` — schema-consistency tests.

## Commands

- Build/test: `cargo test --all-features`
- Strict clippy (mandatory, CI-enforced):
  `cargo clippy --all-targets --all-features -- -D clippy::pedantic -D clippy::unwrap_used -D clippy::expect_used -D warnings`
- Format: `cargo fmt`
- Typos: `typos .` (fixtures excluded via `typos.toml`)

## Conventions

- Zero style: no `unwrap`/`expect`/panic in library code; propagate errors,
  use `assert!` in tests.
- `#[must_use]` on value-returning public methods.
- When the live schema check fails, update structs to match the API — do not
  weaken the test.

## Live schema tests

Ignored by default; run with `cargo test --all-features -- --ignored`.
Weekly CI job (`schema_check.yml`) catches upstream drift: new fields the
structs do not model yet. Player-dependent tests use a candidate list and
pass on the first clean payload.

## Source of truth for DDStats

`ddstats-web` repo (https://git.ddstats.tw/ddstats/ddstats-web), `src/models/`
— structs must mirror it (field names, types, Option-ness).