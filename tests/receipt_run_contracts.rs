use evidence_protocol::v0::receipt::{Receipt, ReceiptContext, Run, RunEvent};
use serde::de::DeserializeOwned;
use serde_json::Value;

const RECEIPT_SCHEMA: &[u8] = include_bytes!("../schemas/receipt.v0.schema.json");
const RUN_SCHEMA: &[u8] = include_bytes!("../schemas/run.v0.schema.json");
const NODE_REFUSAL_RECEIPT: &[u8] = include_bytes!("../fixtures/receipts/node_refusal.v0.json");
const STANDALONE_CONFORMANCE_RECEIPT: &[u8] =
    include_bytes!("../fixtures/receipts/standalone_conformance.v0.json");
const INVALID_COLLAPSED_REFUSAL: &[u8] =
    include_bytes!("../fixtures/receipts/invalid-collapsed-refusal.v0.json");
const INVALID_RUN_CONTEXT_MISSING_NODE: &[u8] =
    include_bytes!("../fixtures/receipts/invalid-run-context-missing-node.v0.json");
const VALID_RUN: &[u8] = include_bytes!("../fixtures/runs/nport_fixture_run.v0.json");
const INVALID_BROKEN_RUN_LINK: &[u8] =
    include_bytes!("../fixtures/runs/invalid-broken-link.v0.json");

#[test]
fn node_refusal_receipt_round_trips_through_schema_and_canonical_json() {
    round_trip::<Receipt>(RECEIPT_SCHEMA, NODE_REFUSAL_RECEIPT);
}

#[test]
fn standalone_conformance_receipt_round_trips_through_schema_and_canonical_json() {
    round_trip::<Receipt>(RECEIPT_SCHEMA, STANDALONE_CONFORMANCE_RECEIPT);
}

#[test]
fn run_ledger_round_trips_through_schema_and_canonical_json() {
    round_trip::<Run>(RUN_SCHEMA, VALID_RUN);
}

#[test]
fn receipt_schema_rejects_refusal_collapsed_into_execution_error() {
    assert_schema_invalid(RECEIPT_SCHEMA, INVALID_COLLAPSED_REFUSAL);
    assert!(serde_json::from_slice::<Receipt>(INVALID_COLLAPSED_REFUSAL).is_err());
}

#[test]
fn receipt_schema_rejects_run_context_missing_node_id() {
    assert_schema_invalid(RECEIPT_SCHEMA, INVALID_RUN_CONTEXT_MISSING_NODE);
    assert!(serde_json::from_slice::<Receipt>(INVALID_RUN_CONTEXT_MISSING_NODE).is_err());
}

#[test]
fn run_type_rejects_broken_hash_links() {
    let schema = parse_json(RUN_SCHEMA);
    let instance = parse_json(INVALID_BROKEN_RUN_LINK);
    assert_schema_valid(&schema, &instance);
    assert!(serde_json::from_value::<Run>(instance).is_err());
}

#[test]
fn standalone_receipt_has_no_run_or_node_context() {
    let receipt: Receipt =
        serde_json::from_slice(STANDALONE_CONFORMANCE_RECEIPT).expect("valid receipt");
    assert!(matches!(receipt.context, ReceiptContext::Standalone));
}

#[test]
fn run_receipt_ref_is_bound_to_the_node_receipt_fixture_hash() {
    let run: Run = serde_json::from_slice(VALID_RUN).expect("valid run");
    let receipt = parse_json(NODE_REFUSAL_RECEIPT);
    let expected = format!(
        "receipt:blake3:{}",
        blake3::hash(&canonical_bytes(&receipt)).to_hex()
    );

    let receipt_hash = match &run.events[0] {
        RunEvent::Receipt { receipt_hash, .. } => receipt_hash.as_str(),
        _ => panic!("first run event must be a receipt"),
    };

    assert_eq!(
        receipt_hash, expected,
        "run fixture receipt_hash must be {expected}"
    );
}

fn round_trip<T>(schema: &[u8], fixture: &[u8])
where
    T: DeserializeOwned + serde::Serialize,
{
    let schema = parse_json(schema);
    let fixture_value = parse_json(fixture);
    let fixture = without_terminal_lf(fixture);

    assert_eq!(
        fixture,
        canonical_bytes(&fixture_value).as_slice(),
        "valid fixture must already be canonical JSON"
    );
    assert_no_floats(&fixture_value);
    assert_schema_valid(&schema, &fixture_value);

    let typed: T = serde_json::from_value(fixture_value.clone()).expect("fixture deserializes");
    let typed_value = serde_json::to_value(typed).expect("typed value serializes");

    assert_eq!(
        canonical_bytes(&fixture_value),
        canonical_bytes(&typed_value)
    );
}

fn assert_schema_valid(schema: &Value, instance: &Value) {
    let validator = jsonschema::validator_for(schema).expect("schema compiles");
    let errors = validator.iter_errors(instance).into_errors();
    assert!(errors.is_empty(), "{errors}");
}

fn assert_schema_invalid(schema: &[u8], fixture: &[u8]) {
    let schema = parse_json(schema);
    let fixture = parse_json(fixture);
    let validator = jsonschema::validator_for(&schema).expect("schema compiles");

    assert!(
        !validator.is_valid(&fixture),
        "invalid fixture unexpectedly passed schema validation"
    );
}

fn parse_json(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).expect("valid JSON")
}

fn canonical_bytes(value: &Value) -> Vec<u8> {
    serde_json::to_vec(value).expect("canonical JSON")
}

fn without_terminal_lf(bytes: &[u8]) -> &[u8] {
    bytes.strip_suffix(b"\n").unwrap_or(bytes)
}

fn assert_no_floats(value: &Value) {
    match value {
        Value::Array(items) => items.iter().for_each(assert_no_floats),
        Value::Object(map) => map.values().for_each(assert_no_floats),
        Value::Number(number) => assert!(
            number.is_i64() || number.is_u64(),
            "floats are not canonical protocol JSON: {number}"
        ),
        Value::Bool(_) | Value::Null | Value::String(_) => {}
    }
}
