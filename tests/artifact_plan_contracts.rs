use evidence_protocol::v0::{
    artifact::Artifact,
    grant::Grant,
    plan::{Plan, PlanPatch},
};
use serde::de::DeserializeOwned;
use serde_json::Value;

const ARTIFACT_SCHEMA: &[u8] = include_bytes!("../schemas/artifact.v0.schema.json");
const PLAN_SCHEMA: &[u8] = include_bytes!("../schemas/plan.v0.schema.json");
const PLAN_PATCH_SCHEMA: &[u8] = include_bytes!("../schemas/plan_patch.v0.schema.json");
const VALID_ARTIFACT: &[u8] = include_bytes!("../fixtures/artifacts/warehouse_read_scope.v0.json");
const INVALID_ARTIFACT: &[u8] =
    include_bytes!("../fixtures/artifacts/invalid-artifact-hash.v0.json");
const VALID_PLAN: &[u8] = include_bytes!("../fixtures/plans/nport_scope_plan.v0.json");
const INVALID_SQL_PLAN: &[u8] = include_bytes!("../fixtures/plans/invalid-sql-key.v0.json");
const VALID_PLAN_PATCH: &[u8] =
    include_bytes!("../fixtures/plan_patches/add-openfigi-anchor.v0.json");
const INVALID_PLAN_PATCH: &[u8] =
    include_bytes!("../fixtures/plan_patches/invalid-missing-trigger.v0.json");
const VALID_INQUIRY: &[u8] = include_bytes!("../fixtures/inquiries/nport_identity_closure.v0.json");
const VALID_GRANT: &[u8] = include_bytes!("../fixtures/grants/nport_identity_closure.v0.json");

#[test]
fn warehouse_read_artifact_round_trips_through_schema_and_canonical_json() {
    round_trip::<Artifact>(ARTIFACT_SCHEMA, VALID_ARTIFACT);
}

#[test]
fn artifact_schema_rejects_non_blake3_artifact_ids() {
    assert_schema_invalid(ARTIFACT_SCHEMA, INVALID_ARTIFACT);
    assert!(serde_json::from_slice::<Artifact>(INVALID_ARTIFACT).is_err());
}

#[test]
fn nport_scope_plan_round_trips_through_schema_and_canonical_json() {
    round_trip::<Plan>(PLAN_SCHEMA, VALID_PLAN);
}

#[test]
fn plan_schema_rejects_free_sql_keys() {
    assert_schema_invalid(PLAN_SCHEMA, INVALID_SQL_PLAN);
    assert!(serde_json::from_slice::<Plan>(INVALID_SQL_PLAN).is_err());
}

#[test]
fn plan_patch_round_trips_through_schema_and_canonical_json() {
    round_trip::<PlanPatch>(PLAN_PATCH_SCHEMA, VALID_PLAN_PATCH);
}

#[test]
fn plan_patch_schema_rejects_missing_triggering_artifact() {
    assert_schema_invalid(PLAN_PATCH_SCHEMA, INVALID_PLAN_PATCH);
    assert!(serde_json::from_slice::<PlanPatch>(INVALID_PLAN_PATCH).is_err());
}

#[test]
fn plan_fixture_is_bound_to_canonical_inquiry_and_grant_hashes() {
    let plan: Plan = serde_json::from_slice(VALID_PLAN).expect("valid plan");
    let inquiry = parse_json(VALID_INQUIRY);
    let grant = parse_json(VALID_GRANT);
    let expected_inquiry = format!(
        "blake3:{}",
        blake3::hash(&canonical_bytes(&inquiry)).to_hex()
    );
    let expected_grant = format!("blake3:{}", blake3::hash(&canonical_bytes(&grant)).to_hex());

    assert_eq!(
        plan.refs.inquiry_hash.as_str(),
        expected_inquiry,
        "plan fixture inquiry_hash must be {expected_inquiry}"
    );
    assert_eq!(
        plan.refs.grant_hash.as_str(),
        expected_grant,
        "plan fixture grant_hash must be {expected_grant}"
    );

    let grant: Grant = serde_json::from_value(grant).expect("valid grant");
    assert_eq!(
        grant.inquiry_hash, plan.refs.inquiry_hash,
        "plan and grant fixtures must bind to the same inquiry hash"
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
