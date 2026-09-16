//! `answer.v0` — what is justified.
//!
//! Relative to the inquiry, lenses, evidence closure, and policy; never an
//! unscoped declaration of truth. Carries the answer class (`forced`,
//! `supported_not_forced`, `contradicted`, `conflicted`, `not_comparable`,
//! `indeterminate_evidence`, `indeterminate_operations`, `out_of_scope`) and
//! value reference; support, contradiction, and feasible-alternative
//! references; a coverage certificate over the declared universe; operational
//! and evidentiary unknowns, separately typed; assumptions and policy
//! mappings; budget spent and stopped-work explanation; every pinned hash; and
//! the closure hash that the pack seals.
//!
//! First consumer: `evidence-machine` `em-36x` (sealer / offline verifier)
//! and `em-1xd` (Milestone 1 report).

use super::primitives::{ArtifactRef, Blake3Hash, Budget, ReceiptRef};
use serde::{Deserialize, Serialize};

/// Schema identifier carried in the `schema` field of every answer.
pub const SCHEMA: &str = "answer.v0";

/// `answer.v0`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Answer {
    pub answers: Vec<AnswerItem>,
    pub assumptions: Vec<String>,
    pub budget: BudgetSummary,
    pub closure_refs: AnswerClosureRefs,
    pub coverage: CoverageCertificate,
    pub human_review: HumanReviewSummary,
    pub policy_mappings: Vec<PolicyMapping>,
    pub schema: AnswerSchema,
    pub source_summary: SourceSummary,
    pub stopped_reason: Option<String>,
    pub unknowns: AnswerUnknowns,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnswerSchema {
    #[serde(rename = "answer.v0")]
    V0,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnswerItem {
    pub answer_class: AnswerClass,
    pub contradiction_refs: Vec<ArtifactRef>,
    pub feasible_alternative_refs: Vec<ArtifactRef>,
    pub subject_ref: String,
    pub support_refs: Vec<ArtifactRef>,
    pub value_ref: Option<ArtifactRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnswerClass {
    Contradicted,
    Conflicted,
    Forced,
    IndeterminateEvidence,
    IndeterminateOperations,
    NotComparable,
    OutOfScope,
    SupportedNotForced,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BudgetSummary {
    pub consumed: Budget,
    pub remaining: Budget,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnswerClosureRefs {
    pub closure_root_hash: Blake3Hash,
    pub declaration_hashes: Vec<Blake3Hash>,
    pub grant_hash: Blake3Hash,
    pub inquiry_hash: Blake3Hash,
    pub operator_snapshot_hash: Blake3Hash,
    pub plan_hash: Blake3Hash,
    pub policy_hash: Blake3Hash,
    pub profile_hash: Blake3Hash,
    pub registry_snapshot_hash: Blake3Hash,
    pub run_ledger_hash: Blake3Hash,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoverageCertificate {
    pub breakdowns: Vec<CoverageBreakdown>,
    pub counts: CoverageCounts,
    pub universe_grain: String,
    pub universe_ref: Blake3Hash,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoverageBreakdown {
    pub by: String,
    pub counts: CoverageCounts,
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "CoverageCountsWire")]
pub struct CoverageCounts {
    pub attempted: u64,
    pub complete_empty: u64,
    pub complete_positive: u64,
    pub incomplete_authority: u64,
    pub incomplete_budget: u64,
    pub incomplete_evidence: u64,
    pub incomplete_execution: u64,
    pub incomplete_source: u64,
    pub not_investigated: u64,
    pub out_of_scope: u64,
    pub total: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
struct CoverageCountsWire {
    attempted: u64,
    complete_empty: u64,
    complete_positive: u64,
    incomplete_authority: u64,
    incomplete_budget: u64,
    incomplete_evidence: u64,
    incomplete_execution: u64,
    incomplete_source: u64,
    not_investigated: u64,
    out_of_scope: u64,
    total: u64,
}

impl TryFrom<CoverageCountsWire> for CoverageCounts {
    type Error = &'static str;

    fn try_from(value: CoverageCountsWire) -> Result<Self, Self::Error> {
        let attempted = checked_sum(&[
            value.complete_empty,
            value.complete_positive,
            value.incomplete_authority,
            value.incomplete_budget,
            value.incomplete_evidence,
            value.incomplete_execution,
            value.incomplete_source,
        ])?;

        if attempted != value.attempted {
            return Err("coverage attempted must equal attempted terminal buckets");
        }

        let total = checked_sum(&[attempted, value.not_investigated, value.out_of_scope])?;

        if total != value.total {
            return Err("coverage total must equal all terminal buckets");
        }

        Ok(Self {
            attempted: value.attempted,
            complete_empty: value.complete_empty,
            complete_positive: value.complete_positive,
            incomplete_authority: value.incomplete_authority,
            incomplete_budget: value.incomplete_budget,
            incomplete_evidence: value.incomplete_evidence,
            incomplete_execution: value.incomplete_execution,
            incomplete_source: value.incomplete_source,
            not_investigated: value.not_investigated,
            out_of_scope: value.out_of_scope,
            total: value.total,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HumanReviewSummary {
    pub adjudicated: u64,
    pub promoted: u64,
    pub queued: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyMapping {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceSummary {
    pub channels: Vec<SourceSummaryEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceSummaryEntry {
    pub artifacts: Vec<ArtifactRef>,
    pub channel_key: String,
    pub receipts: Vec<ReceiptRef>,
    pub source_class: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnswerUnknowns {
    pub evidentiary: Vec<TypedUnknown>,
    pub operational: Vec<TypedUnknown>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TypedUnknown {
    pub can_change_answer: bool,
    pub class: UnknownClass,
    pub closing_action: String,
    pub description: String,
    pub scope_ref: Option<ArtifactRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnknownClass {
    AuthorityMissing,
    BudgetExhausted,
    CandidateReachUnproven,
    EvidenceConflicted,
    EvidenceMissing,
    ExecutionFailed,
    HumanJudgmentRequired,
    KeyFamilyUncovered,
    NotInvestigated,
    OperatorRefused,
    PrivacyBoundary,
    PromotionBlocked,
    SourceUnavailable,
    StrategyUnresolved,
}

fn checked_sum(values: &[u64]) -> Result<u64, &'static str> {
    values.iter().try_fold(0_u64, |total, value| {
        total
            .checked_add(*value)
            .ok_or("coverage bucket sum overflow")
    })
}
