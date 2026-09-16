//! Version 0 of the Evidence Protocol contracts.
//!
//! Each module owns exactly one contract family and exposes its `SCHEMA`
//! identifier. Types are added to a module only when a consumer in
//! `evidence-machine` (or another named consumer) needs them; until then the
//! module carries intent and the identifier, nothing more. See
//! `CONTRIBUTING.md` for the addition rule and the canonical-JSON rule that
//! makes content hashes stable across implementers.

pub mod answer;
pub mod artifact;
pub mod capability;
pub mod grant;
pub mod inquiry;
pub mod plan;
pub mod primitives;
pub mod receipt;
