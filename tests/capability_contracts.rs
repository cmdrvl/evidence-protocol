use evidence_protocol::v0::capability::{CapabilityDeclaration, ImplementationStatus};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::BTreeSet;

const CAPABILITY_SCHEMA: &[u8] = include_bytes!("../schemas/capability.v0.schema.json");
const NPORT_HOLDINGS: &[u8] =
    include_bytes!("../fixtures/capabilities/nport_holdings_authoritative.v0.json");
const NPORT_CONFIRMATION: &[u8] =
    include_bytes!("../fixtures/capabilities/wrgl_edgar_nport_confirmation__coupon.v0.json");
const OPENFIGI: &[u8] = include_bytes!("../fixtures/capabilities/openfigi.v0.json");
const GLEIF_ISIN_LEI: &[u8] = include_bytes!("../fixtures/capabilities/gleif_isin_to_lei.v0.json");
const GLEIF_ENTITIES: &[u8] =
    include_bytes!("../fixtures/capabilities/gleif_entities_relationships.v0.json");
const INVALID_MISSING_INDEPENDENCE: &[u8] =
    include_bytes!("../fixtures/capabilities/invalid-missing-independence.v0.json");

const VALID_CAPABILITIES: &[&[u8]] = &[
    NPORT_HOLDINGS,
    NPORT_CONFIRMATION,
    OPENFIGI,
    GLEIF_ISIN_LEI,
    GLEIF_ENTITIES,
];

#[test]
fn milestone_1_capabilities_round_trip_through_schema_and_canonical_json() {
    for fixture in VALID_CAPABILITIES {
        round_trip::<CapabilityDeclaration>(CAPABILITY_SCHEMA, fixture);
    }
}

#[test]
fn capability_schema_rejects_missing_independence() {
    assert_schema_invalid(CAPABILITY_SCHEMA, INVALID_MISSING_INDEPENDENCE);
    assert!(serde_json::from_slice::<CapabilityDeclaration>(INVALID_MISSING_INDEPENDENCE).is_err());
}

#[test]
fn milestone_1_declares_exactly_the_five_catalog_channels() {
    let actual: BTreeSet<_> = VALID_CAPABILITIES
        .iter()
        .map(|fixture| {
            let declaration: CapabilityDeclaration =
                serde_json::from_slice(fixture).expect("valid capability declaration");
            declaration.channel_key
        })
        .collect();
    let expected = BTreeSet::from([
        "gleif_entities".to_string(),
        "gleif_isin_lei".to_string(),
        "openfigi".to_string(),
        "snowflake://edgar_db.dbt_wrangling_edgar/nport_holdings_authoritative".to_string(),
        "snowflake://edgar_db.dbt_wrangling_edgar/wrgl_edgar_nport_confirmation__coupon"
            .to_string(),
    ]);

    assert_eq!(actual, expected);
}

#[test]
fn sec_confirmation_and_holdings_share_one_upstream_family() {
    let holdings: CapabilityDeclaration =
        serde_json::from_slice(NPORT_HOLDINGS).expect("valid holdings declaration");
    let confirmation: CapabilityDeclaration =
        serde_json::from_slice(NPORT_CONFIRMATION).expect("valid confirmation declaration");

    assert_eq!(holdings.independence.upstream, ["sec_edgar"]);
    assert_eq!(confirmation.independence.upstream, ["sec_edgar"]);
}

#[test]
fn planned_declarations_do_not_claim_conformance_receipts() {
    let planned = [OPENFIGI, GLEIF_ISIN_LEI];
    for fixture in planned {
        let declaration: CapabilityDeclaration =
            serde_json::from_slice(fixture).expect("valid planned declaration");
        assert_eq!(
            declaration.implementation_status,
            Some(ImplementationStatus::Planned)
        );
        assert!(declaration.conformance_receipt.is_none());
    }
}

#[test]
fn shipped_declarations_carry_conformance_receipt_refs() {
    let shipped = [NPORT_HOLDINGS, NPORT_CONFIRMATION, GLEIF_ENTITIES];
    for fixture in shipped {
        let declaration: CapabilityDeclaration =
            serde_json::from_slice(fixture).expect("valid shipped declaration");
        assert_eq!(
            declaration.implementation_status,
            Some(ImplementationStatus::Shipped)
        );
        assert!(declaration.conformance_receipt.is_some());
    }
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
