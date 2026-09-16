//! `artifact.v0` — what moves between nodes.
//!
//! The common transport and lineage envelope: artifact id (content hash),
//! kind, media type, byte hash, payload reference, `as_of` / `observed_at`,
//! scope reference, sensitivity, and derivation edges. Domain payloads keep
//! their own versioned schemas; this envelope identifies a payload without
//! requiring the control plane to read it. A digest proves byte equality; it
//! does not grant access. Producing receipt linkage waits for `receipt.v0`.
//!
//! First consumer: `evidence-machine` `em-3h0` (content addressing) and
//! `em-2sk` (materialized warehouse reads).

use super::primitives::{ArtifactRef, Blake3Hash, CasRef};
use serde::{Deserialize, Serialize};

/// Schema identifier carried in the `schema` field of every artifact envelope.
pub const SCHEMA: &str = "artifact.v0";

/// `artifact.v0`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub artifact_id: ArtifactRef,
    pub as_of: String,
    pub byte_hash: Blake3Hash,
    pub derived_from: Vec<ArtifactRef>,
    pub kind: String,
    pub media_type: String,
    pub observed_at: String,
    pub payload_ref: CasRef,
    pub schema: ArtifactSchema,
    pub scope_ref: Blake3Hash,
    pub sensitivity: ArtifactSensitivity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactSchema {
    #[serde(rename = "artifact.v0")]
    V0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactSensitivity {
    Public,
    TestRestricted,
}
