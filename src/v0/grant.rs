//! `grant.v0` — what this run is actually allowed to do.
//!
//! Separately issued from the inquiry, content-addressed, bound to one
//! inquiry or a named inquiry class. Declares issuer and delegation chain,
//! validity and revocation reference; approved artifact scopes, connectors
//! and operations; model-context and disclosure ceilings; permitted output and
//! mutation destinations; maximum reservable spend; review requirements at
//! named boundaries; and **opaque capability-handle identifiers only**. No
//! credential material ever appears in a grant.
//!
//! v0: issued by the operator by hand as a checked-in fixture. Effective
//! authority is the meet of inquiry request, grant, and local service policy.
//!
//! First consumer: `evidence-machine` `em-3h0` (fixture) and `em-13q`
//! (authority meet, fail-closed planning).

use super::inquiry::Output;
use super::primitives::{
    Blake3Hash, Budget, CapabilityHandleId, ConnectorAuthority, Disclosure, ModelContext,
};
use serde::{Deserialize, Serialize};

/// Schema identifier carried in the `schema` field of every grant.
pub const SCHEMA: &str = "grant.v0";

/// `grant.v0`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Grant {
    pub approved_connectors: Vec<ConnectorAuthority>,
    pub approved_mutations: Vec<String>,
    pub catalog_keys: Vec<String>,
    pub disclosure_ceiling: Disclosure,
    pub grant_id: String,
    pub handles: Vec<CapabilityHandle>,
    pub inquiry_hash: Blake3Hash,
    pub issued_at: String,
    pub issuer: String,
    pub max_budget: Budget,
    pub model_context: ModelContext,
    pub output: Output,
    pub schema: GrantSchema,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GrantSchema {
    #[serde(rename = "grant.v0")]
    V0,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityHandle {
    pub handle: CapabilityHandleId,
    pub provider: String,
}
