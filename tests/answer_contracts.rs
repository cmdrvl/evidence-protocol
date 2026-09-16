use evidence_protocol::v0::answer::{Answer, AnswerClass, UnknownClass};
use serde::de::DeserializeOwned;
use serde_json::Value;

const ANSWER_SCHEMA: &[u8] = include_bytes!("../schemas/answer.v0.schema.json");
const FORCED_ESTABLISHED: &[u8] =
    include_bytes!("../fixtures/answers/nport_forced_established.v0.json");
const UNRESOLVED_INDETERMINATE: &[u8] =
    include_bytes!("../fixtures/answers/nport_unresolved_indeterminate.v0.json");
const INVALID_COVERAGE_TOTAL: &[u8] =
    include_bytes!("../fixtures/answers/invalid-coverage-total.v0.json");

const VALID_ANSWERS: &[&[u8]] = &[FORCED_ESTABLISHED, UNRESOLVED_INDETERMINATE];

#[test]
fn answer_fixtures_round_trip_through_schema_and_canonical_json() {
    for fixture in VALID_ANSWERS {
        round_trip::<Answer>(ANSWER_SCHEMA, fixture);
    }
}

#[test]
fn forced_established_answer_carries_closure_refs_and_stable_hash() {
    let fixture_value = parse_json(FORCED_ESTABLISHED);
    let fixture_hash = blake3::hash(&canonical_bytes(&fixture_value));
    let answer: Answer = serde_json::from_value(fixture_value.clone()).expect("valid answer");
    let typed_value = serde_json::to_value(answer).expect("typed value serializes");
    let typed_hash = blake3::hash(&canonical_bytes(&typed_value));

    assert_eq!(fixture_hash, typed_hash);
    assert!(typed_value["closure_refs"]["closure_root_hash"].is_string());
    assert_eq!(typed_value["answers"][0]["answer_class"], "forced");
}

#[test]
fn unresolved_indeterminate_answer_carries_typed_unknowns() {
    let answer: Answer = serde_json::from_slice(UNRESOLVED_INDETERMINATE).expect("valid answer");

    assert!(matches!(
        answer.answers.first().map(|item| item.answer_class),
        Some(AnswerClass::IndeterminateEvidence)
    ));
    assert!(
        answer
            .unknowns
            .evidentiary
            .iter()
            .any(|unknown| unknown.class == UnknownClass::EvidenceMissing
                && unknown.can_change_answer)
    );
    assert!(
        answer
            .unknowns
            .operational
            .iter()
            .any(|unknown| unknown.class == UnknownClass::SourceUnavailable
                && unknown.can_change_answer)
    );
}

#[test]
fn answer_contract_rejects_coverage_total_mismatch() {
    let schema = parse_json(ANSWER_SCHEMA);
    let instance = parse_json(INVALID_COVERAGE_TOTAL);

    assert_schema_valid(&schema, &instance);
    assert!(serde_json::from_value::<Answer>(instance).is_err());
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
