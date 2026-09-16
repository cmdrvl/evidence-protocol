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

use super::primitives::{Budget, ConnectorAuthority, Disclosure, ModelContext};
use serde::{Deserialize, Serialize};

/// Schema identifier carried in the `schema` field of every inquiry.
pub const SCHEMA: &str = "inquiry.v0";

/// `inquiry.v0`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Inquiry {
    pub budget: Budget,
    pub completion: Completion,
    pub identity_target: IdentityTarget,
    pub lenses: Lenses,
    pub output: Output,
    pub question: Question,
    pub requested_authority: RequestedAuthority,
    pub schema: InquirySchema,
    pub source_policy: SourcePolicy,
    pub universe: Universe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InquirySchema {
    #[serde(rename = "inquiry.v0")]
    V0,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Completion {
    pub acceptable_unknowns: Vec<String>,
    pub allow_operational_unknowns: bool,
    pub require_every_subject_accounted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DateWindow {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityTarget {
    pub allow_new_key_families: bool,
    pub anchor_families: Vec<String>,
    pub entity_grains: Vec<String>,
    pub key_families: Vec<String>,
    pub optimization: String,
    pub temporal: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lenses {
    pub policy: String,
    pub profile: String,
    pub registry: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Output {
    pub disclosure: Disclosure,
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Question {
    pub as_of: DateWindow,
    pub predicate: String,
    pub subject: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestedAuthority {
    pub connectors: Vec<ConnectorAuthority>,
    pub model_context: ModelContext,
    pub mutations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourcePolicy {
    pub classes_allowed: Vec<String>,
    pub disclosure_ceiling: Disclosure,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Universe {
    pub grain: String,
    pub placeholder_identifiers: Vec<String>,
    pub rule: String,
}
