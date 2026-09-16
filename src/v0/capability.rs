//! `capability.v0` — what one registered evidence channel can prove.
//!
//! A per-channel declaration keyed by catalog channel identity: entity grains;
//! `evidences` (from key family, to key family, predicate); evidence class
//! (`primary_observation`, `corroboration`, `constraint`, `external_anchor`);
//! temporal coverage; upstream `independence` lineage; license; cost; and,
//! for shipped channels, a conformance receipt reference from resolving known
//! keys through the channel.
//!
//! This is how the compiler derives specific channels from an inquiry's
//! source classes. Three channels fed by one upstream count as one
//! independent observation. A channel without a declaration is not
//! plannable. The declaration object is stored verbatim as a property on the
//! catalog channel record, not as a sidecar. Declarations are tested by
//! conformance runs; planned-but-not-yet-readable channels are marked honestly
//! and carry no conformance receipt reference.
//!
//! First consumer: `evidence-machine` `em-lnx` (reader + conformance runner)
//! and `em-13q` (source selection).

use super::primitives::ReceiptRef;
use serde::{Deserialize, Serialize};

/// Schema identifier carried in the `schema` field of every capability declaration.
pub const SCHEMA: &str = "capability.v0";

/// `capability.v0`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityDeclaration {
    pub channel_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_receipt: Option<ReceiptRef>,
    pub cost: CapabilityCost,
    pub entity_grains: Vec<String>,
    pub evidence_class: EvidenceClass,
    pub evidences: Vec<EvidenceClaim>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implementation_status: Option<ImplementationStatus>,
    pub independence: Independence,
    pub license: String,
    pub schema: CapabilitySchema,
    pub temporal_coverage: TemporalCoverage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapabilitySchema {
    #[serde(rename = "capability.v0")]
    V0,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityCost {
    pub basis: String,
    pub cost_units: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceClass {
    Corroboration,
    Constraint,
    ExternalAnchor,
    PrimaryObservation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceClaim {
    pub from: String,
    pub predicate: String,
    pub to: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImplementationStatus {
    Planned,
    Shipped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Independence {
    pub upstream: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemporalCoverage {
    pub basis: String,
    pub from: String,
    pub to: String,
}
