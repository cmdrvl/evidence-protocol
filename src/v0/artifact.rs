//! `artifact.v0` — what moves between nodes.
//!
//! The common transport and lineage envelope: artifact id (content hash),
//! kind, media type, byte hash, payload reference, `as_of` / `observed_at`,
//! scope reference, sensitivity, producing receipt, and derivation edges.
//! Domain payloads keep their own versioned schemas; this envelope identifies
//! a payload without requiring the control plane to read it. A digest proves
//! byte equality; it does not grant access.
//!
//! First consumer: `evidence-machine` `em-3h0` (content addressing) and
//! `em-2sk` (materialized warehouse reads).

/// Schema identifier carried in the `schema` field of every artifact envelope.
pub const SCHEMA: &str = "artifact.v0";
