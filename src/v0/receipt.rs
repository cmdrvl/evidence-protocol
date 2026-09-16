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

/// Schema identifier carried in the `schema` field of every node receipt.
pub const SCHEMA: &str = "receipt.v0";

/// Schema identifier carried in the `schema` field of every run ledger.
pub const RUN_SCHEMA: &str = "run.v0";
