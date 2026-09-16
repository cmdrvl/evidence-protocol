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

/// Schema identifier carried in the `schema` field of every grant.
pub const SCHEMA: &str = "grant.v0";
