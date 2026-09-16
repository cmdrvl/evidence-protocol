//! `plan.v0` — how this inquiry will be established.
//!
//! A content-addressed DAG of typed operator invocations. Every node pins the
//! operator digest and semantic version, input artifact references and frozen
//! parameters, grant reference and narrowed capability set, child budget,
//! cache policy and idempotency key, admission/retry/timeout/failure policy,
//! domain-disposition mappings, completion predicate, and the consequence of
//! refusal, failure, cancellation, or empty output. A plan never contains free
//! SQL; warehouse reads are declarative bounded read specifications.
//!
//! `plan_patch.v0` is the only lawful way a plan changes after compilation:
//! emitted by a compiler rule on a triggering artifact, or by an
//! operator-authorized resume with a grant delta. Never authored by a model.
//!
//! First consumer: `evidence-machine` `em-13q` (compiler; determinism test).

/// Schema identifier carried in the `schema` field of every plan.
pub const SCHEMA: &str = "plan.v0";

/// Schema identifier carried in the `schema` field of every plan patch.
pub const PATCH_SCHEMA: &str = "plan_patch.v0";
