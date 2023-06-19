pub mod common;
pub mod date_time;
pub mod edge;
pub mod enterprise;
pub mod error;
pub mod gateway;
pub mod login;
pub mod network_address;
pub mod property;
pub mod tinyint;

pub(crate) const REDACTED: &str = "****";

/// This is the first part of the URL path after the host to get to the REST API.
pub const API_BASE: &str = "portal/rest";

/// The standard Integer type used in the VCO API.
pub type Integer = i32;

/// "Number" type as used in the VCO API.
pub type Number = f32;

/// Double-precision floats as used in the VCO API.
pub type Double = f64;

pub type Map<K, V> = std::collections::BTreeMap<K, V>;

pub type Set<V> = std::collections::HashSet<V>;

//
// Serde methods for Logical IDs (aka UUIDv4) as used in the VCO API.
// TODO turn into a proper type
// `logicalId`s cannot be empty but they _could_ be prepended with e.g. "gateway".
//
// pub(crate) mod serde_logical_id {
//     use super::*;
//     pub fn serialize<S>(value: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         serializer.serialize_str(&value.to_string())
//     }
//
//     pub fn deserialize<'d, D>(deserializer: D) -> Result<Uuid, D::Error>
//     where
//         D: Deserializer<'d>,
//     {
//         let value = String::deserialize(deserializer)?;
//         Uuid::parse_str(&value).map_err(de::Error::custom)
//     }
// }
