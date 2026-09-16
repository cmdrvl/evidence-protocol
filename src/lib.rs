#![forbid(unsafe_code)]
//! # Evidence Protocol
//!
//! Public wire contracts for the CMD+RVL Evidence Machine. This crate defines
//! the `*.v0` contracts once — as serde types, JSON Schemas (`schemas/`), and
//! fixtures (`fixtures/`) — so that the private runtime (`evidence-machine`)
//! can implement, validate, persist, and internally extend them without ever
//! redefining them, and so that `spine-rules` can check conformance against
//! them.
//!
//! ## Boundary
//!
//! - No wire type originates anywhere but here.
//! - This crate depends on nothing CMD+RVL-private and nothing CMD+RVL-public
//!   either: not `evidence-machine`, not `spine-rules`, not any operator.
//!   `deny.toml` enforces the first two; the third is a review rule.
//! - A contract is added only with a named consumer. Fields are fixed only
//!   when that consumer lands. See `CONTRIBUTING.md`.
//!
//! ## Initial surface (v0)
//!
//! `inquiry`, `grant`, `artifact`, `plan` (+ `plan_patch`), `receipt` / `run`,
//! `answer`, `capability`. Everything else (`control`, identity mesh,
//! strategy/tournament, promotion, ontology) is deferred until a consumer
//! exists. `operator.v0` lives in `spine-rules`, not here.

pub mod v0;

/// Every schema identifier this crate owns. The Evidence Machine's boundary
/// test asserts that each `schema` string it emits appears in this list.
pub const CONTRACTS: &[&str] = &[
    v0::inquiry::SCHEMA,
    v0::grant::SCHEMA,
    v0::artifact::SCHEMA,
    v0::plan::SCHEMA,
    v0::plan::PATCH_SCHEMA,
    v0::receipt::SCHEMA,
    v0::receipt::RUN_SCHEMA,
    v0::answer::SCHEMA,
    v0::capability::SCHEMA,
];

#[cfg(test)]
mod tests {
    use super::CONTRACTS;
    use std::collections::BTreeSet;

    #[test]
    fn contract_ids_are_versioned_and_unique() {
        let mut seen = BTreeSet::new();
        for id in CONTRACTS {
            assert!(id.ends_with(".v0"), "{id} must carry a .v0 suffix");
            assert!(seen.insert(*id), "{id} is listed twice");
        }
        assert_eq!(seen.len(), CONTRACTS.len());
    }

    #[test]
    fn operator_manifest_is_not_defined_here() {
        // operator.v0 belongs to spine-rules. A naive "put everything in the
        // protocol crate" implementation would fail this.
        assert!(!CONTRACTS.contains(&"operator.v0"));
    }
}
