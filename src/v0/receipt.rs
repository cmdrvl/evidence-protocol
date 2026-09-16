//! `receipt.v0` and `run.v0` — what actually happened.
//!
//! Each declared plan node receives exactly one terminal receipt. A receipt
//! preserves four independent axes: execution (`ok` / `err` / `cancelled` /
//! `panicked`), domain disposition (`positive` / `negative` / `refusal` /
//! `not_applicable`), coverage (`unattempted` / `partial` / `complete`), and
//! claim state (`supported` / `contradicted` / `conflicted` / `unresolved` /
//! `forced`). None may be collapsed into another; none may be rendered as
//! "no finding". Receipts carry no credential material.
//!
//! `run.v0` is the ordered, hash-linked ledger of node receipts plus plan
//! patches and control events for one run.
//!
//! First consumer: `evidence-machine` `em-1ez` (process adapter) and
//! `em-ky5` (receipt ledger and durable journal).

use super::primitives::{ArtifactRef, Blake3Hash, ReceiptRef};
use serde::{Deserialize, Serialize};

/// Schema identifier carried in the `schema` field of every node receipt.
pub const SCHEMA: &str = "receipt.v0";

/// Schema identifier carried in the `schema` field of every run ledger.
pub const RUN_SCHEMA: &str = "run.v0";

/// `receipt.v0`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "ReceiptWire")]
pub struct Receipt {
    pub artifacts: Vec<ArtifactRef>,
    pub claim_state: ClaimState,
    pub code: Option<String>,
    pub context: ReceiptContext,
    pub coverage: Coverage,
    pub domain_disposition: DomainDisposition,
    pub execution: Execution,
    pub message: String,
    pub observed_at: String,
    pub schema: ReceiptSchema,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReceiptWire {
    artifacts: Vec<ArtifactRef>,
    claim_state: ClaimState,
    code: Option<String>,
    context: ReceiptContext,
    coverage: Coverage,
    domain_disposition: DomainDisposition,
    execution: Execution,
    message: String,
    observed_at: String,
    schema: ReceiptSchema,
}

impl TryFrom<ReceiptWire> for Receipt {
    type Error = &'static str;

    fn try_from(value: ReceiptWire) -> Result<Self, Self::Error> {
        if value.domain_disposition == DomainDisposition::Refusal
            && value.execution != Execution::Ok
        {
            return Err("domain_disposition=refusal requires execution=ok");
        }

        if matches!(
            value.execution,
            Execution::Err | Execution::Cancelled | Execution::Panicked
        ) && value.domain_disposition != DomainDisposition::NotApplicable
        {
            return Err("non-ok execution requires domain_disposition=not_applicable");
        }

        Ok(Self {
            artifacts: value.artifacts,
            claim_state: value.claim_state,
            code: value.code,
            context: value.context,
            coverage: value.coverage,
            domain_disposition: value.domain_disposition,
            execution: value.execution,
            message: value.message,
            observed_at: value.observed_at,
            schema: value.schema,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReceiptSchema {
    #[serde(rename = "receipt.v0")]
    V0,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ReceiptContext {
    Node { node_id: String, run_id: String },
    Standalone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Execution {
    Cancelled,
    Err,
    Ok,
    Panicked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DomainDisposition {
    Negative,
    NotApplicable,
    Positive,
    Refusal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Coverage {
    Complete,
    Partial,
    Unattempted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimState {
    Conflicted,
    Contradicted,
    Forced,
    Supported,
    Unresolved,
}

/// `run.v0`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "RunWire")]
pub struct Run {
    pub events: Vec<RunEvent>,
    pub plan_hash: Blake3Hash,
    pub run_id: String,
    pub schema: RunSchema,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
struct RunWire {
    events: Vec<RunEvent>,
    plan_hash: Blake3Hash,
    run_id: String,
    schema: RunSchema,
}

impl TryFrom<RunWire> for Run {
    type Error = &'static str;

    fn try_from(value: RunWire) -> Result<Self, Self::Error> {
        let mut previous_hash: Option<&Blake3Hash> = None;

        for (expected_sequence, event) in value.events.iter().enumerate() {
            let sequence = event.sequence();
            if sequence != expected_sequence as u64 {
                return Err("run event sequence must be contiguous from zero");
            }

            match (
                expected_sequence,
                event.previous_entry_hash(),
                previous_hash,
            ) {
                (0, None, _) => {}
                (0, Some(_), _) => {
                    return Err("first run event must have previous_entry_hash=null");
                }
                (_, Some(actual), Some(expected)) if actual == expected => {}
                _ => {
                    return Err("run event previous_entry_hash must link to the prior entry_hash");
                }
            }

            previous_hash = Some(event.entry_hash());
        }

        Ok(Self {
            events: value.events,
            plan_hash: value.plan_hash,
            run_id: value.run_id,
            schema: value.schema,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunSchema {
    #[serde(rename = "run.v0")]
    V0,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum RunEvent {
    Control {
        entry_hash: Blake3Hash,
        event: String,
        observed_at: String,
        previous_entry_hash: Option<Blake3Hash>,
        sequence: u64,
    },
    PlanPatch {
        entry_hash: Blake3Hash,
        plan_patch_hash: Blake3Hash,
        previous_entry_hash: Option<Blake3Hash>,
        sequence: u64,
    },
    Receipt {
        entry_hash: Blake3Hash,
        previous_entry_hash: Option<Blake3Hash>,
        receipt_hash: ReceiptRef,
        sequence: u64,
    },
}

impl RunEvent {
    pub fn entry_hash(&self) -> &Blake3Hash {
        match self {
            Self::Control { entry_hash, .. }
            | Self::PlanPatch { entry_hash, .. }
            | Self::Receipt { entry_hash, .. } => entry_hash,
        }
    }

    pub fn previous_entry_hash(&self) -> Option<&Blake3Hash> {
        match self {
            Self::Control {
                previous_entry_hash,
                ..
            }
            | Self::PlanPatch {
                previous_entry_hash,
                ..
            }
            | Self::Receipt {
                previous_entry_hash,
                ..
            } => previous_entry_hash.as_ref(),
        }
    }

    pub fn sequence(&self) -> u64 {
        match self {
            Self::Control { sequence, .. }
            | Self::PlanPatch { sequence, .. }
            | Self::Receipt { sequence, .. } => *sequence,
        }
    }
}
