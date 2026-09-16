//! `answer.v0` — what is justified.
//!
//! Relative to the inquiry, lenses, evidence closure, and policy; never an
//! unscoped declaration of truth. Carries the answer class (`forced`,
//! `supported_not_forced`, `contradicted`, `conflicted`, `not_comparable`,
//! `indeterminate_evidence`, `indeterminate_operations`, `out_of_scope`) and
//! value reference; support, contradiction, and feasible-alternative
//! references; a coverage certificate over the declared universe; operational
//! and evidentiary unknowns, separately typed; assumptions and policy
//! mappings; budget spent and stopped-work explanation; every pinned hash; and
//! the closure hash that the pack seals.
//!
//! First consumer: `evidence-machine` `em-36x` (sealer / offline verifier)
//! and `em-1xd` (Milestone 1 report).

/// Schema identifier carried in the `schema` field of every answer.
pub const SCHEMA: &str = "answer.v0";
