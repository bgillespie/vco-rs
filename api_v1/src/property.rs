use serde::{Deserialize, Serialize};

use crate::date_time::DateTime;
use crate::tinyint::TinyInt;
use crate::Integer;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum PropertyDataType {
    String,
    Number,
    Boolean,
    Json,
    Date,
    Datetime,
}

/// This is the basic SystemProperty that can be sent to update or insert.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SystemProperty {
    pub id: Integer,
    pub name: String,
    pub value: String,
    pub default_value: Option<String>,
    pub is_read_only: TinyInt,
    pub is_password: TinyInt,
    pub data_type: PropertyDataType,
    pub description: Option<String>,
}

/// This is the system property as returned by VCO, complete with create and modify datetimes.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetSystemPropertiesResultItem {
    #[serde(flatten)]
    pub property: SystemProperty,
    pub created: DateTime,
    pub modified: DateTime,
}
