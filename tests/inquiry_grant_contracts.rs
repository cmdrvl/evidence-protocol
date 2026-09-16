use evidence_protocol::v0::{grant::Grant, inquiry::Inquiry};
use serde::de::DeserializeOwned;
use serde_json::Value;

const INQUIRY_SCHEMA: &[u8] = include_bytes!("../schemas/inquiry.v0.schema.json");
const GRANT_SCHEMA: &[u8] = include_bytes!("../schemas/grant.v0.schema.json");
const VALID_INQUIRY: &[u8] = include_bytes!("../fixtures/inquiries/nport_identity_closure.v0.json");
const INVALID_INQUIRY: &[u8] =
    include_bytes!("../fixtures/inquiries/invalid-nport_identity_closure.v0.json");
const VALID_GRANT: &[u8] = include_bytes!("../fixtures/grants/nport_identity_closure.v0.json");
const INVALID_GRANT: &[u8] = include_bytes!("../fixtures/grants/invalid-credential-handle.v0.json");

#[test]
fn nport_inquiry_round_trips_through_schema_and_canonical_json() {
    round_trip::<Inquiry>(INQUIRY_SCHEMA, VALID_INQUIRY);
}

#[test]
fn hand_issued_grant_round_trips_through_schema_and_canonical_json() {
    round_trip::<Grant>(GRANT_SCHEMA, VALID_GRANT);
}

#[test]
fn inquiry_schema_rejects_named_channels() {
    assert_schema_invalid(INQUIRY_SCHEMA, INVALID_INQUIRY);
    assert!(serde_json::from_slice::<Inquiry>(INVALID_INQUIRY).is_err());
}

#[test]
fn grant_schema_rejects_credential_like_handle_values() {
    assert_schema_invalid(GRANT_SCHEMA, INVALID_GRANT);
    assert!(serde_json::from_slice::<Grant>(INVALID_GRANT).is_err());
}

#[test]
fn grant_fixture_is_bound_to_the_canonical_inquiry_hash() {
    let grant: Grant = serde_json::from_slice(VALID_GRANT).expect("valid grant");
    let inquiry = parse_json(VALID_INQUIRY);
    let expected = format!(
        "blake3:{}",
        blake3::hash(&canonical_bytes(&inquiry)).to_hex()
    );

    assert_eq!(
        grant.inquiry_hash.as_str(),
        expected,
        "grant fixture inquiry_hash must be {expected}"
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
