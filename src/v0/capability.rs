//! `capability.v0` — what one registered evidence channel can prove.
//!
//! A per-channel declaration keyed by catalog channel identity: entity grains;
//! `evidences` (from key family, to key family, predicate); evidence class
//! (`primary_observation`, `corroboration`, `constraint`, `external_anchor`);
//! temporal coverage; upstream `independence` lineage; license; cost; and a
//! sealed `conformance` receipt from resolving known keys through the channel.
//!
//! This is how the compiler derives specific channels from an inquiry's
//! source classes. Three channels fed by one upstream count as one
//! independent observation. A channel without a declaration is not
//! plannable. Declarations are written at registration, tested by conformance
//! runs, and may be proposed for uncovered key families by the tournament
//! proposer under the same gate.
//!
//! First consumer: `evidence-machine` `em-lnx` (reader + conformance runner)
//! and `em-13q` (source selection).

/// Schema identifier carried in the `schema` field of every capability declaration.
pub const SCHEMA: &str = "capability.v0";
