//! `inquiry.v0` — what is being asked.
//!
//! Immutable, content-addressed. Declares subject and predicate; the bounded
//! universe or a deterministic rule for constructing it; target entity grains,
//! key families, and anchor families; `as_of` and validity; source classes
//! and policy (never specific channels — the compiler derives those);
//! pinned lenses (profile, registry, policy); completion predicate and
//! acceptable unknowns; root budget; requested output and disclosure; and
//! requested authority, never authority assumed.
//!
//! First consumer: `evidence-machine` `em-3h0` (validation, hashing, the
//! N-PORT identity-closure fixture) and `em-13q` (compiler).

/// Schema identifier carried in the `schema` field of every inquiry.
pub const SCHEMA: &str = "inquiry.v0";
