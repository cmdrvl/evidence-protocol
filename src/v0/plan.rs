//! `plan.v0` — how this inquiry will be established.
//!
//! A content-addressed DAG of typed operator invocations. The v0 slice pins
//! operator digest and semantic version, input artifact references, frozen
//! parameters, grant/inquiry refs, narrowed authority, child budget, cache
//! policy, idempotency key, retry/timeout/failure policy, completion
//! predicate, and the consequence of refusal, failure, cancellation, or empty
//! output. A plan never contains free SQL; warehouse reads are declarative
//! bounded read specifications.
//!
//! `plan_patch.v0` is the only lawful way a plan changes after compilation:
//! emitted by a compiler rule on a triggering artifact, or by an
//! operator-authorized resume with a grant delta. Never authored by a model.
//!
//! First consumer: `evidence-machine` `em-13q` (compiler; determinism test).

use super::primitives::{ArtifactRef, Blake3Hash, Budget, ConnectorAuthority, ModelContext};
use serde::{Deserialize, Serialize};

/// Schema identifier carried in the `schema` field of every plan.
pub const SCHEMA: &str = "plan.v0";

/// Schema identifier carried in the `schema` field of every plan patch.
pub const PATCH_SCHEMA: &str = "plan_patch.v0";

/// `plan.v0`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Plan {
    pub edges: Vec<PlanEdge>,
    pub expected_unknowns: Vec<ExpectedUnknown>,
    pub nodes: Vec<PlanNode>,
    pub refs: PlanRefs,
    pub schema: PlanSchema,
    pub universe: PlanUniverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanSchema {
    #[serde(rename = "plan.v0")]
    V0,
}

/// `plan_patch.v0`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanPatch {
    pub added_nodes: Vec<PlanNode>,
    pub authority_delta: AuthorityDelta,
    pub budget_delta: BudgetDelta,
    pub compiler_rule: String,
    pub retired_node_ids: Vec<String>,
    pub schema: PlanPatchSchema,
    pub triggering_artifact: ArtifactRef,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanPatchSchema {
    #[serde(rename = "plan_patch.v0")]
    V0,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanRefs {
    pub grant_hash: Blake3Hash,
    pub inquiry_hash: Blake3Hash,
    pub policy_hash: Blake3Hash,
    pub registry_snapshot_hash: Blake3Hash,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanUniverse {
    pub grain: String,
    pub partitions: Vec<UniversePartition>,
    pub rule: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UniversePartition {
    pub partition_id: String,
    pub predicate: ReadPredicate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanEdge {
    pub from_node: String,
    pub to_node: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExpectedUnknown {
    pub class: String,
    pub disposition: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanNode {
    pub authority: NodeAuthority,
    pub cache: CachePolicy,
    pub completion: String,
    pub consequence: NodeConsequence,
    pub covers: Vec<String>,
    pub idempotency_key: Blake3Hash,
    pub inputs: Vec<ArtifactRef>,
    pub node_id: String,
    pub obligation: String,
    pub operator: OperatorRef,
    pub params: NodeParams,
    pub retry: RetryPolicy,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeAuthority {
    pub budget: Budget,
    pub connectors: Vec<ConnectorAuthority>,
    pub model_context: ModelContext,
    pub mutations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CachePolicy {
    pub mode: CacheMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CacheMode {
    NoReuse,
    ReuseAllowed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeConsequence {
    pub on_cancel: ConsequenceAction,
    pub on_empty: ConsequenceAction,
    pub on_error: ConsequenceAction,
    pub on_refusal: ConsequenceAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsequenceAction {
    AbortPlan,
    MarkUnknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperatorRef {
    pub digest: Blake3Hash,
    pub id: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum NodeParams {
    Operator { parameters: Vec<Parameter> },
    WarehouseRead { read_spec: WarehouseReadSpec },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Parameter {
    pub name: String,
    pub value: ScalarValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScalarValue {
    Bool(bool),
    Integer(i64),
    String(String),
    Null(()),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WarehouseReadSpec {
    pub catalog_key: String,
    pub columns: Vec<String>,
    pub distinct: bool,
    pub normalization: Vec<NormalizationRule>,
    pub ordering: Vec<ReadOrdering>,
    pub predicates: Vec<ReadPredicate>,
    pub row_bound: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NormalizationRule {
    pub column: String,
    pub placeholder_values: Vec<String>,
    pub to: NormalizedValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NormalizedValue {
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadOrdering {
    pub column: String,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadPredicate {
    pub column: String,
    pub op: PredicateOp,
    pub value: ScalarValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PredicateOp {
    Eq,
    Gte,
    Lte,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetryPolicy {
    pub max_attempts: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityDelta {
    pub connectors: Vec<ConnectorAuthority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_context: Option<ModelContext>,
    pub mutations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BudgetDelta {
    pub bytes: i64,
    pub cost_units: i64,
    pub human_reviews: i64,
    pub model_tokens: i64,
    pub source_calls: i64,
}
