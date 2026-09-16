//! Shared public primitives used by the v0 contract structs.
//!
//! These are not standalone wire contracts. They exist only to keep fields
//! that are shared by concrete contracts from drifting apart.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// Resource budget dimensions used by `inquiry.v0` and `grant.v0`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Budget {
    pub bytes: u64,
    pub cost_units: u64,
    pub human_reviews: u64,
    pub model_tokens: u64,
    pub source_calls: u64,
}

/// A connector plus the operation names requested or approved on it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectorAuthority {
    pub operations: Vec<String>,
    pub provider: String,
}

/// Disclosure ceiling used in the Milestone 1 public slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Disclosure {
    Public,
}

/// Model-context ceiling used in the Milestone 1 public slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelContext {
    MetadataOnly,
}

/// BLAKE3 content hash string in protocol form: `blake3:<64 lowercase hex>`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Blake3Hash(String);

impl Blake3Hash {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Blake3Hash {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if is_blake3_hash(&value) {
            Ok(Self(value))
        } else {
            Err("expected blake3:<64 lowercase hex>".to_string())
        }
    }
}

impl Serialize for Blake3Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for Blake3Hash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::try_from(String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for Blake3Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Protocol reference to an artifact envelope: `artifact:blake3:<64 lowercase hex>`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactRef(String);

impl ArtifactRef {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ArtifactRef {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if is_prefixed_blake3_ref(&value, "artifact:") {
            Ok(Self(value))
        } else {
            Err("expected artifact:blake3:<64 lowercase hex>".to_string())
        }
    }
}

impl Serialize for ArtifactRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ArtifactRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::try_from(String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for ArtifactRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Protocol reference to content-addressed payload bytes: `cas:blake3:<64 lowercase hex>`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CasRef(String);

impl CasRef {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for CasRef {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if is_prefixed_blake3_ref(&value, "cas:") {
            Ok(Self(value))
        } else {
            Err("expected cas:blake3:<64 lowercase hex>".to_string())
        }
    }
}

impl Serialize for CasRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for CasRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::try_from(String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for CasRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Opaque handle id. The broker resolves it to credentials outside the wire object.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CapabilityHandleId(String);

impl CapabilityHandleId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for CapabilityHandleId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if is_capability_handle_id(&value) {
            Ok(Self(value))
        } else {
            Err(
                "expected opaque handle:<provider>:<purpose> id with no credential markers"
                    .to_string(),
            )
        }
    }
}

impl Serialize for CapabilityHandleId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for CapabilityHandleId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::try_from(String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for CapabilityHandleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

fn is_blake3_hash(value: &str) -> bool {
    let Some(hex) = value.strip_prefix("blake3:") else {
        return false;
    };

    hex.len() == 64
        && hex
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn is_prefixed_blake3_ref(value: &str, prefix: &str) -> bool {
    value.strip_prefix(prefix).is_some_and(is_blake3_hash)
}

fn is_capability_handle_id(value: &str) -> bool {
    if has_credential_marker(value) {
        return false;
    }

    let mut parts = value.split(':');
    if parts.next() != Some("handle") {
        return false;
    }

    let rest: Vec<_> = parts.collect();
    rest.len() >= 2
        && rest
            .iter()
            .all(|part| !part.is_empty() && part.bytes().all(is_handle_segment_byte))
}

fn is_handle_segment_byte(byte: u8) -> bool {
    byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
}

fn has_credential_marker(value: &str) -> bool {
    let value = value.to_ascii_lowercase();
    [
        "://",
        "=",
        "access-key",
        "access_key",
        "api-key",
        "api_key",
        "credential",
        "password",
        "private-key",
        "private_key",
        "secret",
        "token",
        "connection-string",
        "connection_string",
    ]
    .iter()
    .any(|marker| value.contains(marker))
}
