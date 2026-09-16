use evidence_protocol::v0::{grant::Grant, inquiry::Inquiry};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

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
fn inquiry_subject_can_change_to_another_structurally_valid_table_ref() {
    let schema = parse_json(INQUIRY_SCHEMA);
    let mut inquiry = parse_json(VALID_INQUIRY);
    inquiry["question"]["subject"] =
        json!("snowflake://edgar_db.dbt_wrangling_edgar/alternate_instrument_observations");

    assert_schema_valid_and_typed_roundtrip::<Inquiry>(&schema, &inquiry);
}

#[test]
fn inquiry_workload_selected_names_can_vary_without_schema_changes() {
    let schema = parse_json(INQUIRY_SCHEMA);
    let mut inquiry = parse_json(VALID_INQUIRY);
    inquiry["identity_target"]["anchor_families"] = json!(["cik", "figi", "lei"]);
    inquiry["identity_target"]["entity_grains"] = json!(["instrument", "issuer"]);
    inquiry["identity_target"]["key_families"] = json!(["isin", "sedol", "ticker"]);
    inquiry["identity_target"]["optimization"] = json!("bounded_admissible_closure");
    inquiry["identity_target"]["temporal"] = json!("current_snapshot");
    inquiry["output"]["kind"] = json!("portfolio_identity_closure");
    inquiry["question"]["predicate"] = json!("resolves_portfolio_identity");
    inquiry["requested_authority"]["mutations"] = json!([
        "registry_promotion:issuer_identity",
        "registry_promotion:security_identity"
    ]);
    inquiry["universe"]["grain"] = json!("security_observation");
    inquiry["universe"]["rule"] = json!("distinct_security_tuple_per_period");

    assert_schema_valid_and_typed_roundtrip::<Inquiry>(&schema, &inquiry);
}

#[test]
fn grant_workload_selected_names_can_vary_without_schema_changes() {
    let schema = parse_json(GRANT_SCHEMA);
    let mut grant = parse_json(VALID_GRANT);
    grant["approved_mutations"] = json!([
        "registry_promotion:issuer_identity",
        "registry_promotion:security_identity"
    ]);
    grant["output"]["kind"] = json!("portfolio_identity_closure");

    assert_schema_valid_and_typed_roundtrip::<Grant>(&schema, &grant);
}

#[test]
fn inquiry_schema_rejects_empty_or_malformed_workload_selected_names() {
    let schema = parse_json(INQUIRY_SCHEMA);
    let cases = [
        (json!(""), &["question", "subject"][..]),
        (json!("not-a-table-ref"), &["question", "subject"][..]),
        (json!(""), &["question", "predicate"][..]),
        (json!("bad predicate"), &["question", "predicate"][..]),
        (json!(""), &["universe", "grain"][..]),
        (json!("bad grain"), &["universe", "grain"][..]),
        (json!(""), &["universe", "rule"][..]),
        (json!("bad rule"), &["universe", "rule"][..]),
        (json!(""), &["output", "kind"][..]),
        (json!("bad kind"), &["output", "kind"][..]),
        (json!([""]), &["identity_target", "anchor_families"][..]),
        (
            json!(["bad anchor"]),
            &["identity_target", "anchor_families"][..],
        ),
        (json!([""]), &["identity_target", "entity_grains"][..]),
        (
            json!(["bad grain"]),
            &["identity_target", "entity_grains"][..],
        ),
        (json!([""]), &["identity_target", "key_families"][..]),
        (json!(["bad key"]), &["identity_target", "key_families"][..]),
        (json!(""), &["identity_target", "optimization"][..]),
        (
            json!("bad optimization"),
            &["identity_target", "optimization"][..],
        ),
        (json!(""), &["identity_target", "temporal"][..]),
        (json!("bad temporal"), &["identity_target", "temporal"][..]),
        (json!([""]), &["requested_authority", "mutations"][..]),
        (
            json!(["bad mutation"]),
            &["requested_authority", "mutations"][..],
        ),
    ];

    for (replacement, path) in cases {
        let mut inquiry = parse_json(VALID_INQUIRY);
        set_path(&mut inquiry, path, replacement);
        assert_schema_invalid_value(&schema, &inquiry);
    }
}

#[test]
fn grant_schema_rejects_empty_or_malformed_workload_selected_names() {
    let schema = parse_json(GRANT_SCHEMA);
    let cases = [
        (json!(""), &["output", "kind"][..]),
        (json!("bad kind"), &["output", "kind"][..]),
        (json!([""]), &["approved_mutations"][..]),
        (json!(["bad mutation"]), &["approved_mutations"][..]),
    ];

    for (replacement, path) in cases {
        let mut grant = parse_json(VALID_GRANT);
        set_path(&mut grant, path, replacement);
        assert_schema_invalid_value(&schema, &grant);
    }
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

fn assert_schema_valid_and_typed_roundtrip<T>(schema: &Value, instance: &Value)
where
    T: DeserializeOwned + serde::Serialize,
{
    assert_no_floats(instance);
    assert_schema_valid(schema, instance);

    let typed: T = serde_json::from_value(instance.clone()).expect("instance deserializes");
    let typed_value = serde_json::to_value(typed).expect("typed value serializes");

    assert_eq!(canonical_bytes(instance), canonical_bytes(&typed_value));
}

fn assert_schema_invalid(schema: &[u8], fixture: &[u8]) {
    let schema = parse_json(schema);
    let fixture = parse_json(fixture);

    assert_schema_invalid_value(&schema, &fixture);
}

fn assert_schema_invalid_value(schema: &Value, fixture: &Value) {
    let validator = jsonschema::validator_for(schema).expect("schema compiles");

    assert!(
        !validator.is_valid(fixture),
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

fn set_path(value: &mut Value, path: &[&str], replacement: Value) {
    let Some((leaf, parents)) = path.split_last() else {
        return;
    };

    let mut cursor = value;
    for key in parents {
        cursor = cursor
            .as_object_mut()
            .and_then(|object| object.get_mut(*key))
            .expect("test path exists");
    }

    cursor
        .as_object_mut()
        .expect("test path parent is an object")
        .insert((*leaf).to_string(), replacement);
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
