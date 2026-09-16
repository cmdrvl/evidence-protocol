# Contributing

This crate is deliberately small. Two rules keep it that way.

## 1. A contract is added only with a named consumer

Every new contract, and every new field on an existing contract, arrives in a
change that names the consuming code: a bead or PR in `SaltIO/evidence-machine`
(or another named consumer) that will read or write the field. No speculative
fields. No "we will probably need". If the consumer is not written yet, the
field is not either.

The change ships four things together:

1. the serde type (or field) under `src/v0/<contract>.rs`;
2. the JSON Schema under `schemas/<contract>.v0.schema.json`;
3. at least one valid and one invalid fixture under `fixtures/<contract>/`;
4. a round-trip test: deserialize → schema-validate → canonicalize →
   serialize equals the canonicalized input.

Removing or renaming a field in a `.v0` contract is a new version, not an
edit.

## 2. Canonical JSON

Content hashes must be identical across implementers, so serialization is
canonical:

- object keys sorted by Unicode code point (use `BTreeMap`, never `HashMap`);
- no insignificant whitespace;
- integers as integers; money and counts are never floats;
- strings NFC-normalized;
- hashes written as `blake3:<hex>` (lowercase);
- timestamps RFC 3339 in UTC with a `Z` suffix.

The hash of a protocol object is BLAKE3 over its canonical JSON bytes.

## What does not belong here

- `operator.v0` and conformance rules (`spine-rules`).
- Runtime concerns: scheduling, budgets in flight, journals, brokers
  (`evidence-machine`).
- Domain payload schemas (identity edges, parsed tapes, and so on). Those
  travel as `artifact.v0` payloads under schemas owned by the runtime or the
  operator that produces them, until a second consumer needs them shared.
- Any dependency on CMD+RVL code, public or private. `deny.toml` enforces the
  private half; reviewers enforce the rest.

## Quality gate

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo deny check bans sources licenses   # requires cargo-deny
```
