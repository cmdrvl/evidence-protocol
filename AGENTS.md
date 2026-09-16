# AGENTS.md — evidence-protocol

> Guidelines for AI coding agents working in this Rust crate.

---

## What This Crate Is

The public wire contracts of the CMD+RVL Evidence Machine: `inquiry`,
`grant`, `artifact`, `plan` (+`plan_patch`), `receipt`/`run`, `answer`, and
`capability`, all `.v0`. Serde types, JSON Schemas, fixtures. Nothing else.

It is consumed by `SaltIO/evidence-machine` (private runtime) and
`cmdrvl/spine-rules` (conformance). It consumes nothing from CMD+RVL.

### Source of Truth

- **Design intent:** `cmdrvl-context/docs/02-architecture/evidence-machine.md`
  §4 (contracts) and §2 "Protocol origination and boundary rules".
- **What lands first:** `SaltIO/evidence-machine/docs/MILESTONE_1.md`.
- **Addition rule and canonical JSON:** [`CONTRIBUTING.md`](./CONTRIBUTING.md).

---

## Core Invariants (Do Not Break)

1. **No wire type originates anywhere but here.** If the runtime needs a
   field, it lands here first, with a named consumer.
2. **No speculative contracts.** `control`, identity mesh, strategy,
   tournament, promotion, ontology are deferred. Do not add them without a
   consumer bead.
3. **`operator.v0` is not here.** It belongs to `spine-rules`. There is a test
   for this.
4. **Zero CMD+RVL dependencies.** Not `evidence-machine`, not `spine-rules`,
   not any operator. `deny.toml` + CI enforce the first two; you enforce the
   rest.
5. **Canonical JSON.** Sorted keys, no floats for counts or money, BLAKE3
   hashes as `blake3:<hex>`. Same object, same bytes, same hash, everywhere.
6. **`.v0` is append-only.** Renames and removals are a new version.
7. **Fixtures are the spec.** Every type ships with a valid and an invalid
   fixture and a round-trip test. A schema without fixtures is not done.

---

## Toolchain and Quality Gate

Rust 2024, `#![forbid(unsafe_code)]`, dependencies `serde` + `serde_json`
only (plus a schema library when the first schema lands, in its own bead).

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo deny check bans sources licenses
```

---

## Editing Rules

- **No file deletion** without explicit written user permission.
- **No destructive git commands** without explicit authorization.
- **No scripted mass edits.** Intentional, reviewable changes only.
- **No surprise behavior.** Do not invent contracts or fields not in the
  design intent and not requested by a named consumer.

## RULE 0

If the user gives a direct instruction, follow it even if it conflicts with
defaults in this file.

## RULE 1: No File Deletion

You are never allowed to delete a file without express permission.

---

## Honest Work

- Never present a fixture as proof of runtime behavior.
- Never weaken a schema to make a fixture pass; fix the fixture or the type
  with the consumer's agreement.
- A stub module with a `SCHEMA` constant and doc intent is the honest v0
  state for a contract with no consumer yet. Adding empty structs or
  `todo!()` to look further along is not.

---

## Beads (`br`) Workflow

Issue prefix `ep-`. `br` never runs git; after `br sync --flush-only`, commit
`.beads/` yourself.

```bash
br ready
br show <id>
br update <id> --status=in_progress
br close <id> --reason "..."
br sync --flush-only
```

Every bead that adds a contract or field names its consumer.

---

## Session Completion

1. Quality gate green (fmt, clippy, test, deny).
2. `br sync --flush-only`; commit `.beads/` with your change.
3. Push. Work is not complete until `git push` succeeds.
4. If you changed a contract's intent, note it for re-sync into
   `cmdrvl-context` (canon wins).
